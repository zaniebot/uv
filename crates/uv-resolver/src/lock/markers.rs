use std::str::FromStr;
use std::sync::LazyLock;

use uv_distribution_filename::WheelFilename;
use uv_distribution_types::RequiresPython;
use uv_pep508::MarkerTree;
use uv_platform_tags::{PlatformTag, Tags};

use crate::universal_marker::{ConflictMarker, UniversalMarker};

type PlatformPredicate = fn(&PlatformTag) -> bool;

#[derive(Clone, Copy)]
struct MarkerRule {
    predicate: PlatformPredicate,
    marker: &'static LazyLock<UniversalMarker>,
}

#[derive(Clone, Copy)]
struct PlatformFamilyRule {
    family: PlatformPredicate,
    family_marker: &'static LazyLock<UniversalMarker>,
    arch_rules: &'static [MarkerRule],
}

fn universal_marker(expression: &str) -> UniversalMarker {
    let pep508 = MarkerTree::from_str(expression).unwrap();
    UniversalMarker::new(pep508, ConflictMarker::TRUE)
}

fn combined_marker(
    base: &'static LazyLock<UniversalMarker>,
    arch: &'static LazyLock<UniversalMarker>,
) -> UniversalMarker {
    let mut marker = **base;
    marker.and(**arch);
    marker
}

static LINUX_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("os_name == 'posix' and sys_platform == 'linux'"));
static WINDOWS_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("os_name == 'nt' and sys_platform == 'win32'"));
static MAC_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("os_name == 'posix' and sys_platform == 'darwin'"));
static ANDROID_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("sys_platform == 'android'"));
static ARM_MARKERS: LazyLock<UniversalMarker> = LazyLock::new(|| {
    universal_marker(
        "platform_machine == 'aarch64' or platform_machine == 'arm64' or platform_machine == 'ARM64'",
    )
});
static X86_64_MARKERS: LazyLock<UniversalMarker> = LazyLock::new(|| {
    universal_marker(
        "platform_machine == 'x86_64' or platform_machine == 'amd64' or platform_machine == 'AMD64'",
    )
});
static X86_MARKERS: LazyLock<UniversalMarker> = LazyLock::new(|| {
    universal_marker(
        "platform_machine == 'i686' or platform_machine == 'i386' or platform_machine == 'win32' or platform_machine == 'x86'",
    )
});
static PPC64LE_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("platform_machine == 'ppc64le'"));
static PPC64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("platform_machine == 'ppc64'"));
static S390X_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("platform_machine == 's390x'"));
static RISCV64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("platform_machine == 'riscv64'"));
static LOONGARCH64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("platform_machine == 'loongarch64'"));
static ARMV7L_MARKERS: LazyLock<UniversalMarker> = LazyLock::new(|| {
    universal_marker("platform_machine == 'armv7l' or platform_machine == 'armv8l'")
});
static ARMV6L_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| universal_marker("platform_machine == 'armv6l'"));
static LINUX_ARM_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &ARM_MARKERS));
static LINUX_X86_64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &X86_64_MARKERS));
static LINUX_X86_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &X86_MARKERS));
static LINUX_PPC64LE_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &PPC64LE_MARKERS));
static LINUX_PPC64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &PPC64_MARKERS));
static LINUX_S390X_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &S390X_MARKERS));
static LINUX_RISCV64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &RISCV64_MARKERS));
static LINUX_LOONGARCH64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &LOONGARCH64_MARKERS));
static LINUX_ARMV7L_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &ARMV7L_MARKERS));
static LINUX_ARMV6L_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&LINUX_MARKERS, &ARMV6L_MARKERS));
static WINDOWS_ARM_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&WINDOWS_MARKERS, &ARM_MARKERS));
static WINDOWS_X86_64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&WINDOWS_MARKERS, &X86_64_MARKERS));
static WINDOWS_X86_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&WINDOWS_MARKERS, &X86_MARKERS));
static MAC_ARM_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&MAC_MARKERS, &ARM_MARKERS));
static MAC_X86_64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&MAC_MARKERS, &X86_64_MARKERS));
static MAC_X86_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&MAC_MARKERS, &X86_MARKERS));
static ANDROID_ARM_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&ANDROID_MARKERS, &ARM_MARKERS));
static ANDROID_X86_64_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&ANDROID_MARKERS, &X86_64_MARKERS));
static ANDROID_X86_MARKERS: LazyLock<UniversalMarker> =
    LazyLock::new(|| combined_marker(&ANDROID_MARKERS, &X86_MARKERS));

const GENERIC_ARCH_RULES: &[MarkerRule] = &[
    MarkerRule {
        predicate: PlatformTag::is_arm,
        marker: &ARM_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86_64,
        marker: &X86_64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86,
        marker: &X86_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_ppc64le,
        marker: &PPC64LE_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_ppc64,
        marker: &PPC64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_s390x,
        marker: &S390X_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_riscv64,
        marker: &RISCV64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_loongarch64,
        marker: &LOONGARCH64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_armv7l,
        marker: &ARMV7L_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_armv6l,
        marker: &ARMV6L_MARKERS,
    },
];

const LINUX_ARCH_RULES: &[MarkerRule] = &[
    MarkerRule {
        predicate: PlatformTag::is_arm,
        marker: &LINUX_ARM_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86_64,
        marker: &LINUX_X86_64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86,
        marker: &LINUX_X86_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_ppc64le,
        marker: &LINUX_PPC64LE_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_ppc64,
        marker: &LINUX_PPC64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_s390x,
        marker: &LINUX_S390X_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_riscv64,
        marker: &LINUX_RISCV64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_loongarch64,
        marker: &LINUX_LOONGARCH64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_armv7l,
        marker: &LINUX_ARMV7L_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_armv6l,
        marker: &LINUX_ARMV6L_MARKERS,
    },
];

