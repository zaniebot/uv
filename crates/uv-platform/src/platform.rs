use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown operating system: {0}")]
    UnknownOs(String),
    #[error("Unknown architecture: {0}")]
    UnknownArch(String),
    #[error("Unknown libc environment: {0}")]
    UnknownLibc(String),
    #[error("Unsupported variant `{0}` for architecture `{1}`")]
    UnsupportedVariant(String, String),
    #[error(transparent)]
    LibcDetectionError(#[from] crate::libc::LibcDetectionError),
}