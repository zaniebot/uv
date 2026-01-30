use std::str::FromStr;

use tracing::debug;

use uv_normalize::{ExtraName, PackageName};
use uv_pep440::Version;
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

    /// Convert an [`Unspecified`](Target::Unspecified) target to [`Latest`](Target::Latest).
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
    Unspecified(&'a str, PackageName, Box<[ExtraName]>),
    /// A raw requirement string that needs PEP 508 / URL / path parsing,
    /// e.g., `ruff>=0.6.0`, `git+https://...`, or `./local-path`.
    Requirement(&'a str),
    /// e.g., `ruff[extra]@0.6.0`
    Version(&'a str, PackageName, Box<[ExtraName]>, Version),
    /// e.g., `ruff[extra]@latest`
    Latest(&'a str, PackageName, Box<[ExtraName]>),
}

impl<'a> Target<'a> {
    /// Parse a target into a command name and a requirement.
    pub(crate) fn parse(target: &'a str) -> Self {
        // If there's no `@`, try to parse as a bare package name (with optional extras).
        let Some((name, version)) = target.split_once('@') else {
            return Self::parse_bare(target);
        };

        // e.g. `ruff@`, warn and treat the whole thing as the command
        if version.is_empty() {
            debug!("Ignoring empty version request in command");
            return Self::parse_bare(target);
        }

        // Split into name and extras (e.g., `flask[dotenv]`).
        let (executable, extras) = match name.split_once('[') {
            Some((executable, extras)) => {
                let Some(extras) = extras.strip_suffix(']') else {
                    // e.g., ignore `flask[dotenv`.
                    return Self::parse_bare(target);
                };
                (executable, extras)
            }
            None => (name, ""),
        };

        // e.g., ignore `git+https://github.com/astral-sh/ruff.git@main`
        let Ok(name) = PackageName::from_str(executable) else {
            debug!("Ignoring non-package name `{name}` in command");
            return Self::parse_bare(target);
        };

        // e.g., ignore `ruff[1.0.0]` or any other invalid extra.
        let Ok(extras) = extras
            .split(',')
            .map(str::trim)
            .filter(|extra| !extra.is_empty())
            .map(ExtraName::from_str)
            .collect::<Result<Box<_>, _>>()
        else {
            debug!("Ignoring invalid extras `{extras}` in command");
            return Self::parse_bare(target);
        };

        match version {
            // e.g., `ruff@latest`
            "latest" => Self::Latest(executable, name, extras),
            // e.g., `ruff@0.6.0`
            version => {
                if let Ok(version) = Version::from_str(version) {
                    Self::Version(executable, name, extras, version)
                } else {
                    // e.g. `ruff@invalid`, warn and treat the whole thing as the command
                    debug!("Ignoring invalid version request `{version}` in command");
                    Self::parse_bare(target)
                }
            }
        }
    }

    /// Try to parse `target` as a bare package name with optional extras (e.g., `ruff` or
    /// `flask[dotenv]`). Falls back to [`Requirement`](Target::Requirement) for anything
    /// that doesn't parse as a valid package name.
    fn parse_bare(target: &'a str) -> Self {
        // Split into name and extras (e.g., `flask[dotenv]`).
        let (name, extras_str) = match target.split_once('[') {
            Some((name, rest)) => {
                let Some(extras) = rest.strip_suffix(']') else {
                    return Self::Requirement(target);
                };
                (name, extras)
            }
            None => (target, ""),
        };

        let Ok(package_name) = PackageName::from_str(name) else {
            return Self::Requirement(target);
        };

        let Ok(extras) = extras_str
            .split(',')
            .map(str::trim)
            .filter(|extra| !extra.is_empty())
            .map(ExtraName::from_str)
            .collect::<Result<Box<_>, _>>()
        else {
            return Self::Requirement(target);
        };

        Self::Unspecified(name, package_name, extras)
    }

    /// Convert an [`Unspecified`](Target::Unspecified) target to [`Latest`](Target::Latest).
    pub(crate) fn into_latest(self) -> Self {
        match self {
            Self::Unspecified(name, package_name, extras) => {
                Self::Latest(name, package_name, extras)
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_target() {
        // Bare package name.
        let target = Target::parse("flask");
        let expected = Target::Unspecified(
            "flask",
            PackageName::from_str("flask").unwrap(),
            Box::new([]),
        );
        assert_eq!(target, expected);

        // Bare package name with extras.
        let target = Target::parse("flask[dotenv]");
        let expected = Target::Unspecified(
            "flask",
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
            "flask",
            PackageName::from_str("flask").unwrap(),
            Box::new([]),
            Version::new([3, 0, 0]),
        );
        assert_eq!(target, expected);

        // Latest via `@latest`.
        let target = Target::parse("flask@latest");
        let expected = Target::Latest(
            "flask",
            PackageName::from_str("flask").unwrap(),
            Box::new([]),
        );
        assert_eq!(target, expected);

        // Version with extras.
        let target = Target::parse("flask[dotenv]@3.0.0");
        let expected = Target::Version(
            "flask",
            PackageName::from_str("flask").unwrap(),
            Box::new([ExtraName::from_str("dotenv").unwrap()]),
            Version::new([3, 0, 0]),
        );
        assert_eq!(target, expected);

        // Latest with extras.
        let target = Target::parse("flask[dotenv]@latest");
        let expected = Target::Latest(
            "flask",
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
            Target::Latest("ruff", PackageName::from_str("ruff").unwrap(), Box::new([]))
        );

        // Package with extras is converted.
        let target = Target::parse("flask[dotenv]").into_latest();
        assert!(matches!(target, Target::Latest("flask", _, _)));

        // Already-latest is unchanged.
        let target = Target::parse("ruff@latest").into_latest();
        assert!(matches!(target, Target::Latest("ruff", _, _)));

        // Version-pinned is unchanged.
        let target = Target::parse("ruff@0.6.0").into_latest();
        assert!(matches!(target, Target::Version(..)));

        // Raw requirement strings are unchanged.
        let target = Target::parse("ruff>=0.5").into_latest();
        assert!(matches!(target, Target::Requirement(_)));
    }
}
