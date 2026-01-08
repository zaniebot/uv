//! PyPI platform tag compatibility checking.
//!
//! This module implements validation of wheel platform tags against PyPI's allowed
//! platform tags. PyPI only accepts a subset of platform tags, and this module allows
//! checking wheels before upload to avoid rejected uploads.
//!
//! The validation logic is based on PyPI's warehouse code:
//! <https://github.com/pypi/warehouse/blob/main/warehouse/forklift/legacy.py>

use std::path::Path;

use uv_distribution_filename::{DistFilename, WheelFilename};
use uv_platform_tags::{AndroidAbi, Arch, BinaryFormat, IosMultiarch, PlatformTag};

/// Errors that can occur during PyPI compatibility checking.
#[derive(Debug, thiserror::Error)]
pub enum PypiCompatError {
    /// A wheel has a platform tag not accepted by PyPI.
    #[error(
        "Wheel `{filename}` has platform tag `{platform_tag}` which is not accepted by PyPI{reason}"
    )]
    UnsupportedPlatformTag {
        filename: String,
        platform_tag: String,
        reason: String,
    },
}

/// Result of checking a distribution for PyPI compatibility.
#[derive(Debug)]
pub struct PypiCompatResult {
    /// The filename that was checked.
    pub filename: String,
    /// Any compatibility issues found.
    pub issues: Vec<PypiCompatError>,
}

impl PypiCompatResult {
    /// Returns `true` if the distribution is compatible with PyPI.
    pub fn is_compatible(&self) -> bool {
        self.issues.is_empty()
    }
}

/// macOS major versions allowed by PyPI (for versions > 10).
/// See: <https://github.com/pypi/warehouse/blob/main/warehouse/forklift/legacy.py>
const MACOS_MAJOR_VERSIONS: &[u16] = &[11, 12, 13, 14, 15, 26];

/// Architectures allowed for manylinux wheels by PyPI.
const MANYLINUX_ARCHES: &[Arch] = &[
    Arch::X86_64,
    Arch::X86,     // i686
    Arch::Aarch64,
    Arch::Armv7L,
    Arch::Powerpc64Le,
    Arch::S390X,
    Arch::Riscv64,
    Arch::Powerpc64, // Only manylinux, not musllinux
];

/// Architectures allowed for musllinux wheels by PyPI.
const MUSLLINUX_ARCHES: &[Arch] = &[
    Arch::X86_64,
    Arch::X86, // i686
    Arch::Aarch64,
    Arch::Armv7L,
    Arch::Powerpc64Le,
    Arch::S390X,
    Arch::Riscv64,
];

/// Architectures allowed for macOS wheels by PyPI.
const MACOS_BINARY_FORMATS: &[BinaryFormat] = &[
    BinaryFormat::Ppc,
    BinaryFormat::Ppc64,
    BinaryFormat::I386,
    BinaryFormat::X86_64,
    BinaryFormat::Arm64,
    BinaryFormat::Intel,
    BinaryFormat::Fat,
    // Note: fat3 is listed in warehouse but maps to Fat32 in uv
    BinaryFormat::Fat32,
    BinaryFormat::Fat64,
    BinaryFormat::Universal,
    BinaryFormat::Universal2,
];

