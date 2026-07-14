/// The format to use when exporting a `uv.lock` file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum ExportFormat {
    /// Export in `requirements.txt` format.
    #[default]
    #[serde(rename = "requirements.txt", alias = "requirements-txt")]
    #[cfg_attr(
        feature = "clap",
        clap(name = "requirements.txt", alias = "requirements-txt")
    )]
    RequirementsTxt,
    /// Export in `pylock.toml` format.
    #[serde(rename = "pylock.toml", alias = "pylock-toml")]
    #[cfg_attr(feature = "clap", clap(name = "pylock.toml", alias = "pylock-toml"))]
    PylockToml,
    /// Export in `CycloneDX` v1.5 JSON format.
    #[serde(rename = "cyclonedx1.5")]
    #[cfg_attr(
        feature = "clap",
        clap(name = "cyclonedx1.5", alias = "cyclonedx1.5+json")
    )]
    CycloneDX1_5,
}

/// The output format to use in `uv pip compile`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum PipCompileFormat {
    /// Export in `requirements.txt` format.
    #[default]
    #[serde(rename = "requirements.txt", alias = "requirements-txt")]
    #[cfg_attr(
        feature = "clap",
        clap(name = "requirements.txt", alias = "requirements-txt")
    )]
    RequirementsTxt,
    /// Export in `pylock.toml` format.
    #[serde(rename = "pylock.toml", alias = "pylock-toml")]
    #[cfg_attr(feature = "clap", clap(name = "pylock.toml", alias = "pylock-toml"))]
    PylockToml,
}

bitflags::bitflags! {
    /// The optional content to include in exported output.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct OutputFlags: u16 {
        /// Include distribution hashes.
        const HASHES = 1 << 0;
        /// Include extras in requirements.
        const EXTRAS = 1 << 1;
        /// Include environment markers in requirements.
        const MARKERS = 1 << 2;
        /// Include dependency annotations.
        const ANNOTATIONS = 1 << 3;
        /// Include the generated-file header.
        const HEADER = 1 << 4;
        /// Include index URLs.
        const INDEX_URL = 1 << 5;
        /// Include find-links locations.
        const FIND_LINKS = 1 << 6;
        /// Include build options.
        const BUILD_OPTIONS = 1 << 7;
        /// Include the marker expression.
        const MARKER_EXPRESSION = 1 << 8;
        /// Include index annotations.
        const INDEX_ANNOTATION = 1 << 9;
    }
}

/// Whether to generate hashes for exported output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashOutput {
    /// Generate and include hashes in the output.
    Generate,
    /// Omit hashes from the output.
    Omit,
}

impl HashOutput {
    /// Determine the [`HashOutput`] setting from the command-line arguments.
    pub const fn from_args(generate_hashes: bool) -> Self {
        if generate_hashes {
            Self::Generate
        } else {
            Self::Omit
        }
    }
}

/// Whether to include dependency annotations in exported output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationOutput {
    /// Include dependency annotations in the output.
    Include,
    /// Omit dependency annotations from the output.
    Omit,
}

impl AnnotationOutput {
    /// Determine the [`AnnotationOutput`] setting from the command-line arguments.
    pub const fn from_args(include_annotations: bool) -> Self {
        if include_annotations {
            Self::Include
        } else {
            Self::Omit
        }
    }
}

/// Whether to include the header in exported output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderOutput {
    /// Include the header in the output.
    Include,
    /// Omit the header from the output.
    Omit,
}

impl HeaderOutput {
    /// Determine the [`HeaderOutput`] setting from the command-line arguments.
    pub const fn from_args(include_header: bool) -> Self {
        if include_header {
            Self::Include
        } else {
            Self::Omit
        }
    }
}

/// Whether to include index URLs in exported output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexUrlOutput {
    /// Include index URLs in the output.
    Include,
    /// Omit index URLs from the output.
    Omit,
}

impl IndexUrlOutput {
    /// Determine the [`IndexUrlOutput`] setting from the command-line arguments.
    pub const fn from_args(include_index_url: bool) -> Self {
        if include_index_url {
            Self::Include
        } else {
            Self::Omit
        }
    }
}

/// Whether to include find-links locations in exported output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindLinksOutput {
    /// Include find-links locations in the output.
    Include,
    /// Omit find-links locations from the output.
    Omit,
}

impl FindLinksOutput {
    /// Determine the [`FindLinksOutput`] setting from the command-line arguments.
    pub const fn from_args(include_find_links: bool) -> Self {
        if include_find_links {
            Self::Include
        } else {
            Self::Omit
        }
    }
}