const WINDOWS_ARCH_RULES: &[MarkerRule] = &[
    MarkerRule {
        predicate: PlatformTag::is_arm,
        marker: &WINDOWS_ARM_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86_64,
        marker: &WINDOWS_X86_64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86,
        marker: &WINDOWS_X86_MARKERS,
    },
];

const MAC_ARCH_RULES: &[MarkerRule] = &[
    MarkerRule {
        predicate: PlatformTag::is_arm,
        marker: &MAC_ARM_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86_64,
        marker: &MAC_X86_64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86,
        marker: &MAC_X86_MARKERS,
    },
];

const ANDROID_ARCH_RULES: &[MarkerRule] = &[
    MarkerRule {
        predicate: PlatformTag::is_arm,
        marker: &ANDROID_ARM_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86_64,
        marker: &ANDROID_X86_64_MARKERS,
    },
    MarkerRule {
        predicate: PlatformTag::is_x86,
        marker: &ANDROID_X86_MARKERS,
    },
];

const PLATFORM_FAMILY_RULES: &[PlatformFamilyRule] = &[
    PlatformFamilyRule {
        family: PlatformTag::is_linux,
        family_marker: &LINUX_MARKERS,
        arch_rules: LINUX_ARCH_RULES,
    },
    PlatformFamilyRule {
        family: PlatformTag::is_windows,
        family_marker: &WINDOWS_MARKERS,
        arch_rules: WINDOWS_ARCH_RULES,
    },
    PlatformFamilyRule {
        family: PlatformTag::is_macos,
        family_marker: &MAC_MARKERS,
        arch_rules: MAC_ARCH_RULES,
    },
    PlatformFamilyRule {
        family: PlatformTag::is_android,
        family_marker: &ANDROID_MARKERS,
        arch_rules: ANDROID_ARCH_RULES,
    },
];

fn all_tags_match(platform_tags: &[PlatformTag], predicate: PlatformPredicate) -> bool {
    platform_tags.iter().all(predicate)
}

fn disjoint_for_rules(
    platform_tags: &[PlatformTag],
    marker: &UniversalMarker,
    rules: &[MarkerRule],
) -> Option<bool> {
    rules
        .iter()
        .find(|rule| all_tags_match(platform_tags, rule.predicate))
        .map(|rule| marker.is_disjoint(**rule.marker))
}

pub(super) fn is_wheel_unreachable_for_marker(
    filename: &WheelFilename,
    requires_python: &RequiresPython,
    marker: &UniversalMarker,
    tags: Option<&Tags>,
) -> bool {
    if let Some(tags) = tags
        && !filename.compatibility(tags).is_compatible()
    {
        return true;
    }
    if !requires_python.matches_wheel_tag(filename) {
        return true;
    }

    let platform_tags = filename.platform_tags();
    if platform_tags.iter().all(PlatformTag::is_any) {
        return false;
    }

    for family_rule in PLATFORM_FAMILY_RULES {
        if all_tags_match(platform_tags, family_rule.family) {
            if let Some(disjoint) =
                disjoint_for_rules(platform_tags, marker, family_rule.arch_rules)
            {
                return disjoint;
            }
            if marker.is_disjoint(**family_rule.family_marker) {
                return true;
            }
        }
    }

    if let Some(disjoint) = disjoint_for_rules(platform_tags, marker, GENERIC_ARCH_RULES) {
        return disjoint;
    }

    false
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use uv_pep440::VersionSpecifiers;

    use super::*;

    fn marker(expression: &str) -> UniversalMarker {
        UniversalMarker::new(
            MarkerTree::from_str(expression).unwrap(),
            ConflictMarker::TRUE,
        )
    }

    fn requires_python(specifiers: &str) -> RequiresPython {
        RequiresPython::from_specifiers(&VersionSpecifiers::from_str(specifiers).unwrap())
    }

    #[test]
    fn linux_wheels_are_unreachable_for_windows_markers() {
        let filename = WheelFilename::from_str("example-1.0.0-py3-none-linux_x86_64.whl").unwrap();
        let marker = marker("os_name == 'nt' and sys_platform == 'win32'");

        assert!(is_wheel_unreachable_for_marker(
            &filename,
            &requires_python(">=3.8"),
            &marker,
            None,
        ));
    }

    #[test]
    fn linux_x86_64_wheels_are_unreachable_for_linux_arm_markers() {
        let filename =
            WheelFilename::from_str("example-1.0.0-cp312-cp312-manylinux_2_17_x86_64.whl").unwrap();
        let marker = marker(
            "os_name == 'posix' and sys_platform == 'linux' and (platform_machine == 'aarch64' or platform_machine == 'arm64' or platform_machine == 'ARM64')",
        );

        assert!(is_wheel_unreachable_for_marker(
            &filename,
            &requires_python(">=3.12"),
            &marker,
            None,
        ));
    }

    #[test]
    fn linux_x86_64_wheels_remain_reachable_for_linux_markers() {
        let filename = WheelFilename::from_str("example-1.0.0-py3-none-linux_x86_64.whl").unwrap();
        let marker = marker("os_name == 'posix' and sys_platform == 'linux'");

        assert!(!is_wheel_unreachable_for_marker(
            &filename,
            &requires_python(">=3.8"),
            &marker,
            None,
        ));
    }
}
