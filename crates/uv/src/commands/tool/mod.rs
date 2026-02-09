use std::str::FromStr;

use tracing::debug;

use uv_normalize::PackageName;
use uv_pep440::Version;
use uv_pep508::VerbatimUrl;
use uv_python::PythonRequest;

mod common;
pub(crate) mod dir;
pub(crate) mod install;
pub(crate) mod list;
pub(crate) mod run;
pub(crate) mod uninstall;
pub(crate) mod update_shell;
pub(crate) mod upgrade;

/// A request to run or install a tool (e.g., `uvx ruff@latest`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ToolRequest<'a> {
    // Running the interpreter directly e.g. `uvx python` or `uvx pypy@3.8`
    Python {
        /// The executable name (e.g., `bash`), if the interpreter was given via --from.
        executable: Option<&'a str>,
        // The interpreter to install or run (e.g., `python@3.8` or `pypy311`.
        request: PythonRequest,
    },
    // Running a Python package
    Package {
        /// The executable name (e.g., `ruff`), if the target was given via --from.
        executable: Option<&'a str>,
        /// The target to install or run (e.g., `ruff@latest` or `ruff==0.6.0`).
        target: Target<'a>,
    },
}

impl<'a> ToolRequest<'a> {
    /// Parse a tool request into an executable name and a target.
    pub(crate) fn parse(command: &'a str, from: Option<&'a str>) -> anyhow::Result<Self> {
        // If --from is used, the command could be an arbitrary binary in the PATH (e.g. `bash`),
        // and we don't try to parse it.
        let (component_to_parse, executable) = match from {
            Some(from) => (from, Some(command)),
            None => (command, None),
        };

        // First try parsing the command as a Python interpreter, like `python`, `python39`, or
        // `pypy@39`. `pythonw` is also allowed on Windows. This overlaps with how `--python` flag
        // values are parsed, but see `PythonRequest::parse` vs `PythonRequest::try_from_tool_name`
        // for the differences.
        if let Some(python_request) = PythonRequest::try_from_tool_name(component_to_parse)? {
            Ok(Self::Python {
                request: python_request,
                executable,
            })
        } else {
            // Otherwise the command is a Python package, like `ruff` or `ruff@0.6.0`.
            Ok(Self::Package {
                target: Target::parse(component_to_parse),
                executable,
            })
        }
    }

    /// Returns `true` if the target is `latest`.
    pub(crate) fn is_latest(&self) -> bool {
        matches!(
            self,
            Self::Package {
                target: Target::Latest(..),
                ..
            }
        )
    }

