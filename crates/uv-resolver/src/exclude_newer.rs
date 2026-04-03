use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use rustc_hash::FxHashMap;
use serde::Deserialize;
use serde::de::value::MapAccessDeserializer;
use uv_normalize::PackageName;

// Re-export foundational types from uv-pypi-types.
pub(crate) use uv_pypi_types::ExcludeNewerSpan;
pub use uv_pypi_types::{ExcludeNewerValue, ExcludeNewerValueChange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExcludeNewerChange {
    GlobalChanged(ExcludeNewerValueChange),
    GlobalAdded(ExcludeNewerValue),
    GlobalRemoved,
    Package(ExcludeNewerPackageChange),
}

impl ExcludeNewerChange {
    /// Whether the change is due to a change in a relative timestamp.
    pub fn is_relative_timestamp_change(&self) -> bool {
        match self {
            Self::GlobalChanged(change) => change.is_relative_timestamp_change(),
            Self::GlobalAdded(_) | Self::GlobalRemoved => false,
            Self::Package(change) => change.is_relative_timestamp_change(),
        }
    }
}

impl std::fmt::Display for ExcludeNewerChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlobalChanged(change) => {
                write!(f, "{change}")
            }
            Self::GlobalAdded(value) => {
                write!(f, "addition of global exclude newer {value}")
            }
            Self::GlobalRemoved => write!(f, "removal of global exclude newer"),
            Self::Package(change) => {
                write!(f, "{change}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExcludeNewerPackageChange {
    PackageAdded(PackageName, PackageExcludeNewer),
    PackageRemoved(PackageName),
    PackageChanged(PackageName, Box<PackageExcludeNewerChange>),
}

impl ExcludeNewerPackageChange {
    pub fn is_relative_timestamp_change(&self) -> bool {
        match self {
            Self::PackageAdded(_, _) | Self::PackageRemoved(_) => false,
            Self::PackageChanged(_, change) => change.is_relative_timestamp_change(),
        }
    }
}

impl std::fmt::Display for ExcludeNewerPackageChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PackageAdded(name, PackageExcludeNewer::Enabled(value)) => {
                write!(
                    f,
                    "addition of exclude newer `{}` for package `{name}`",
                    value.as_ref()
                )
            }
            Self::PackageAdded(name, PackageExcludeNewer::Disabled) => {
                write!(
                    f,
                    "addition of exclude newer exclusion for package `{name}`"
                )
            }
            Self::PackageRemoved(name) => {
                write!(f, "removal of exclude newer for package `{name}`")
            }
            Self::PackageChanged(name, change) => write!(f, "{change} for package `{name}`"),
        }
    }
}

/// Per-package exclude-newer setting.
///
/// This enum represents whether exclude-newer should be disabled for a package,
/// or if a specific cutoff (absolute or relative) should be used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageExcludeNewer {
    /// Disable exclude-newer for this package (allow all versions regardless of upload date).
    Disabled,
    /// Enable exclude-newer with this cutoff for this package.
    Enabled(Box<ExcludeNewerValue>),
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for PackageExcludeNewer {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("PackageExcludeNewer")
    }

    fn json_schema(generator: &mut schemars::generate::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "oneOf": [
                {
                    "type": "boolean",
                    "const": false,
                    "description": "Disable exclude-newer for this package."
                },
                generator.subschema_for::<ExcludeNewerValue>(),
            ]
        })
    }
}

/// A package-specific exclude-newer entry.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ExcludeNewerPackageEntry {
    pub package: PackageName,
    pub setting: PackageExcludeNewer,
}

impl FromStr for ExcludeNewerPackageEntry {
    type Err = String;

    /// Parses a [`ExcludeNewerPackageEntry`] from a string in the format `PACKAGE=DATE` or `PACKAGE=false`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((package, value)) = s.split_once('=') else {
            return Err(format!(
                "Invalid `exclude-newer-package` value `{s}`: expected format `PACKAGE=DATE` or `PACKAGE=false`"
            ));
        };

        let package = PackageName::from_str(package).map_err(|err| {
            format!("Invalid `exclude-newer-package` package name `{package}`: {err}")
        })?;

        let setting = if value == "false" {
            PackageExcludeNewer::Disabled
        } else {
            PackageExcludeNewer::Enabled(Box::new(ExcludeNewerValue::from_str(value).map_err(
                |err| format!("Invalid `exclude-newer-package` value `{value}`: {err}"),
            )?))
        };

        Ok(Self { package, setting })
    }
}

impl From<(PackageName, PackageExcludeNewer)> for ExcludeNewerPackageEntry {
    fn from((package, setting): (PackageName, PackageExcludeNewer)) -> Self {
        Self { package, setting }
    }
}

