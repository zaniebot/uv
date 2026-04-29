use owo_colors::OwoColorize;

use crate::installation::Changelog;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to prepare distributions")]
    Prepare(#[from] uv_installer::PrepareError),

    #[error(transparent)]
    Resolve(#[from] uv_resolver::ResolveError),

    #[error(transparent)]
    Uninstall(#[from] uv_installer::UninstallError),

    #[error(transparent)]
    Hash(#[from] uv_types::HashStrategyError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),

    #[error(transparent)]
    Requirements(#[from] uv_requirements::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("The environment is outdated; run `{}` to update the environment", "uv sync".cyan())]
    OutdatedEnvironment(Box<Changelog>),
}
