use super::*;

pub(super) fn index_locations_from_components(
    indexes: impl IntoIterator<Item = Index>,
    flat_index: impl IntoIterator<Item = Index>,
    no_index: bool,
) -> IndexLocations {
    IndexLocations::new(
        indexes.into_iter().collect(),
        flat_index.into_iter().collect(),
        no_index,
    )
}

pub(super) fn build_options_from_args(
    no_binary: Option<bool>,
    no_binary_package: Vec<PackageName>,
    no_build: Option<bool>,
    no_build_package: Vec<PackageName>,
) -> BuildOptions {
    BuildOptions::new(
        NoBinary::from_args(no_binary, no_binary_package),
        NoBuild::from_args(no_build, no_build_package),
    )
}

pub(super) fn sources_from_args(
    no_sources: Option<bool>,
    no_sources_package: Vec<PackageName>,
) -> NoSources {
    NoSources::from_args(no_sources, no_sources_package)
}