    /// Convert a [`Name`](Target::Name) target to [`Latest`](Target::Latest).
    pub(crate) fn into_latest(self) -> Self {
        match self {
            Self::Package { executable, target } => Self::Package {
                executable,
                target: target.into_latest(),
            },
            other => other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Target<'a> {
    /// A bare package name, e.g., `ruff` or `flask[dotenv]`.
    Name(PackageName, Box<[uv_normalize::ExtraName]>),
    /// A raw requirement string that needs PEP 508 / URL / path parsing,
    /// e.g., `ruff>=0.6.0`, `git+https://...`, or `./local-path`.
    Requirement(&'a str),
    /// e.g., `ruff[extra]@0.6.0`
    Version(PackageName, Box<[uv_normalize::ExtraName]>, Version),
    /// e.g., `ruff[extra]@latest`
    Latest(PackageName, Box<[uv_normalize::ExtraName]>),
}

impl<'a> Target<'a> {
    /// Parse a target into a package name and version/requirement.
    pub(crate) fn parse(target: &'a str) -> Self {
        // If there's no `@`, check if it's a bare package name or a more complex requirement.
        let Some((name, version)) = target.split_once('@') else {
            return Self::parse_bare(target);
        };

        // e.g. `ruff@`, warn and treat as bare
        if version.is_empty() {
            debug!("Ignoring empty version request in command");
            return Self::parse_bare(target);
        }

        // Parse the name portion (before `@`) as a package name with optional extras.
        let Some((package_name, extras)) = Self::parse_name_and_extras(name) else {
            // e.g., `git+https://github.com/astral-sh/ruff.git@main`
            debug!("Ignoring non-package name `{name}` in command");
            return Self::parse_bare(target);
        };

        match version {
            // e.g., `ruff@latest`
            "latest" => Self::Latest(package_name, extras),
            // e.g., `ruff@0.6.0`
            version => {
                if let Ok(version) = Version::from_str(version) {
                    Self::Version(package_name, extras, version)
                } else {
                    // e.g. `ruff@invalid`, warn and treat as bare
                    debug!("Ignoring invalid version request `{version}` in command");
                    Self::parse_bare(target)
                }
            }
        }
    }

    /// Try to parse `target` as a bare package name with optional extras.
    /// Falls back to [`Requirement`](Target::Requirement) for anything else.
    fn parse_bare(target: &'a str) -> Self {
        // Try PEP 508 parsing - if it succeeds with no version specifier, it's a bare name.
        if let Ok(req) = uv_pep508::Requirement::<VerbatimUrl>::from_str(target) {
            if req.version_or_url.is_none() && req.marker.is_true() {
                return Self::Name(req.name, req.extras);
            }
        }
        Self::Requirement(target)
    }

    /// Parse a name with optional extras, e.g., `flask` or `flask[dotenv]`.
    fn parse_name_and_extras(input: &str) -> Option<(PackageName, Box<[uv_normalize::ExtraName]>)> {
        // Try PEP 508 parsing
        let req = uv_pep508::Requirement::<VerbatimUrl>::from_str(input).ok()?;
        // Only accept if it's just a name (with optional extras), no version/url/markers
        if req.version_or_url.is_some() || !req.marker.is_true() {
            return None;
        }
        Some((req.name, req.extras))
    }

    /// Convert a [`Name`](Target::Name) target to [`Latest`](Target::Latest).
    pub(crate) fn into_latest(self) -> Self {
        match self {
            Self::Name(package_name, extras) => Self::Latest(package_name, extras),
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uv_normalize::ExtraName;

    #[test]
    fn parse_target() {
        // Bare package name.
        let target = Target::parse("flask");
        let expected = Target::Name(PackageName::from_str("flask").unwrap(), Box::new([]));
        assert_eq!(target, expected);

        // Bare package name with extras.
        let target = Target::parse("flask[dotenv]");
        let expected = Target::Name(
            PackageName::from_str("flask").unwrap(),
            Box::new([ExtraName::from_str("dotenv").unwrap()]),
        );
        assert_eq!(target, expected);

        // PEP 508 specifier falls back to Requirement.
        let target = Target::parse("ruff>=0.5");
        assert_eq!(target, Target::Requirement("ruff>=0.5"));

        // URL falls back to Requirement.
        let target = Target::parse("./local-path");
        assert_eq!(target, Target::Requirement("./local-path"));

        // Version pinned via `@`.
        let target = Target::parse("flask@3.0.0");
        let expected = Target::Version(
            PackageName::from_str("flask").unwrap(),
            Box::new([]),
            Version::new([3, 0, 0]),
        );
        assert_eq!(target, expected);

        // Latest via `@latest`.
        let target = Target::parse("flask@latest");
        let expected = Target::Latest(PackageName::from_str("flask").unwrap(), Box::new([]));
        assert_eq!(target, expected);

        // Version with extras.
        let target = Target::parse("flask[dotenv]@3.0.0");
        let expected = Target::Version(
            PackageName::from_str("flask").unwrap(),
            Box::new([ExtraName::from_str("dotenv").unwrap()]),
            Version::new([3, 0, 0]),
        );
        assert_eq!(target, expected);

        // Latest with extras.
        let target = Target::parse("flask[dotenv]@latest");
        let expected = Target::Latest(
            PackageName::from_str("flask").unwrap(),
            Box::new([ExtraName::from_str("dotenv").unwrap()]),
        );
        assert_eq!(target, expected);

        // Missing a closing `]` falls back to Requirement.
        let target = Target::parse("flask[dotenv");
        assert_eq!(target, Target::Requirement("flask[dotenv"));

        // Too many `]` falls back to Requirement.
        let target = Target::parse("flask[dotenv]]");
        assert_eq!(target, Target::Requirement("flask[dotenv]]"));
    }

    #[test]
    fn target_into_latest() {
        // Simple package name is converted.
        let target = Target::parse("ruff").into_latest();
        assert_eq!(
            target,
            Target::Latest(PackageName::from_str("ruff").unwrap(), Box::new([]))
        );

        // Package with extras is converted.
        let target = Target::parse("flask[dotenv]").into_latest();
        assert!(matches!(target, Target::Latest(..)));

        // Already-latest is unchanged.
        let target = Target::parse("ruff@latest").into_latest();
        assert!(matches!(target, Target::Latest(..)));

        // Version-pinned is unchanged.
        let target = Target::parse("ruff@0.6.0").into_latest();
        assert!(matches!(target, Target::Version(..)));

        // Raw requirement strings are unchanged.
        let target = Target::parse("ruff>=0.5").into_latest();
        assert!(matches!(target, Target::Requirement(_)));
    }
}
