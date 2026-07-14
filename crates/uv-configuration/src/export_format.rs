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

/// Whether to include extras in `uv pip compile` output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtrasOutput {
    /// Include extras in the output.
    Include,
    /// Strip extras from the output.
    Strip,
}

impl ExtrasOutput {
    /// Determine the [`ExtrasOutput`] setting from the command-line arguments.
    pub const fn from_args(include_extras: bool) -> Self {
        if include_extras {
            Self::Include
        } else {
            Self::Strip
        }
    }
}

/// Whether to include environment markers in `uv pip compile` output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkersOutput {
    /// Include environment markers in the output.
    Include,
    /// Strip environment markers from the output.
    Strip,
}

impl MarkersOutput {
    /// Determine the [`MarkersOutput`] setting from the command-line arguments.
    pub const fn from_args(include_markers: bool) -> Self {
        if include_markers {
            Self::Include
        } else {
            Self::Strip
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

/// Whether to include the header in `uv pip compile` output.
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

/// Whether to include index URLs in `uv pip compile` output.
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

/// Whether to include find-links locations in `uv pip compile` output.
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

/// Whether to include build options in `uv pip compile` output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildOptionsOutput {
    /// Include build options in the output.
    Include,
    /// Omit build options from the output.
    Omit,
}

impl BuildOptionsOutput {
    /// Determine the [`BuildOptionsOutput`] setting from the command-line arguments.
    pub const fn from_args(include_build_options: bool) -> Self {
        if include_build_options {
            Self::Include
        } else {
            Self::Omit
        }
    }
}

/// Whether to include the marker expression in `uv pip compile` output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerExpressionOutput {
    /// Include the marker expression in the output.
    Include,
    /// Omit the marker expression from the output.
    Omit,
}

impl MarkerExpressionOutput {
    /// Determine the [`MarkerExpressionOutput`] setting from the command-line arguments.
    pub const fn from_args(include_marker_expression: bool) -> Self {
        if include_marker_expression {
            Self::Include
        } else {
            Self::Omit
        }
    }
}

/// Whether to include index annotations in `uv pip compile` output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexAnnotationOutput {
    /// Include index annotations in the output.
    Include,
    /// Omit index annotations from the output.
    Omit,
}

impl IndexAnnotationOutput {
    /// Determine the [`IndexAnnotationOutput`] setting from the command-line arguments.
    pub const fn from_args(include_index_annotation: bool) -> Self {
        if include_index_annotation {
            Self::Include
        } else {
            Self::Omit
        }
    }
}
