use std::fmt::Write;

use anyhow::Result;
use futures::StreamExt;
use owo_colors::OwoColorize;
use rustc_hash::FxHashMap;
use serde::Serialize;

use uv_cache::{Cache, Refresh};
use uv_cache_info::Timestamp;
use uv_cli::ToolListFormat;
use uv_client::{BaseClientBuilder, RegistryClientBuilder};
use uv_configuration::Concurrency;
use uv_distribution_filename::DistFilename;
use uv_distribution_types::IndexCapabilities;
use uv_fs::Simplified;
use uv_normalize::PackageName;
use uv_pep440::Version;
use uv_python::LenientImplementationName;
use uv_resolver::{ExcludeNewer, PrereleaseMode};
use uv_tool::InstalledTools;
use uv_warnings::warn_user;

use crate::commands::ExitStatus;
use crate::commands::pip::latest::LatestClient;
use crate::commands::reporters::LatestVersionReporter;
use crate::printer::Printer;

/// An entry representing a single installed tool.
#[derive(Debug, Serialize)]
struct ToolEntry {
    /// The tool package name.
    name: PackageName,
    /// The installed version.
    version: Version,
    /// The version specifiers used to install the tool.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    version_specifiers: Vec<String>,
    /// The extras installed with the tool.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    extras: Vec<String>,
    /// Additional requirements installed with the tool.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    with: Vec<String>,
    /// The Python interpreter for the tool environment.
    python: String,
    /// The latest available version, if outdated.
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_version: Option<Version>,
    /// The path to the tool environment directory.
    path: String,
    /// The entry points provided by the tool.
    entrypoints: Vec<EntrypointEntry>,
}

/// An entry representing a single tool entrypoint.
#[derive(Debug, Serialize)]
struct EntrypointEntry {
    /// The name of the entrypoint executable.
    name: String,
    /// The install path of the entrypoint executable.
    path: String,
}

impl ToolEntry {
    /// Format this entry as human-readable text.
    #[expect(clippy::fn_params_excessive_bools)]
    fn format_text(
        &self,
        show_paths: bool,
        show_version_specifiers: bool,
        show_with: bool,
        show_extras: bool,
        show_python: bool,
        outdated: bool,
    ) -> String {
        let mut output = String::new();

        let version_specifier = if show_version_specifiers && !self.version_specifiers.is_empty() {
            format!(" [required: {}]", self.version_specifiers.join(", "))
        } else {
            String::new()
        };

        let extra_requirements = if show_extras && !self.extras.is_empty() {
            format!(" [extras: {}]", self.extras.join(", "))
        } else {
            String::new()
        };

        let with_requirements = if show_with && !self.with.is_empty() {
            format!(" [with: {}]", self.with.join(", "))
        } else {
            String::new()
        };

        let python_version = if show_python {
            format!(" [{}]", self.python)
        } else {
            String::new()
        };

        let latest_version = if outdated {
            self.latest_version
                .as_ref()
                .map(|version| format!(" [latest: {version}]"))
                .unwrap_or_default()
        } else {
            String::new()
        };

        if show_paths {
            writeln!(
                output,
                "{} ({})",
                format!(
                    "{name} v{version}{version_specifier}{extra_requirements}{with_requirements}{python_version}{latest_version}",
                    name = self.name,
                    version = self.version,
                )
                .bold(),
                self.path.cyan(),
            )
            .expect("write to String");
        } else {
            writeln!(
                output,
                "{}",
                format!(
                    "{name} v{version}{version_specifier}{extra_requirements}{with_requirements}{python_version}{latest_version}",
                    name = self.name,
                    version = self.version,
                )
                .bold()
            )
            .expect("write to String");
        }

        for entrypoint in &self.entrypoints {
            if show_paths {
                writeln!(output, "- {}", entrypoint.to_string().cyan()).expect("write to String");
            } else {
                writeln!(output, "- {}", entrypoint.name).expect("write to String");
            }
        }

        output
    }
}

impl std::fmt::Display for EntrypointEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.path)
    }
}

