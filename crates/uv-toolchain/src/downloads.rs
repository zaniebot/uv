use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::implementation::{Error as ImplementationError, ImplementationName};
use crate::platform::{Arch, Libc, Os};
use crate::toolchain::ToolchainKey;
use crate::{PythonVersion, ToolchainRequest, VersionRequest};
use thiserror::Error;
use uv_client::BetterReqwestError;

use futures::TryStreamExt;

use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::debug;
use url::Url;
use uv_fs::Simplified;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    ImplementationError(#[from] ImplementationError),
    #[error("Invalid python version: {0}")]
    InvalidPythonVersion(String),
    #[error("Download failed")]
    NetworkError(#[from] BetterReqwestError),
    #[error("Download failed")]
    NetworkMiddlewareError(#[source] anyhow::Error),
    #[error("Failed to extract archive: {0}")]
    ExtractError(String, #[source] uv_extract::Error),
    #[error("Invalid download url")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Failed to create download directory")]
    DownloadDirError(#[source] io::Error),
    #[error("Failed to copy to: {0}", to.user_display())]
    CopyError {
        to: PathBuf,
        #[source]
        err: io::Error,
    },
    #[error("Failed to read toolchain directory: {0}", dir.user_display())]
    ReadError {
        dir: PathBuf,
        #[source]
        err: io::Error,
    },
    #[error("Failed to parse toolchain directory name: {0}")]
    NameError(String),
    #[error("Cannot download toolchain for request: {0}")]
    InvalidRequestKind(ToolchainRequest),
    // TODO(zanieb): Implement display for `PythonDownloadRequest`
    #[error("No download found for request: {0:?}")]
    NoDownloadFound(ToolchainKey),
}

#[derive(Debug, PartialEq)]
pub struct PythonDownload {
    implementation: ImplementationName,
    arch: Arch,
    os: Os,
    libc: Libc,
    major: u8,
    minor: u8,
    patch: u8,
    url: &'static str,
    sha256: Option<&'static str>,
}

include!("downloads.inc");

pub enum DownloadResult {
    AlreadyAvailable(PathBuf),
    Fetched(PathBuf),
}

/// Iterate over all [`PythonDownload`]'s that match a [`ToolchainRequest`].
pub fn find_downloads_matching_request(
    request: &ToolchainRequest,
) -> Result<impl Iterator<Item = &'static PythonDownload> + '_, Error> {
    let key = match request {
        ToolchainRequest::Any => ToolchainKey::from_env()?,
        ToolchainRequest::Directory(_) => return Err(Error::InvalidRequestKind(request.clone())),
        ToolchainRequest::ExecutableName(_) => {
            return Err(Error::InvalidRequestKind(request.clone()))
        }
        ToolchainRequest::File(_) => return Err(Error::InvalidRequestKind(request.clone())),
        ToolchainRequest::Implementation(implementation) => {
            ToolchainKey::new(implementation.clone(), None, None, None, None)
        }
    };

    PythonDownload::iter_all().filter(move |download| {
        if let Some(arch) = &request.arch {
            if download.arch() != *arch {
                return false;
            }
        }
        if let Some(os) = &request.os {
            if download.os() != *os {
                return false;
            }
        }
        if let Some(implementation) = &request.implementation {
            if download.implementation() != *implementation {
                return false;
            }
        }
        if let Some(version) = &request.version {
            if !version.matches_major_minor_patch(
                download.major_version(),
                download.minor_version(),
                download.patch_version(),
            ) {
                return false;
            }
        }
        true
    })
}

impl PythonDownload {
    /// Return the first [`PythonDownload`] matching a request, if any.
    pub fn from_request(request: &ToolchainRequest) -> Result<&'static PythonDownload, Error> {
        find_downloads_matching_request(request)?
            .next()
            .ok_or(Error::NoDownloadFound(request.clone()))
    }

    /// Iterate over all [`PythonDownload`]'s.
    pub fn iter_all() -> impl Iterator<Item = &'static PythonDownload> {
        PYTHON_DOWNLOADS.iter()
    }

    pub fn url(&self) -> &str {
        self.url
    }

    pub fn arch(&self) -> &Arch {
        &self.arch
    }

    pub fn os(&self) -> &Os {
        &self.os
    }

    pub fn libc(&self) -> &Libc {
        &self.libc
    }

    pub fn major_version(&self) -> u8 {
        self.major
    }

    pub fn minor_version(&self) -> u8 {
        self.minor
    }

    pub fn patch_version(&self) -> u8 {
        self.patch
    }

    pub fn implementation(&self) -> &ImplementationName {
        &self.implementation
    }

    pub fn key(&self) -> ToolchainKey {
        ToolchainKey::new(
            self.implementation.clone(),
            self.arch.clone(),
            self.os.clone(),
            self.libc.clone(),
            self.major,
            self.minor,
            self.patch,
        )
    }

    pub fn os(&self) -> &Os {
        &self.os
    }

    pub fn sha256(&self) -> Option<&str> {
        self.sha256
    }

    /// Download and extract
    pub async fn fetch(
        &self,
        client: &uv_client::BaseClient,
        parent_path: &Path,
    ) -> Result<DownloadResult, Error> {
        let url = Url::parse(self.url)?;
        let path = parent_path.join(self.key()).clone();

        // If it already exists, return it
        if path.is_dir() {
            return Ok(DownloadResult::AlreadyAvailable(path));
        }

        let filename = url.path_segments().unwrap().last().unwrap();
        let response = client.get(url.clone()).send().await?;

        // Ensure the request was successful.
        response.error_for_status_ref()?;

        // Download and extract into a temporary directory.
        let temp_dir = tempfile::tempdir_in(parent_path).map_err(Error::DownloadDirError)?;

        debug!(
            "Downloading {url} to temporary location {}",
            temp_dir.path().display()
        );
        let reader = response
            .bytes_stream()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
            .into_async_read();

        debug!("Extracting {filename}");
        uv_extract::stream::archive(reader.compat(), filename, temp_dir.path())
            .await
            .map_err(|err| Error::ExtractError(filename.to_string(), err))?;

        // Extract the top-level directory.
        let extracted = match uv_extract::strip_component(temp_dir.path()) {
            Ok(top_level) => top_level,
            Err(uv_extract::Error::NonSingularArchive(_)) => temp_dir.into_path(),
            Err(err) => return Err(Error::ExtractError(filename.to_string(), err)),
        };

        // Persist it to the target
        debug!("Moving {} to {}", extracted.display(), path.user_display());
        fs_err::tokio::rename(extracted, &path)
            .await
            .map_err(|err| Error::CopyError {
                to: path.clone(),
                err,
            })?;

        Ok(DownloadResult::Fetched(path))
    }

    pub fn python_version(&self) -> PythonVersion {
        PythonVersion::from_str(&format!("{}.{}.{}", self.major, self.minor, self.patch))
            .expect("Python downloads should always have valid versions")
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::NetworkError(BetterReqwestError::from(error))
    }
}

impl From<reqwest_middleware::Error> for Error {
    fn from(error: reqwest_middleware::Error) -> Self {
        match error {
            reqwest_middleware::Error::Middleware(error) => Self::NetworkMiddlewareError(error),
            reqwest_middleware::Error::Reqwest(error) => {
                Self::NetworkError(BetterReqwestError::from(error))
            }
        }
    }
}

impl Display for PythonDownload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.key())
    }
}
