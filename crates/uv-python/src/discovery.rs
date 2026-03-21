use itertools::{Either, Itertools};
use owo_colors::AnsiColors;
use regex::Regex;
use reqwest_retry::policies::ExponentialBackoff;
use rustc_hash::{FxBuildHasher, FxHashSet};
use same_file::is_same_file;
use std::borrow::Cow;
use std::env::consts::EXE_SUFFIX;
use std::fmt::{self, Debug, Formatter};
use std::{env, io, iter};
use std::{path::Path, path::PathBuf, str::FromStr};
use thiserror::Error;
use tracing::{debug, instrument, trace};
use uv_cache::Cache;
use uv_client::BaseClient;
use uv_distribution_types::RequiresPython;
use uv_fs::Simplified;
use uv_fs::which::is_executable;
use uv_pep440::{
    LowerBound, Prerelease, UpperBound, Version, VersionSpecifier, VersionSpecifiers,
    release_specifiers_to_ranges,
};
use uv_preview::Preview;
use uv_static::EnvVars;
use uv_warnings::anstream;
use uv_warnings::warn_user_once;
use which::{which, which_all};

use crate::downloads::{ManagedPythonDownloadList, PlatformRequest, PythonDownloadRequest};
use crate::implementation::ImplementationName;
use crate::installation::{PythonInstallation, PythonInstallationKey};
use crate::interpreter::Error as InterpreterError;
use crate::interpreter::{StatusCodeError, UnexpectedResponseError};
use crate::managed::{ManagedPythonInstallations, PythonMinorVersionLink};
#[cfg(windows)]
use crate::microsoft_store::find_microsoft_store_pythons;
use crate::python_version::python_build_versions_from_env;
use crate::virtualenv::Error as VirtualEnvError;
use crate::virtualenv::{
    CondaEnvironmentKind, conda_environment_from_env, virtualenv_from_env,
    virtualenv_from_working_dir, virtualenv_python_executable,
};
#[cfg(windows)]
use crate::windows_registry::{WindowsPython, registry_pythons};
use crate::{BrokenLink, Interpreter, PythonVersion};

mod request;
mod search;

pub(crate) use request::DiscoveryPreferences;
pub use request::{
    EnvironmentPreference, PythonDownloads, PythonPreference, PythonRequest, PythonVariant,
    VersionRequest,
};
pub use search::find_python_installations;
pub(crate) use search::{find_best_python_installation, find_python_installation};

type FindPythonResult = Result<PythonInstallation, PythonNotFound>;

/// The result of failed Python installation discovery.
///
/// See [`FindPythonResult`].
#[derive(Clone, Debug, Error)]
pub struct PythonNotFound {
    pub request: PythonRequest,
    pub python_preference: PythonPreference,
    pub environment_preference: EnvironmentPreference,
}

