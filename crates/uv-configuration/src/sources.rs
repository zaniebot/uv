use uv_normalize::PackageName;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum SourceStrategy {
    /// Use `tool.uv.sources` when resolving dependencies.
    #[default]
    Enabled,
    /// Ignore `tool.uv.sources` when resolving dependencies.
    Disabled,
    /// Ignore `tool.uv.sources` for the given packages.
    Packages(Vec<PackageName>),
}

impl SourceStrategy {
    /// Return the [`SourceStrategy`] from the command-line arguments, if any.
    pub fn from_args(no_sources: Option<bool>, no_sources_package: Vec<PackageName>) -> Self {
        match no_sources {
            Some(true) => Self::Disabled,
            Some(false) => Self::Enabled,
            None => {
                if no_sources_package.is_empty() {
                    Self::Enabled
                } else {
                    Self::Packages(no_sources_package)
                }
            }
        }
    }

    /// Returns `true` if sources should be disabled for the given package.
    pub fn no_sources_package(&self, package_name: &PackageName) -> bool {
        match self {
            Self::Enabled => false,
            Self::Disabled => true,
            Self::Packages(packages) => packages.contains(package_name),
        }
    }

    /// Returns `true` if sources are enabled globally (for all packages).
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }

    /// Returns `true` if sources are disabled globally (for all packages).
    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }
}