/// Check if a platform tag is valid for PyPI uploads.
///
/// Based on PyPI's `_valid_platform_tag` function:
/// <https://github.com/pypi/warehouse/blob/main/warehouse/forklift/legacy.py>
fn is_valid_pypi_platform_tag(tag: &PlatformTag) -> Result<(), String> {
    match tag {
        // Universal wheel - always valid
        PlatformTag::Any => Ok(()),

        // Windows platforms - static list
        PlatformTag::Win32 | PlatformTag::WinAmd64 | PlatformTag::WinArm64 | PlatformTag::WinIa64 => Ok(()),

        // Legacy manylinux versions with specific arch restrictions
        PlatformTag::Manylinux1 { arch } => {
            if matches!(arch, Arch::X86_64 | Arch::X86) {
                Ok(())
            } else {
                Err(format!(
                    ": manylinux1 only supports x86_64 and i686 architectures, not {arch}"
                ))
            }
        }

        PlatformTag::Manylinux2010 { arch } => {
            if matches!(arch, Arch::X86_64 | Arch::X86) {
                Ok(())
            } else {
                Err(format!(
                    ": manylinux2010 only supports x86_64 and i686 architectures, not {arch}"
                ))
            }
        }

        PlatformTag::Manylinux2014 { arch } => {
            // manylinux2014 supports more architectures than manylinux1/2010
            if matches!(
                arch,
                Arch::X86_64
                    | Arch::X86
                    | Arch::Aarch64
                    | Arch::Armv7L
                    | Arch::Powerpc64
                    | Arch::Powerpc64Le
                    | Arch::S390X
            ) {
                Ok(())
            } else {
                Err(format!(
                    ": manylinux2014 does not support architecture {arch}"
                ))
            }
        }

        // PEP 600 manylinux (manylinux_X_Y_arch)
        PlatformTag::Manylinux { arch, .. } => {
            if MANYLINUX_ARCHES.contains(arch) {
                Ok(())
            } else {
                Err(format!(": manylinux does not support architecture {arch}"))
            }
        }

        // PEP 656 musllinux
        PlatformTag::Musllinux { arch, .. } => {
            if MUSLLINUX_ARCHES.contains(arch) {
                Ok(())
            } else {
                Err(format!(": musllinux does not support architecture {arch}"))
            }
        }

        // Plain linux - only armv6l and armv7l are allowed
        PlatformTag::Linux { arch } => {
            if matches!(arch, Arch::Armv6L | Arch::Armv7L) {
                Ok(())
            } else {
                Err(format!(
                    ": linux_{arch} is not accepted by PyPI. Use manylinux or musllinux tags instead"
                ))
            }
        }

        // macOS - version and arch validation
        PlatformTag::Macos {
            major,
            minor,
            binary_format,
        } => {
            if !MACOS_BINARY_FORMATS.contains(binary_format) {
                return Err(format!(
                    ": macOS binary format {binary_format} is not accepted by PyPI"
                ));
            }

            // For macOS 10.x, any minor version is allowed
            if *major == 10 {
                return Ok(());
            }

            // For newer macOS versions, only specific major versions with minor=0 are allowed
            if MACOS_MAJOR_VERSIONS.contains(major) && *minor == 0 {
                return Ok(());
            }

            if !MACOS_MAJOR_VERSIONS.contains(major) {
                Err(format!(
                    ": macOS major version {major} is not accepted by PyPI. Accepted versions: 10.x, {}",
                    MACOS_MAJOR_VERSIONS
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            } else {
                Err(format!(
                    ": macOS {major}.{minor} is not accepted by PyPI. For macOS {major}, only {major}.0 is allowed"
                ))
            }
        }

        // iOS - architecture validation
        PlatformTag::Ios { multiarch, .. } => {
            // PyPI accepts arm64 and x86_64 for iOS
            match multiarch {
                IosMultiarch::Arm64Device
                | IosMultiarch::Arm64Simulator
                | IosMultiarch::X86_64Simulator => Ok(()),
            }
        }

        // Android - ABI validation
        PlatformTag::Android { abi, .. } => {
            // PyPI accepts all standard Android ABIs
            match abi {
                AndroidAbi::ArmeabiV7a
                | AndroidAbi::Arm64V8a
                | AndroidAbi::X86
                | AndroidAbi::X86_64 => Ok(()),
            }
        }

        // Platforms not supported by PyPI
        PlatformTag::FreeBsd { .. } => Err(": FreeBSD is not supported by PyPI".to_string()),
        PlatformTag::NetBsd { .. } => Err(": NetBSD is not supported by PyPI".to_string()),
        PlatformTag::OpenBsd { .. } => Err(": OpenBSD is not supported by PyPI".to_string()),
        PlatformTag::Dragonfly { .. } => Err(": DragonFly BSD is not supported by PyPI".to_string()),
        PlatformTag::Haiku { .. } => Err(": Haiku is not supported by PyPI".to_string()),
        PlatformTag::Illumos { .. } => Err(": illumos is not supported by PyPI".to_string()),
        PlatformTag::Solaris { .. } => Err(": Solaris is not supported by PyPI".to_string()),
        PlatformTag::Pyodide { .. } => Err(": Pyodide/WASM is not supported by PyPI".to_string()),
    }
}

/// Check a wheel filename for PyPI compatibility.
fn check_wheel_filename(wheel: &WheelFilename) -> Vec<PypiCompatError> {
    let mut issues = Vec::new();

    for platform_tag in wheel.platform_tags() {
        if let Err(reason) = is_valid_pypi_platform_tag(platform_tag) {
            issues.push(PypiCompatError::UnsupportedPlatformTag {
                filename: wheel.to_string(),
                platform_tag: platform_tag.to_string(),
                reason,
            });
        }
    }

    issues
}

/// Check a distribution file for PyPI compatibility.
///
/// This function checks if a distribution file (wheel or source distribution)
/// would be accepted by PyPI based on its platform tags.
///
/// Source distributions are always compatible (they have no platform restrictions).
pub fn check_pypi_compat(filename: &DistFilename) -> PypiCompatResult {
    match filename {
        DistFilename::WheelFilename(wheel) => PypiCompatResult {
            filename: wheel.to_string(),
            issues: check_wheel_filename(wheel),
        },
        DistFilename::SourceDistFilename(sdist) => {
            // Source distributions are always compatible
            PypiCompatResult {
                filename: sdist.to_string(),
                issues: Vec::new(),
            }
        }
    }
}

/// Check a file path for PyPI compatibility.
///
/// Parses the filename and checks for compatibility. Returns `None` if the
/// filename cannot be parsed as a distribution file.
pub fn check_pypi_compat_path(path: &Path) -> Option<PypiCompatResult> {
    let filename = path.file_name()?.to_str()?;
    let dist_filename = DistFilename::try_from_normalized_filename(filename)?;
    Some(check_pypi_compat(&dist_filename))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn check_wheel(name: &str) -> PypiCompatResult {
        let wheel = WheelFilename::from_str(name).unwrap();
        PypiCompatResult {
            filename: wheel.to_string(),
            issues: check_wheel_filename(&wheel),
        }
    }

    #[test]
    fn test_any_platform() {
        let result = check_wheel("foo-1.0.0-py3-none-any.whl");
        assert!(result.is_compatible());
    }

    #[test]
    fn test_windows_platforms() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-win32.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-win_amd64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-win_arm64.whl").is_compatible());
    }

    #[test]
    fn test_manylinux1_platforms() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-manylinux1_x86_64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-manylinux1_i686.whl").is_compatible());
    }

    #[test]
    fn test_manylinux2014_platforms() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-manylinux2014_x86_64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-manylinux2014_aarch64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-manylinux2014_ppc64le.whl").is_compatible());
    }

    #[test]
    fn test_pep600_manylinux() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-manylinux_2_17_x86_64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-manylinux_2_28_aarch64.whl").is_compatible());
    }

    #[test]
    fn test_musllinux() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-musllinux_1_1_x86_64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-musllinux_1_2_aarch64.whl").is_compatible());
    }

    #[test]
    fn test_macos_10x() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-macosx_10_9_x86_64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-macosx_10_15_x86_64.whl").is_compatible());
    }

    #[test]
    fn test_macos_11plus() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-macosx_11_0_arm64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-macosx_12_0_arm64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-macosx_11_0_universal2.whl").is_compatible());
    }

    #[test]
    fn test_macos_invalid_minor() {
        let result = check_wheel("foo-1.0.0-cp39-cp39-macosx_11_1_arm64.whl");
        assert!(!result.is_compatible());
        assert!(result.issues[0]
            .to_string()
            .contains("only 11.0 is allowed"));
    }

    #[test]
    fn test_linux_armv6l_armv7l() {
        // These are the only plain linux tags accepted by PyPI
        assert!(check_wheel("foo-1.0.0-cp39-cp39-linux_armv6l.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-linux_armv7l.whl").is_compatible());
    }

    #[test]
    fn test_linux_x86_64_rejected() {
        let result = check_wheel("foo-1.0.0-cp39-cp39-linux_x86_64.whl");
        assert!(!result.is_compatible());
        assert!(result.issues[0]
            .to_string()
            .contains("Use manylinux or musllinux"));
    }

    #[test]
    fn test_freebsd_rejected() {
        let result = check_wheel("foo-1.0.0-cp39-cp39-freebsd_13_0_x86_64.whl");
        assert!(!result.is_compatible());
        assert!(result.issues[0].to_string().contains("FreeBSD"));
    }

    #[test]
    fn test_multiple_platform_tags() {
        // Wheel with multiple platform tags - common for manylinux compatibility
        let result = check_wheel(
            "foo-1.0.0-cp39-cp39-manylinux_2_17_x86_64.manylinux2014_x86_64.whl",
        );
        assert!(result.is_compatible());
    }

    #[test]
    fn test_ios_platforms() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-ios_13_0_arm64_iphoneos.whl").is_compatible());
        assert!(
            check_wheel("foo-1.0.0-cp39-cp39-ios_13_0_arm64_iphonesimulator.whl").is_compatible()
        );
        assert!(
            check_wheel("foo-1.0.0-cp39-cp39-ios_13_0_x86_64_iphonesimulator.whl").is_compatible()
        );
    }

    #[test]
    fn test_android_platforms() {
        assert!(check_wheel("foo-1.0.0-cp39-cp39-android_21_arm64_v8a.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-android_21_armeabi_v7a.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-android_21_x86_64.whl").is_compatible());
        assert!(check_wheel("foo-1.0.0-cp39-cp39-android_21_x86.whl").is_compatible());
    }
}