/// A location for discovery of a Python installation or interpreter.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub enum PythonSource {
    /// The path was provided directly
    ProvidedPath,
    /// An environment was active e.g. via `VIRTUAL_ENV`
    ActiveEnvironment,
    /// A conda environment was active e.g. via `CONDA_PREFIX`
    CondaPrefix,
    /// A base conda environment was active e.g. via `CONDA_PREFIX`
    BaseCondaPrefix,
    /// An environment was discovered e.g. via `.venv`
    DiscoveredEnvironment,
    /// An executable was found in the search path i.e. `PATH`
    SearchPath,
    /// The first executable found in the search path i.e. `PATH`
    SearchPathFirst,
    /// An executable was found in the Windows registry via PEP 514
    Registry,
    /// An executable was found in the known Microsoft Store locations
    MicrosoftStore,
    /// The Python installation was found in the uv managed Python directory
    Managed,
    /// The Python installation was found via the invoking interpreter i.e. via `python -m uv ...`
    ParentInterpreter,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    /// An error was encountering when retrieving interpreter information.
    #[error("Failed to inspect Python interpreter from {} at `{}` ", _2, _1.user_display())]
    Query(
        #[source] Box<crate::interpreter::Error>,
        PathBuf,
        PythonSource,
    ),

    /// An error was encountered while trying to find a managed Python installation matching the
    /// current platform.
    #[error("Failed to discover managed Python installations")]
    ManagedPython(#[from] crate::managed::Error),

    /// An error was encountered when inspecting a virtual environment.
    #[error(transparent)]
    VirtualEnv(#[from] crate::virtualenv::Error),

    #[cfg(windows)]
    #[error("Failed to query installed Python versions from the Windows registry")]
    RegistryError(#[from] windows::core::Error),

    /// An invalid version request was given
    #[error("Invalid version request: {0}")]
    InvalidVersionRequest(String),

    /// The @latest version request was given
    #[error("Requesting the 'latest' Python version is not yet supported")]
    LatestVersionRequest,

    // TODO(zanieb): Is this error case necessary still? We should probably drop it.
    #[error("Interpreter discovery for `{0}` requires `{1}` but only `{2}` is allowed")]
    SourceNotAllowed(PythonRequest, PythonSource, PythonPreference),

    #[error(transparent)]
    BuildVersion(#[from] crate::python_version::BuildVersionError),
}

/// Lazily iterate over Python executables in mutable virtual environments.
///
/// The following sources are supported:
///
/// - Active virtual environment (via `VIRTUAL_ENV`)
/// - Discovered virtual environment (e.g. `.venv` in a parent directory)
///
/// Notably, "system" environments are excluded. See [`python_executables_from_installed`].
#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use assert_fs::{TempDir, prelude::*};
    use target_lexicon::{Aarch64Architecture, Architecture};
    use test_log::test;
    use uv_distribution_types::RequiresPython;
    use uv_pep440::{Prerelease, PrereleaseKind, Version, VersionSpecifiers};

    use crate::{
        discovery::{PythonRequest, VersionRequest},
        downloads::{ArchRequest, PythonDownloadRequest},
        implementation::ImplementationName,
    };
    use uv_platform::{Arch, Libc, Os};

    use super::{
        DiscoveryPreferences, EnvironmentPreference, Error, PythonPreference, PythonVariant,
    };

    #[test]
    fn interpreter_request_from_str() {
        assert_eq!(PythonRequest::parse("any"), PythonRequest::Any);
        assert_eq!(PythonRequest::parse("default"), PythonRequest::Default);
        assert_eq!(
            PythonRequest::parse("3.12"),
            PythonRequest::Version(VersionRequest::from_str("3.12").unwrap())
        );
        assert_eq!(
            PythonRequest::parse(">=3.12"),
            PythonRequest::Version(VersionRequest::from_str(">=3.12").unwrap())
        );
        assert_eq!(
            PythonRequest::parse(">=3.12,<3.13"),
            PythonRequest::Version(VersionRequest::from_str(">=3.12,<3.13").unwrap())
        );
        assert_eq!(
            PythonRequest::parse(">=3.12,<3.13"),
            PythonRequest::Version(VersionRequest::from_str(">=3.12,<3.13").unwrap())
        );

        assert_eq!(
            PythonRequest::parse("3.13.0a1"),
            PythonRequest::Version(VersionRequest::from_str("3.13.0a1").unwrap())
        );
        assert_eq!(
            PythonRequest::parse("3.13.0b5"),
            PythonRequest::Version(VersionRequest::from_str("3.13.0b5").unwrap())
        );
        assert_eq!(
            PythonRequest::parse("3.13.0rc1"),
            PythonRequest::Version(VersionRequest::from_str("3.13.0rc1").unwrap())
        );
        assert_eq!(
            PythonRequest::parse("3.13.1rc1"),
            PythonRequest::ExecutableName("3.13.1rc1".to_string()),
            "Pre-release version requests require a patch version of zero"
        );
        assert_eq!(
            PythonRequest::parse("3rc1"),
            PythonRequest::ExecutableName("3rc1".to_string()),
            "Pre-release version requests require a minor version"
        );

        assert_eq!(
            PythonRequest::parse("cpython"),
            PythonRequest::Implementation(ImplementationName::CPython)
        );

        assert_eq!(
            PythonRequest::parse("cpython3.12.2"),
            PythonRequest::ImplementationVersion(
                ImplementationName::CPython,
                VersionRequest::from_str("3.12.2").unwrap(),
            )
        );

        assert_eq!(
            PythonRequest::parse("cpython-3.13.2"),
            PythonRequest::Key(PythonDownloadRequest {
                version: Some(VersionRequest::MajorMinorPatch(
                    3,
                    13,
                    2,
                    PythonVariant::Default
                )),
                implementation: Some(ImplementationName::CPython),
                arch: None,
                os: None,
                libc: None,
                build: None,
                prereleases: None
            })
        );
        assert_eq!(
            PythonRequest::parse("cpython-3.13.2-macos-aarch64-none"),
            PythonRequest::Key(PythonDownloadRequest {
                version: Some(VersionRequest::MajorMinorPatch(
                    3,
                    13,
                    2,
                    PythonVariant::Default
                )),
                implementation: Some(ImplementationName::CPython),
                arch: Some(ArchRequest::Explicit(Arch::new(
                    Architecture::Aarch64(Aarch64Architecture::Aarch64),
                    None
                ))),
                os: Some(Os::new(target_lexicon::OperatingSystem::Darwin(None))),
                libc: Some(Libc::None),
                build: None,
                prereleases: None
            })
        );
        assert_eq!(
            PythonRequest::parse("any-3.13.2"),
            PythonRequest::Key(PythonDownloadRequest {
                version: Some(VersionRequest::MajorMinorPatch(
                    3,
                    13,
                    2,
                    PythonVariant::Default
                )),
                implementation: None,
                arch: None,
                os: None,
                libc: None,
                build: None,
                prereleases: None
            })
        );
        assert_eq!(
            PythonRequest::parse("any-3.13.2-any-aarch64"),
            PythonRequest::Key(PythonDownloadRequest {
                version: Some(VersionRequest::MajorMinorPatch(
                    3,
                    13,
                    2,
                    PythonVariant::Default
                )),
                implementation: None,
                arch: Some(ArchRequest::Explicit(Arch::new(
                    Architecture::Aarch64(Aarch64Architecture::Aarch64),
                    None
                ))),
                os: None,
                libc: None,
                build: None,
                prereleases: None
            })
        );

        assert_eq!(
            PythonRequest::parse("pypy"),
            PythonRequest::Implementation(ImplementationName::PyPy)
        );
        assert_eq!(
            PythonRequest::parse("pp"),
            PythonRequest::Implementation(ImplementationName::PyPy)
        );
        assert_eq!(
            PythonRequest::parse("graalpy"),
            PythonRequest::Implementation(ImplementationName::GraalPy)
        );
        assert_eq!(
            PythonRequest::parse("gp"),
            PythonRequest::Implementation(ImplementationName::GraalPy)
        );
        assert_eq!(
            PythonRequest::parse("cp"),
            PythonRequest::Implementation(ImplementationName::CPython)
        );
        assert_eq!(
            PythonRequest::parse("pypy3.10"),
            PythonRequest::ImplementationVersion(
                ImplementationName::PyPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
        );
        assert_eq!(
            PythonRequest::parse("pp310"),
            PythonRequest::ImplementationVersion(
                ImplementationName::PyPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
        );
        assert_eq!(
            PythonRequest::parse("graalpy3.10"),
            PythonRequest::ImplementationVersion(
                ImplementationName::GraalPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
        );
        assert_eq!(
            PythonRequest::parse("gp310"),
            PythonRequest::ImplementationVersion(
                ImplementationName::GraalPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
        );
        assert_eq!(
            PythonRequest::parse("cp38"),
            PythonRequest::ImplementationVersion(
                ImplementationName::CPython,
                VersionRequest::from_str("3.8").unwrap(),
            )
        );
        assert_eq!(
            PythonRequest::parse("pypy@3.10"),
            PythonRequest::ImplementationVersion(
                ImplementationName::PyPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
        );
        assert_eq!(
            PythonRequest::parse("pypy310"),
            PythonRequest::ImplementationVersion(
                ImplementationName::PyPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
        );
        assert_eq!(
            PythonRequest::parse("graalpy@3.10"),
            PythonRequest::ImplementationVersion(
                ImplementationName::GraalPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
        );
        assert_eq!(
            PythonRequest::parse("graalpy310"),
            PythonRequest::ImplementationVersion(
                ImplementationName::GraalPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
        );

        let tempdir = TempDir::new().unwrap();
        assert_eq!(
            PythonRequest::parse(tempdir.path().to_str().unwrap()),
            PythonRequest::Directory(tempdir.path().to_path_buf()),
            "An existing directory is treated as a directory"
        );
        assert_eq!(
            PythonRequest::parse(tempdir.child("foo").path().to_str().unwrap()),
            PythonRequest::File(tempdir.child("foo").path().to_path_buf()),
            "A path that does not exist is treated as a file"
        );
        tempdir.child("bar").touch().unwrap();
        assert_eq!(
            PythonRequest::parse(tempdir.child("bar").path().to_str().unwrap()),
            PythonRequest::File(tempdir.child("bar").path().to_path_buf()),
            "An existing file is treated as a file"
        );
        assert_eq!(
            PythonRequest::parse("./foo"),
            PythonRequest::File(PathBuf::from_str("./foo").unwrap()),
            "A string with a file system separator is treated as a file"
        );
        assert_eq!(
            PythonRequest::parse("3.13t"),
            PythonRequest::Version(VersionRequest::from_str("3.13t").unwrap())
        );
    }

    #[test]
    fn discovery_sources_prefer_system_orders_search_path_first() {
        let preferences = DiscoveryPreferences {
            python_preference: PythonPreference::System,
            environment_preference: EnvironmentPreference::OnlySystem,
        };
        let sources = preferences.sources(&PythonRequest::Default);

        if cfg!(windows) {
            assert_eq!(sources, "search path, registry, or managed installations");
        } else {
            assert_eq!(sources, "search path or managed installations");
        }
    }

    #[test]
    fn discovery_sources_only_system_matches_platform_order() {
        let preferences = DiscoveryPreferences {
            python_preference: PythonPreference::OnlySystem,
            environment_preference: EnvironmentPreference::OnlySystem,
        };
        let sources = preferences.sources(&PythonRequest::Default);

        if cfg!(windows) {
            assert_eq!(sources, "search path or registry");
        } else {
            assert_eq!(sources, "search path");
        }
    }

    #[test]
    fn interpreter_request_to_canonical_string() {
        assert_eq!(PythonRequest::Default.to_canonical_string(), "default");
        assert_eq!(PythonRequest::Any.to_canonical_string(), "any");
        assert_eq!(
            PythonRequest::Version(VersionRequest::from_str("3.12").unwrap()).to_canonical_string(),
            "3.12"
        );
        assert_eq!(
            PythonRequest::Version(VersionRequest::from_str(">=3.12").unwrap())
                .to_canonical_string(),
            ">=3.12"
        );
        assert_eq!(
            PythonRequest::Version(VersionRequest::from_str(">=3.12,<3.13").unwrap())
                .to_canonical_string(),
            ">=3.12, <3.13"
        );

        assert_eq!(
            PythonRequest::Version(VersionRequest::from_str("3.13.0a1").unwrap())
                .to_canonical_string(),
            "3.13a1"
        );

        assert_eq!(
            PythonRequest::Version(VersionRequest::from_str("3.13.0b5").unwrap())
                .to_canonical_string(),
            "3.13b5"
        );

        assert_eq!(
            PythonRequest::Version(VersionRequest::from_str("3.13.0rc1").unwrap())
                .to_canonical_string(),
            "3.13rc1"
        );

        assert_eq!(
            PythonRequest::Version(VersionRequest::from_str("313rc4").unwrap())
                .to_canonical_string(),
            "3.13rc4"
        );

        assert_eq!(
            PythonRequest::ExecutableName("foo".to_string()).to_canonical_string(),
            "foo"
        );
        assert_eq!(
            PythonRequest::Implementation(ImplementationName::CPython).to_canonical_string(),
            "cpython"
        );
        assert_eq!(
            PythonRequest::ImplementationVersion(
                ImplementationName::CPython,
                VersionRequest::from_str("3.12.2").unwrap(),
            )
            .to_canonical_string(),
            "cpython@3.12.2"
        );
        assert_eq!(
            PythonRequest::Implementation(ImplementationName::PyPy).to_canonical_string(),
            "pypy"
        );
        assert_eq!(
            PythonRequest::ImplementationVersion(
                ImplementationName::PyPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
            .to_canonical_string(),
            "pypy@3.10"
        );
        assert_eq!(
            PythonRequest::Implementation(ImplementationName::GraalPy).to_canonical_string(),
            "graalpy"
        );
        assert_eq!(
            PythonRequest::ImplementationVersion(
                ImplementationName::GraalPy,
                VersionRequest::from_str("3.10").unwrap(),
            )
            .to_canonical_string(),
            "graalpy@3.10"
        );

        let tempdir = TempDir::new().unwrap();
        assert_eq!(
            PythonRequest::Directory(tempdir.path().to_path_buf()).to_canonical_string(),
            tempdir.path().to_str().unwrap(),
            "An existing directory is treated as a directory"
        );
        assert_eq!(
            PythonRequest::File(tempdir.child("foo").path().to_path_buf()).to_canonical_string(),
            tempdir.child("foo").path().to_str().unwrap(),
            "A path that does not exist is treated as a file"
        );
        tempdir.child("bar").touch().unwrap();
        assert_eq!(
            PythonRequest::File(tempdir.child("bar").path().to_path_buf()).to_canonical_string(),
            tempdir.child("bar").path().to_str().unwrap(),
            "An existing file is treated as a file"
        );
        assert_eq!(
            PythonRequest::File(PathBuf::from_str("./foo").unwrap()).to_canonical_string(),
            "./foo",
            "A string with a file system separator is treated as a file"
        );
    }

    #[test]
    fn version_request_from_str() {
        assert_eq!(
            VersionRequest::from_str("3").unwrap(),
            VersionRequest::Major(3, PythonVariant::Default)
        );
        assert_eq!(
            VersionRequest::from_str("3.12").unwrap(),
            VersionRequest::MajorMinor(3, 12, PythonVariant::Default)
        );
        assert_eq!(
            VersionRequest::from_str("3.12.1").unwrap(),
            VersionRequest::MajorMinorPatch(3, 12, 1, PythonVariant::Default)
        );
        assert!(VersionRequest::from_str("1.foo.1").is_err());
        assert_eq!(
            VersionRequest::from_str("3").unwrap(),
            VersionRequest::Major(3, PythonVariant::Default)
        );
        assert_eq!(
            VersionRequest::from_str("38").unwrap(),
            VersionRequest::MajorMinor(3, 8, PythonVariant::Default)
        );
        assert_eq!(
            VersionRequest::from_str("312").unwrap(),
            VersionRequest::MajorMinor(3, 12, PythonVariant::Default)
        );
        assert_eq!(
            VersionRequest::from_str("3100").unwrap(),
            VersionRequest::MajorMinor(3, 100, PythonVariant::Default)
        );
        assert_eq!(
            VersionRequest::from_str("3.13a1").unwrap(),
            VersionRequest::MajorMinorPrerelease(
                3,
                13,
                Prerelease {
                    kind: PrereleaseKind::Alpha,
                    number: 1
                },
                PythonVariant::Default
            )
        );
        assert_eq!(
            VersionRequest::from_str("313b1").unwrap(),
            VersionRequest::MajorMinorPrerelease(
                3,
                13,
                Prerelease {
                    kind: PrereleaseKind::Beta,
                    number: 1
                },
                PythonVariant::Default
            )
        );
        assert_eq!(
            VersionRequest::from_str("3.13.0b2").unwrap(),
            VersionRequest::MajorMinorPrerelease(
                3,
                13,
                Prerelease {
                    kind: PrereleaseKind::Beta,
                    number: 2
                },
                PythonVariant::Default
            )
        );
        assert_eq!(
            VersionRequest::from_str("3.13.0rc3").unwrap(),
            VersionRequest::MajorMinorPrerelease(
                3,
                13,
                Prerelease {
                    kind: PrereleaseKind::Rc,
                    number: 3
                },
                PythonVariant::Default
            )
        );
        assert!(
            matches!(
                VersionRequest::from_str("3rc1"),
                Err(Error::InvalidVersionRequest(_))
            ),
            "Pre-release version requests require a minor version"
        );
        assert!(
            matches!(
                VersionRequest::from_str("3.13.2rc1"),
                Err(Error::InvalidVersionRequest(_))
            ),
            "Pre-release version requests require a patch version of zero"
        );
        assert!(
            matches!(
                VersionRequest::from_str("3.12-dev"),
                Err(Error::InvalidVersionRequest(_))
            ),
            "Development version segments are not allowed"
        );
        assert!(
            matches!(
                VersionRequest::from_str("3.12+local"),
                Err(Error::InvalidVersionRequest(_))
            ),
            "Local version segments are not allowed"
        );
        assert!(
            matches!(
                VersionRequest::from_str("3.12.post0"),
                Err(Error::InvalidVersionRequest(_))
            ),
            "Post version segments are not allowed"
        );
        assert!(
            // Test for overflow
            matches!(
                VersionRequest::from_str("31000"),
                Err(Error::InvalidVersionRequest(_))
            )
        );
        assert_eq!(
            VersionRequest::from_str("3t").unwrap(),
            VersionRequest::Major(3, PythonVariant::Freethreaded)
        );
        assert_eq!(
            VersionRequest::from_str("313t").unwrap(),
            VersionRequest::MajorMinor(3, 13, PythonVariant::Freethreaded)
        );
        assert_eq!(
            VersionRequest::from_str("3.13t").unwrap(),
            VersionRequest::MajorMinor(3, 13, PythonVariant::Freethreaded)
        );
        assert_eq!(
            VersionRequest::from_str(">=3.13t").unwrap(),
            VersionRequest::Range(
                VersionSpecifiers::from_str(">=3.13").unwrap(),
                PythonVariant::Freethreaded
            )
        );
        assert_eq!(
            VersionRequest::from_str(">=3.13").unwrap(),
            VersionRequest::Range(
                VersionSpecifiers::from_str(">=3.13").unwrap(),
                PythonVariant::Default
            )
        );
        assert_eq!(
            VersionRequest::from_str(">=3.12,<3.14t").unwrap(),
            VersionRequest::Range(
                VersionSpecifiers::from_str(">=3.12,<3.14").unwrap(),
                PythonVariant::Freethreaded
            )
        );
        assert!(matches!(
            VersionRequest::from_str("3.13tt"),
            Err(Error::InvalidVersionRequest(_))
        ));
    }

    #[test]
    fn executable_names_from_request() {
        fn case(request: &str, expected: &[&str]) {
            let (implementation, version) = match PythonRequest::parse(request) {
                PythonRequest::Any => (None, VersionRequest::Any),
                PythonRequest::Default => (None, VersionRequest::Default),
                PythonRequest::Version(version) => (None, version),
                PythonRequest::ImplementationVersion(implementation, version) => {
                    (Some(implementation), version)
                }
                PythonRequest::Implementation(implementation) => {
                    (Some(implementation), VersionRequest::Default)
                }
                result => {
                    panic!("Test cases should request versions or implementations; got {result:?}")
                }
            };

            let result: Vec<_> = version
                .executable_names(implementation.as_ref())
                .into_iter()
                .map(|name| name.to_string())
                .collect();

            let expected: Vec<_> = expected
                .iter()
                .map(|name| format!("{name}{exe}", exe = std::env::consts::EXE_SUFFIX))
                .collect();

            assert_eq!(result, expected, "mismatch for case \"{request}\"");
        }

        case(
            "any",
            &[
                "python", "python3", "cpython", "cpython3", "pypy", "pypy3", "graalpy", "graalpy3",
                "pyodide", "pyodide3",
            ],
        );

        case("default", &["python", "python3"]);

        case("3", &["python3", "python"]);

        case("4", &["python4", "python"]);

        case("3.13", &["python3.13", "python3", "python"]);

        case("pypy", &["pypy", "pypy3", "python", "python3"]);

        case(
            "pypy@3.10",
            &[
                "pypy3.10",
                "pypy3",
                "pypy",
                "python3.10",
                "python3",
                "python",
            ],
        );

        case(
            "3.13t",
            &[
                "python3.13t",
                "python3.13",
                "python3t",
                "python3",
                "pythont",
                "python",
            ],
        );
        case("3t", &["python3t", "python3", "pythont", "python"]);

        case(
            "3.13.2",
            &["python3.13.2", "python3.13", "python3", "python"],
        );

        case(
            "3.13rc2",
            &["python3.13rc2", "python3.13", "python3", "python"],
        );
    }

    #[test]
    fn test_try_split_prefix_and_version() {
        assert!(matches!(
            PythonRequest::try_split_prefix_and_version("prefix", "prefix"),
            Ok(None),
        ));
        assert!(matches!(
            PythonRequest::try_split_prefix_and_version("prefix", "prefix3"),
            Ok(Some(_)),
        ));
        assert!(matches!(
            PythonRequest::try_split_prefix_and_version("prefix", "prefix@3"),
            Ok(Some(_)),
        ));
        assert!(matches!(
            PythonRequest::try_split_prefix_and_version("prefix", "prefix3notaversion"),
            Ok(None),
        ));
        // Version parsing errors are only raised if @ is present.
        assert!(
            PythonRequest::try_split_prefix_and_version("prefix", "prefix@3notaversion").is_err()
        );
        // @ is not allowed if the prefix is empty.
        assert!(PythonRequest::try_split_prefix_and_version("", "@3").is_err());
    }

    #[test]
    fn version_request_as_pep440_version() {
        // Non-concrete requests return `None`
        assert_eq!(VersionRequest::Default.as_pep440_version(), None);
        assert_eq!(VersionRequest::Any.as_pep440_version(), None);
        assert_eq!(
            VersionRequest::from_str(">=3.10")
                .unwrap()
                .as_pep440_version(),
            None
        );

        // `VersionRequest::Major`
        assert_eq!(
            VersionRequest::Major(3, PythonVariant::Default).as_pep440_version(),
            Some(Version::from_str("3").unwrap())
        );

        // `VersionRequest::MajorMinor`
        assert_eq!(
            VersionRequest::MajorMinor(3, 12, PythonVariant::Default).as_pep440_version(),
            Some(Version::from_str("3.12").unwrap())
        );

        // `VersionRequest::MajorMinorPatch`
        assert_eq!(
            VersionRequest::MajorMinorPatch(3, 12, 5, PythonVariant::Default).as_pep440_version(),
            Some(Version::from_str("3.12.5").unwrap())
        );

        // `VersionRequest::MajorMinorPrerelease`
        assert_eq!(
            VersionRequest::MajorMinorPrerelease(
                3,
                14,
                Prerelease {
                    kind: PrereleaseKind::Alpha,
                    number: 1
                },
                PythonVariant::Default
            )
            .as_pep440_version(),
            Some(Version::from_str("3.14.0a1").unwrap())
        );
        assert_eq!(
            VersionRequest::MajorMinorPrerelease(
                3,
                14,
                Prerelease {
                    kind: PrereleaseKind::Beta,
                    number: 2
                },
                PythonVariant::Default
            )
            .as_pep440_version(),
            Some(Version::from_str("3.14.0b2").unwrap())
        );
        assert_eq!(
            VersionRequest::MajorMinorPrerelease(
                3,
                13,
                Prerelease {
                    kind: PrereleaseKind::Rc,
                    number: 3
                },
                PythonVariant::Default
            )
            .as_pep440_version(),
            Some(Version::from_str("3.13.0rc3").unwrap())
        );

        // Variant is ignored
        assert_eq!(
            VersionRequest::Major(3, PythonVariant::Freethreaded).as_pep440_version(),
            Some(Version::from_str("3").unwrap())
        );
        assert_eq!(
            VersionRequest::MajorMinor(3, 13, PythonVariant::Freethreaded).as_pep440_version(),
            Some(Version::from_str("3.13").unwrap())
        );
    }

    #[test]
    fn python_request_as_pep440_version() {
        // `PythonRequest::Any` and `PythonRequest::Default` return `None`
        assert_eq!(PythonRequest::Any.as_pep440_version(), None);
        assert_eq!(PythonRequest::Default.as_pep440_version(), None);

        // `PythonRequest::Version` delegates to `VersionRequest`
        assert_eq!(
            PythonRequest::Version(VersionRequest::MajorMinor(3, 11, PythonVariant::Default))
                .as_pep440_version(),
            Some(Version::from_str("3.11").unwrap())
        );

        // `PythonRequest::ImplementationVersion` extracts version
        assert_eq!(
            PythonRequest::ImplementationVersion(
                ImplementationName::CPython,
                VersionRequest::MajorMinorPatch(3, 12, 1, PythonVariant::Default),
            )
            .as_pep440_version(),
            Some(Version::from_str("3.12.1").unwrap())
        );

        // `PythonRequest::Implementation` returns `None` (no version)
        assert_eq!(
            PythonRequest::Implementation(ImplementationName::CPython).as_pep440_version(),
            None
        );

        // `PythonRequest::Key` with version
        assert_eq!(
            PythonRequest::parse("cpython-3.13.2").as_pep440_version(),
            Some(Version::from_str("3.13.2").unwrap())
        );

        // `PythonRequest::Key` without version returns `None`
        assert_eq!(
            PythonRequest::parse("cpython-macos-aarch64-none").as_pep440_version(),
            None
        );

        // Range versions return `None`
        assert_eq!(
            PythonRequest::Version(VersionRequest::from_str(">=3.10").unwrap()).as_pep440_version(),
            None
        );
    }

    #[test]
    fn intersects_requires_python_exact() {
        let requires_python =
            RequiresPython::from_specifiers(&VersionSpecifiers::from_str(">=3.12").unwrap());

        assert!(PythonRequest::parse("3.12").intersects_requires_python(&requires_python));
        assert!(!PythonRequest::parse("3.11").intersects_requires_python(&requires_python));
    }

    #[test]
    fn intersects_requires_python_major() {
        let requires_python =
            RequiresPython::from_specifiers(&VersionSpecifiers::from_str(">=3.12").unwrap());

        // `3` overlaps with `>=3.12` (e.g., 3.12, 3.13, ... are all Python 3)
        assert!(PythonRequest::parse("3").intersects_requires_python(&requires_python));
        // `2` does not overlap with `>=3.12`
        assert!(!PythonRequest::parse("2").intersects_requires_python(&requires_python));
    }

    #[test]
    fn intersects_requires_python_range() {
        let requires_python =
            RequiresPython::from_specifiers(&VersionSpecifiers::from_str(">=3.12").unwrap());

        assert!(PythonRequest::parse(">=3.12,<3.13").intersects_requires_python(&requires_python));
        assert!(!PythonRequest::parse(">=3.10,<3.12").intersects_requires_python(&requires_python));
    }

    #[test]
    fn intersects_requires_python_implementation_range() {
        let requires_python =
            RequiresPython::from_specifiers(&VersionSpecifiers::from_str(">=3.12").unwrap());

        assert!(
            PythonRequest::parse("cpython@>=3.12,<3.13")
                .intersects_requires_python(&requires_python)
        );
        assert!(
            !PythonRequest::parse("cpython@>=3.10,<3.12")
                .intersects_requires_python(&requires_python)
        );
    }

    #[test]
    fn intersects_requires_python_no_version() {
        let requires_python =
            RequiresPython::from_specifiers(&VersionSpecifiers::from_str(">=3.12").unwrap());

        // Requests without version constraints are always compatible
        assert!(PythonRequest::Any.intersects_requires_python(&requires_python));
        assert!(PythonRequest::Default.intersects_requires_python(&requires_python));
        assert!(
            PythonRequest::Implementation(ImplementationName::CPython)
                .intersects_requires_python(&requires_python)
        );
    }
}