impl From<(PackageName, ExcludeNewerValue)> for ExcludeNewerPackageEntry {
    fn from((package, timestamp): (PackageName, ExcludeNewerValue)) -> Self {
        Self {
            package,
            setting: PackageExcludeNewer::Enabled(Box::new(timestamp)),
        }
    }
}

impl<'de> serde::Deserialize<'de> for PackageExcludeNewer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PackageExcludeNewer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "a date/timestamp/duration string, false to disable exclude-newer, or a table \
                     with timestamp/span",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                ExcludeNewerValue::from_str(v)
                    .map(|ts| PackageExcludeNewer::Enabled(Box::new(ts)))
                    .map_err(|e| E::custom(format!("failed to parse exclude-newer value: {e}")))
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v {
                    Err(E::custom(
                        "expected false to disable exclude-newer, got true",
                    ))
                } else {
                    Ok(PackageExcludeNewer::Disabled)
                }
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                Ok(PackageExcludeNewer::Enabled(Box::new(
                    ExcludeNewerValue::deserialize(MapAccessDeserializer::new(map))?,
                )))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl serde::Serialize for PackageExcludeNewer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Enabled(timestamp) => timestamp.to_string().serialize(serializer),
            Self::Disabled => serializer.serialize_bool(false),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageExcludeNewerChange {
    Disabled { was: ExcludeNewerValue },
    Enabled { now: ExcludeNewerValue },
    TimestampChanged(ExcludeNewerValueChange),
}

impl PackageExcludeNewerChange {
    pub fn is_relative_timestamp_change(&self) -> bool {
        match self {
            Self::Disabled { .. } | Self::Enabled { .. } => false,
            Self::TimestampChanged(change) => change.is_relative_timestamp_change(),
        }
    }
}

impl std::fmt::Display for PackageExcludeNewerChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled { was } => {
                write!(f, "add exclude newer exclusion (was `{was}`)")
            }
            Self::Enabled { now } => {
                write!(f, "remove exclude newer exclusion (now `{now}`)")
            }
            Self::TimestampChanged(change) => write!(f, "{change}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ExcludeNewerPackage(FxHashMap<PackageName, PackageExcludeNewer>);

impl Deref for ExcludeNewerPackage {
    type Target = FxHashMap<PackageName, PackageExcludeNewer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ExcludeNewerPackage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<ExcludeNewerPackageEntry> for ExcludeNewerPackage {
    fn from_iter<T: IntoIterator<Item = ExcludeNewerPackageEntry>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|entry| (entry.package, entry.setting))
                .collect(),
        )
    }
}

impl IntoIterator for ExcludeNewerPackage {
    type Item = (PackageName, PackageExcludeNewer);
    type IntoIter = std::collections::hash_map::IntoIter<PackageName, PackageExcludeNewer>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ExcludeNewerPackage {
    type Item = (&'a PackageName, &'a PackageExcludeNewer);
    type IntoIter = std::collections::hash_map::Iter<'a, PackageName, PackageExcludeNewer>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl ExcludeNewerPackage {
    /// Convert to the inner `HashMap`.
    pub fn into_inner(self) -> FxHashMap<PackageName, PackageExcludeNewer> {
        self.0
    }

    /// Returns true if this map is empty (no package-specific settings).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn compare(&self, other: &Self) -> Option<ExcludeNewerPackageChange> {
        for (package, setting) in self {
            match (setting, other.get(package)) {
                (
                    PackageExcludeNewer::Enabled(self_timestamp),
                    Some(PackageExcludeNewer::Enabled(other_timestamp)),
                ) => {
                    if let Some(change) = self_timestamp.compare(other_timestamp) {
                        return Some(ExcludeNewerPackageChange::PackageChanged(
                            package.clone(),
                            Box::new(PackageExcludeNewerChange::TimestampChanged(change)),
                        ));
                    }
                }
                (
                    PackageExcludeNewer::Enabled(self_timestamp),
                    Some(PackageExcludeNewer::Disabled),
                ) => {
                    return Some(ExcludeNewerPackageChange::PackageChanged(
                        package.clone(),
                        Box::new(PackageExcludeNewerChange::Disabled {
                            was: self_timestamp.as_ref().clone(),
                        }),
                    ));
                }
                (
                    PackageExcludeNewer::Disabled,
                    Some(PackageExcludeNewer::Enabled(other_timestamp)),
                ) => {
                    return Some(ExcludeNewerPackageChange::PackageChanged(
                        package.clone(),
                        Box::new(PackageExcludeNewerChange::Enabled {
                            now: other_timestamp.as_ref().clone(),
                        }),
                    ));
                }
                (PackageExcludeNewer::Disabled, Some(PackageExcludeNewer::Disabled)) => {}
                (_, None) => {
                    return Some(ExcludeNewerPackageChange::PackageRemoved(package.clone()));
                }
            }
        }

        for (package, value) in other {
            if !self.contains_key(package) {
                return Some(ExcludeNewerPackageChange::PackageAdded(
                    package.clone(),
                    value.clone(),
                ));
            }
        }

        None
    }
}