/// List installed tools.
#[expect(clippy::fn_params_excessive_bools)]
pub(crate) async fn list(
    show_paths: bool,
    show_version_specifiers: bool,
    show_with: bool,
    show_extras: bool,
    show_python: bool,
    outdated: bool,
    output_format: ToolListFormat,
    client_builder: BaseClientBuilder<'_>,
    concurrency: Concurrency,
    cache: &Cache,
    printer: Printer,
) -> Result<ExitStatus> {
    let installed_tools = InstalledTools::from_settings()?;
    let _lock = match installed_tools.lock().await {
        Ok(lock) => lock,
        Err(err)
            if err
                .as_io_error()
                .is_some_and(|err| err.kind() == std::io::ErrorKind::NotFound) =>
        {
            match output_format {
                ToolListFormat::Text => {
                    writeln!(printer.stderr(), "No tools installed")?;
                }
                ToolListFormat::Json => {
                    writeln!(printer.stdout(), "[]")?;
                }
            }
            return Ok(ExitStatus::Success);
        }
        Err(err) => return Err(err.into()),
    };

    let mut tools = installed_tools.tools()?.into_iter().collect::<Vec<_>>();
    tools.sort_by_key(|(name, _)| name.clone());

    if tools.is_empty() {
        match output_format {
            ToolListFormat::Text => {
                writeln!(printer.stderr(), "No tools installed")?;
            }
            ToolListFormat::Json => {
                writeln!(printer.stdout(), "[]")?;
            }
        }
        return Ok(ExitStatus::Success);
    }

    // Collect valid tools (skip invalid ones) before checking for outdated versions.
    let mut valid_tools = Vec::new();
    for (name, tool) in tools {
        // Skip invalid tools
        let Ok(tool) = tool else {
            warn_user!(
                "Ignoring malformed tool `{name}` (run `{}` to remove)",
                format!("uv tool uninstall {name}").green()
            );
            continue;
        };

        // Get the tool environment
        let tool_env = match installed_tools.get_environment(&name, cache) {
            Ok(Some(env)) => env,
            Ok(None) => {
                warn_user!(
                    "Tool `{name}` environment not found (run `{}` to reinstall)",
                    format!("uv tool install {name} --reinstall").green()
                );
                continue;
            }
            Err(e) => {
                warn_user!(
                    "{e} (run `{}` to reinstall)",
                    format!("uv tool install {name} --reinstall").green()
                );
                continue;
            }
        };

        // Get the tool version
        let version = match tool_env.version() {
            Ok(version) => version,
            Err(e) => {
                if let uv_tool::Error::EnvironmentError(e) = e {
                    warn_user!(
                        "{e} (run `{}` to reinstall)",
                        format!("uv tool install {name} --reinstall").green()
                    );
                } else {
                    writeln!(printer.stderr(), "{e}")?;
                }
                continue;
            }
        };

        valid_tools.push((name, tool, tool_env, version));
    }

    // Determine the latest version for each tool when `--outdated` is requested.
    let latest: FxHashMap<PackageName, Option<DistFilename>> = if outdated
        && !valid_tools.is_empty()
    {
        let capabilities = IndexCapabilities::default();

        // Initialize the registry client.
        let client = RegistryClientBuilder::new(
            client_builder,
            cache.clone().with_refresh(Refresh::All(Timestamp::now())),
        )
        .build();
        let download_concurrency = concurrency.downloads_semaphore.clone();

        // Initialize the client to fetch the latest version of each package.
        let latest_client = LatestClient {
            client: &client,
            capabilities: &capabilities,
            prerelease: PrereleaseMode::default(),
            exclude_newer: &ExcludeNewer::default(),
            tags: None,
            requires_python: None,
        };

        let reporter = LatestVersionReporter::from(printer).with_length(valid_tools.len() as u64);

        // Fetch the latest version for each tool.
        let mut fetches = futures::stream::iter(&valid_tools)
            .map(async |(name, _tool, _tool_env, _version)| {
                let latest = latest_client
                    .find_latest(name, None, &download_concurrency)
                    .await?;
                Ok::<(&PackageName, Option<DistFilename>), uv_client::Error>((name, latest))
            })
            .buffer_unordered(concurrency.downloads);

        let mut map = FxHashMap::default();
        while let Some((name, version)) = fetches.next().await.transpose()? {
            if let Some(version) = version.as_ref() {
                reporter.on_fetch_version(name, version.version());
            } else {
                reporter.on_fetch_progress();
            }
            map.insert(name.clone(), version);
        }
        reporter.on_fetch_complete();
        map
    } else {
        FxHashMap::default()
    };

    // Build tool entries from the collected data.
    let mut entries = Vec::new();
    for (name, tool, tool_env, version) in valid_tools {
        // If `--outdated` is set, skip tools that are up-to-date.
        if outdated {
            let is_outdated = latest
                .get(&name)
                .and_then(Option::as_ref)
                .is_some_and(|filename| filename.version() > &version);
            if !is_outdated {
                continue;
            }
        }

        let version_specifiers = tool
            .requirements()
            .iter()
            .filter(|req| req.name == name)
            .map(|req| req.source.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let extras = tool
            .requirements()
            .iter()
            .filter(|req| req.name == name)
            .flat_map(|req| req.extras.iter())
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let with = tool
            .requirements()
            .iter()
            .filter(|req| req.name != name)
            .map(|req| format!("{}{}", req.name, req.source))
            .collect::<Vec<_>>();

        let interpreter = tool_env.environment().interpreter();
        let implementation = LenientImplementationName::from(interpreter.implementation_name());
        let python = format!(
            "{} {}",
            implementation.pretty(),
            interpreter.python_full_version()
        );

        let latest_version = latest
            .get(&name)
            .and_then(Option::as_ref)
            .map(|filename| filename.version().clone());

        let path = installed_tools
            .tool_dir(&name)
            .simplified_display()
            .to_string();

        let entrypoints = tool
            .entrypoints()
            .iter()
            .map(|ep| EntrypointEntry {
                name: ep.name.clone(),
                path: ep.install_path.simplified_display().to_string(),
            })
            .collect();

        entries.push(ToolEntry {
            name,
            version,
            version_specifiers,
            extras,
            with,
            python,
            latest_version,
            path,
            entrypoints,
        });
    }

    match output_format {
        ToolListFormat::Json => {
            let output = serde_json::to_string(&entries)?;
            writeln!(printer.stdout(), "{output}")?;
        }
        ToolListFormat::Text => {
            for entry in &entries {
                write!(
                    printer.stdout(),
                    "{}",
                    entry.format_text(
                        show_paths,
                        show_version_specifiers,
                        show_with,
                        show_extras,
                        show_python,
                        outdated,
                    )
                )?;
            }
        }
    }

    Ok(ExitStatus::Success)
}