/// A setting that excludes files newer than a timestamp, at a global level or per-package.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ExcludeNewer {
    /// Global timestamp that applies to all packages if no package-specific timestamp is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global: Option<ExcludeNewerValue>,
    /// Per-package timestamps that override the global timestamp.
    #[serde(default, skip_serializing_if = "FxHashMap::is_empty")]
    pub package: ExcludeNewerPackage,
}

impl ExcludeNewer {
    /// Create a new exclude newer configuration with just a global timestamp.
    pub fn global(global: ExcludeNewerValue) -> Self {
        Self {
            global: Some(global),
            package: ExcludeNewerPackage::default(),
        }
    }

    /// Create a new exclude newer configuration.
    pub fn new(global: Option<ExcludeNewerValue>, package: ExcludeNewerPackage) -> Self {
        Self { global, package }
    }

    /// Create from CLI arguments.
    pub fn from_args(
        global: Option<ExcludeNewerValue>,
        package: Vec<ExcludeNewerPackageEntry>,
    ) -> Self {
        let package: ExcludeNewerPackage = package.into_iter().collect();

        Self { global, package }
    }

    /// Returns the exclude-newer value for a specific package, returning `Some(value)` if the
    /// package has a package-specific setting or falls back to the global value if set, or `None`
    /// if exclude-newer is explicitly disabled for the package (set to `false`) or if no
    /// exclude-newer is configured.
    pub fn exclude_newer_package(&self, package_name: &PackageName) -> Option<ExcludeNewerValue> {
        match self.package.get(package_name) {
            Some(PackageExcludeNewer::Enabled(timestamp)) => Some(timestamp.as_ref().clone()),
            Some(PackageExcludeNewer::Disabled) => None,
            None => self.global.clone(),
        }
    }

    /// Returns the exclude-newer value for a specific package on a specific index.
    ///
    /// Resolution priority: package-specific > index-specific > global.
    /// If the index disables exclude-newer, the global value is skipped (but a package-specific
    /// override still applies).
    pub fn exclude_newer_for_index_package(
        &self,
        index_exclude_newer: Option<&IndexExcludeNewer>,
        package_name: &PackageName,
    ) -> Option<ExcludeNewerValue> {
        // Check package-specific first (highest priority).
        match self.package.get(package_name) {
            Some(PackageExcludeNewer::Enabled(timestamp)) => {
                return Some(timestamp.as_ref().clone());
            }
            Some(PackageExcludeNewer::Disabled) => return None,
            None => {}
        }
        // Then index-specific.
        match index_exclude_newer {
            Some(IndexExcludeNewer::Disabled) => None,
            Some(IndexExcludeNewer::Enabled(value)) => Some(value.as_ref().clone()),
            None => self.global.clone(),
        }
    }

    /// Returns true if this has any configuration (global or per-package).
    pub fn is_empty(&self) -> bool {
        self.global.is_none() && self.package.is_empty()
    }

    pub fn compare(&self, other: &Self) -> Option<ExcludeNewerChange> {
        match (&self.global, &other.global) {
            (Some(self_global), Some(other_global)) => {
                if let Some(change) = self_global.compare(other_global) {
                    return Some(ExcludeNewerChange::GlobalChanged(change));
                }
            }
            (None, Some(global)) => {
                return Some(ExcludeNewerChange::GlobalAdded(global.clone()));
            }
            (Some(_), None) => return Some(ExcludeNewerChange::GlobalRemoved),
            (None, None) => (),
        }
        self.package
            .compare(&other.package)
            .map(ExcludeNewerChange::Package)
    }
}

impl std::fmt::Display for ExcludeNewer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(global) = &self.global {
            write!(f, "global: {global}")?;
            if !self.package.is_empty() {
                write!(f, ", ")?;
            }
        }
        let mut first = true;
        for (name, setting) in &self.package {
            if !first {
                write!(f, ", ")?;
            }
            match setting {
                PackageExcludeNewer::Enabled(timestamp) => {
                    write!(f, "{name}: {}", timestamp.as_ref())?;
                }
                PackageExcludeNewer::Disabled => {
                    write!(f, "{name}: disabled")?;
                }
            }
            first = false;
        }
        Ok(())
    }
}

// Re-export `IndexExcludeNewer` from `uv-distribution-types` where it lives alongside `Index`.
pub use uv_distribution_types::IndexExcludeNewer;
