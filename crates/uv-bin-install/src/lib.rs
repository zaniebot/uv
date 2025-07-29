//! Binary download and installation utilities for uv.
//!
//! These utilities are specifically for consuming distributions that are _not_ Python packages,
//! e.g., `ruff` (which does have a Python package, but also has standalone binaries on GitHub).

use std::path::PathBuf;
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};

use futures::TryStreamExt;
use serde::Deserialize;
use thiserror::Error;
use tokio::io::{AsyncRead, ReadBuf};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use url::Url;

use uv_cache::{Cache, CacheBucket, CacheEntry};
use uv_client::BaseClient;
use uv_distribution_filename::SourceDistExtension;
use uv_extract::stream;
use uv_pep440::Version;
use uv_platform::{Arch, Libc, Os};

/// Binary tools that can be installed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binary {
    Ruff,
    Uv,
}

impl Binary {
    /// Get the default version for this binary.
    pub fn default_version(&self) -> Version {
        match self {
            Binary::Ruff => Version::from_str("0.12.5").expect("valid version"),
            Binary::Uv => Version::from_str("0.5.0").expect("valid version"),
        }
    }

    /// The name of the binary.
    ///
    /// See [`Binary::executable`] for the platform-specific executable name.
    pub fn name(&self) -> &'static str {
        match self {
            Binary::Ruff => "ruff",
            Binary::Uv => "uv",
        }
    }

    /// Get the download URL for a specific version and platform.
    pub fn download_url(
        &self,
        version: &Version,
        platform: &str,
        ext: &SourceDistExtension,
    ) -> Result<Url, Error> {
        match self {
            Binary::Ruff => {
                let url = format!(
                    "https://github.com/astral-sh/ruff/releases/download/{version}/ruff-{platform}.{ext}"
                );
                Url::parse(&url).map_err(|err| Error::UrlParse { url, source: err })
            }
            Binary::Uv => {
                let url = format!(
                    "https://github.com/astral-sh/uv/releases/download/{version}/uv-{platform}.{ext}"
                );
                Url::parse(&url).map_err(|err| Error::UrlParse { url, source: err })
            }
        }
    }

    /// Get the executable name
    pub fn executable(&self) -> String {
        format!("{}{}", self.name(), std::env::consts::EXE_SUFFIX)
    }
}

/// Binary version manifest structure.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BinVersionManifest {
    versions: Vec<BinVersionInfo>,
}

/// Binary version information.
#[derive(Debug, Deserialize)]
struct BinVersionInfo {
    version: String,
    #[allow(dead_code)]
    date: String,
    artifacts: Vec<BinArtifact>,
}

/// Binary artifact information.
#[derive(Debug, Deserialize)]
struct BinArtifact {
    platform: String,
    url: String,
    #[allow(dead_code)]
    sha256_url: String,
    archive_format: String,
}

/// Errors that can occur during binary download and installation.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to download binary.
    #[error("Failed to download {tool} {version} from {url}")]
    Download {
        tool: String,
        version: String,
        url: String,
        #[source]
        source: reqwest_middleware::Error,
    },

    /// Failed to parse download URL.
    #[error("Failed to parse download URL: {url}")]
    UrlParse {
        url: String,
        #[source]
        source: url::ParseError,
    },

    /// Failed to extract archive.
    #[error("Failed to extract {tool} archive")]
    Extract {
        tool: String,
        #[source]
        source: anyhow::Error,
    },

    /// Binary not found in extracted archive.
    #[error("Binary not found in {tool} archive at expected location: {expected}")]
    BinaryNotFound { tool: String, expected: PathBuf },

    /// I/O error during installation.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Platform detection error.
    #[error("Failed to detect platform")]
    Platform(#[from] uv_platform::Error),

    /// Version manifest fetch error.
    #[error("Failed to fetch version manifest")]
    ManifestFetch(#[from] reqwest_middleware::Error),

    /// Version parsing error.
    #[error("Failed to parse version: {version}")]
    VersionParse {
        version: String,
        #[source]
        source: uv_pep440::VersionParseError,
    },

    /// No compatible version found.
    #[error("No compatible version found for constraints: {constraints}")]
    NoCompatibleVersion { constraints: String },

    /// Unsupported archive format.
    #[error("Unsupported archive format: {0}")]
    UnsupportedArchiveFormat(String),
}

/// Progress reporter for binary downloads.
pub trait Reporter: Send + Sync {
    /// Called when a download starts.
    fn on_download_start(&self, name: &str, version: &Version, size: Option<u64>) -> usize;
    /// Called when download progress is made.
    fn on_download_progress(&self, id: usize, inc: u64);
    /// Called when a download completes.
    fn on_download_complete(&self, id: usize);
}

/// An asynchronous reader that reports progress as bytes are read.
struct ProgressReader<'a, R> {
    reader: R,
    index: usize,
    reporter: &'a dyn Reporter,
}

impl<'a, R> ProgressReader<'a, R> {
    /// Create a new [`ProgressReader`] that wraps another reader.
    fn new(reader: R, index: usize, reporter: &'a dyn Reporter) -> Self {
        Self {
            reader,
            index,
            reporter,
        }
    }
}

impl<R> AsyncRead for ProgressReader<'_, R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let before = buf.filled().len();
        match Pin::new(&mut self.reader).poll_read(cx, buf) {
            Poll::Ready(Ok(())) => {
                let after = buf.filled().len();
                let bytes = after - before;
                if bytes > 0 {
                    self.reporter.on_download_progress(self.index, bytes as u64);
                }
                Poll::Ready(Ok(()))
            }
            poll => poll,
        }
    }
}

/// Find a compatible version from the versions manifest.
///
/// Uses NDJSON format for efficient streaming line-by-line.
async fn find_compatible_version(
    binary: Binary,
    constraints: &uv_pep440::VersionSpecifiers,
    client: &uv_client::BaseClient,
) -> Result<(Version, Vec<BinArtifact>), Error> {
    use futures::StreamExt;

    // Fetch the NDJSON versions manifest
    let manifest_url = format!(
        "https://zanieb.github.io/versions/v1/{}.ndjson",
        binary.name()
    );
    let manifest_url_parsed = Url::parse(&manifest_url).map_err(|err| Error::UrlParse {
        url: manifest_url.clone(),
        source: err,
    })?;

    let response = client
        .for_host(&manifest_url_parsed.clone().into())
        .get(manifest_url_parsed.clone())
        .send()
        .await?
        .error_for_status()
        .map_err(reqwest_middleware::Error::Reqwest)?;

    // Stream the response line by line
    let mut stream = response.bytes_stream();
    let mut buffer = Vec::new();

    while let Some(chunk) = stream.next().await {
        let chunk =
            chunk.map_err(|e| Error::ManifestFetch(reqwest_middleware::Error::Reqwest(e)))?;
        buffer.extend_from_slice(&chunk);

        // Process complete lines
        while let Some(newline_pos) = buffer.iter().position(|&b| b == b'\n') {
            let line = buffer.drain(..=newline_pos).collect::<Vec<_>>();

            // Skip empty lines
            if line.len() <= 1 {
                continue;
            }

            // Parse the JSON line (removing the newline)
            let line_str = std::str::from_utf8(&line[..line.len() - 1]).map_err(|e| {
                Error::NoCompatibleVersion {
                    constraints: format!("Invalid UTF-8 in manifest: {}", e),
                }
            })?;

            let version_info: BinVersionInfo =
                serde_json::from_str(line_str).map_err(|err| Error::NoCompatibleVersion {
                    constraints: format!("Failed to parse version line: {}", err),
                })?;

            let version =
                Version::from_str(&version_info.version).map_err(|source| Error::VersionParse {
                    version: version_info.version.clone(),
                    source,
                })?;

            if constraints.contains(&version) {
                return Ok((version, version_info.artifacts));
            }
        }
    }

    // Process any remaining data in buffer
    if !buffer.is_empty() {
        let line_str = std::str::from_utf8(&buffer).map_err(|e| Error::NoCompatibleVersion {
            constraints: format!("Invalid UTF-8 in manifest: {}", e),
        })?;

        if !line_str.trim().is_empty() {
            let version_info: BinVersionInfo =
                serde_json::from_str(line_str).map_err(|err| Error::NoCompatibleVersion {
                    constraints: format!("Failed to parse version line: {}", err),
                })?;

            let version =
                Version::from_str(&version_info.version).map_err(|source| Error::VersionParse {
                    version: version_info.version.clone(),
                    source,
                })?;

            if constraints.contains(&version) {
                return Ok((version, version_info.artifacts));
            }
        }
    }

    Err(Error::NoCompatibleVersion {
        constraints: constraints.to_string(),
    })
}

/// Install a binary for the given tool.
pub async fn bin_install(
    binary: Binary,
    version: Option<&Version>,
    client: &BaseClient,
    cache: &Cache,
    reporter: Option<&dyn Reporter>,
) -> Result<PathBuf, Error> {
    // For uv, use the manifest-based approach
    if binary == Binary::Uv {
        let version = version.cloned().unwrap_or_else(|| binary.default_version());
        let constraints = uv_pep440::VersionSpecifiers::from(
            uv_pep440::VersionSpecifier::from_version(uv_pep440::Operator::Equal, version.clone())
                .unwrap(),
        );
        return install_compatible_uv(&constraints, client, cache, reporter).await;
    }
    let os = Os::from_env();
    let arch = Arch::from_env();
    let libc = Libc::from_env()?;
    let version = version.cloned().unwrap_or_else(|| binary.default_version());
    let platform_name = platform_name_for_binary(os, arch, libc);

    // Check the cache first
    let cache_entry = CacheEntry::new(
        cache
            .bucket(CacheBucket::Binaries)
            .join(binary.name())
            .join(version.to_string())
            .join(&platform_name),
        binary.executable(),
    );

    if let Ok(true) = cache_entry.path().try_exists() {
        return Ok(cache_entry.into_path_buf());
    }

    let ext = if os.is_windows() {
        SourceDistExtension::Zip
    } else {
        SourceDistExtension::TarGz
    };

    let download_url = binary.download_url(&version, &platform_name, &ext)?;

    let cache_dir = cache_entry.dir();
    tokio::fs::create_dir_all(&cache_dir).await?;

    // Create a temporary directory for extraction
    let temp_dir = tempfile::tempdir_in(cache_dir.parent().unwrap())?;

    let response = client
        .for_host(&download_url.clone().into())
        .get(download_url.clone())
        .send()
        .await
        .map_err(|err| Error::Download {
            tool: binary.name().to_string(),
            version: version.to_string(),
            url: download_url.to_string(),
            source: err,
        })?;

    let response = response.error_for_status().map_err(|err| Error::Download {
        tool: binary.name().to_string(),
        version: version.to_string(),
        url: download_url.to_string(),
        source: reqwest_middleware::Error::Reqwest(err),
    })?;

    // Get the download size from headers if available
    let size = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u64>().ok());

    // Stream download directly to extraction
    let mut reader = response
        .bytes_stream()
        .map_err(std::io::Error::other)
        .into_async_read()
        .compat();

    if let Some(reporter) = reporter {
        let id = reporter.on_download_start(binary.name(), &version, size);
        let mut progress_reader = ProgressReader::new(reader, id, reporter);
        stream::archive(&mut progress_reader, ext, temp_dir.path())
            .await
            .map_err(|e| Error::Extract {
                tool: binary.name().to_string(),
                source: e.into(),
            })?;
        reporter.on_download_complete(id);
    } else {
        stream::archive(&mut reader, ext, temp_dir.path())
            .await
            .map_err(|e| Error::Extract {
                tool: binary.name().to_string(),
                source: e.into(),
            })?;
    }

    // Find the binary in the extracted files
    // The archive contains a directory with the platform name
    let extracted_binary = temp_dir
        .path()
        .join(format!("{}-{platform_name}", binary.name()))
        .join(binary.executable());

    uv_fs::rename_with_retry(&extracted_binary, cache_entry.path()).await?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(cache_entry.path()).await?.permissions();
        perms.set_mode(0o755);
        tokio::fs::set_permissions(cache_entry.path(), perms).await?;
    }

    Ok(cache_entry.into_path_buf())
}

/// Find a binary in a directory, searching common archive structures.
fn find_binary_in_dir(dir: &std::path::Path, binary_name: &str) -> Result<PathBuf, Error> {
    // Check if the binary is directly in the directory
    let direct_path = dir.join(binary_name);
    if direct_path.exists() {
        return Ok(direct_path);
    }

    // Check if it's in a subdirectory (common for archives)
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let nested_path = entry.path().join(binary_name);
            if nested_path.exists() {
                return Ok(nested_path);
            }
        }
    }

    Err(Error::BinaryNotFound {
        tool: binary_name.to_string(),
        expected: dir.join(binary_name),
    })
}

/// Install a binary with specific artifact URLs from the manifest.
async fn install_binary_with_artifacts(
    binary: Binary,
    version: &Version,
    artifacts: Vec<BinArtifact>,
    client: &BaseClient,
    cache: &Cache,
    reporter: Option<&dyn Reporter>,
) -> Result<PathBuf, Error> {
    let os = Os::from_env();
    let arch = Arch::from_env();
    let libc = Libc::from_env()?;
    let platform_name = platform_name_for_binary(os, arch, libc);

    // Find the artifact for this platform
    let artifact = artifacts
        .into_iter()
        .find(|a| a.platform == platform_name)
        .ok_or_else(|| Error::NoCompatibleVersion {
            constraints: format!("No artifact found for platform {}", platform_name),
        })?;

    // Check the cache first
    let cache_entry = CacheEntry::new(
        cache
            .bucket(CacheBucket::Binaries)
            .join(binary.name())
            .join(version.to_string())
            .join(&platform_name),
        binary.executable(),
    );

    if let Ok(true) = cache_entry.path().try_exists() {
        return Ok(cache_entry.into_path_buf());
    }

    let download_url = Url::parse(&artifact.url).map_err(|err| Error::UrlParse {
        url: artifact.url.clone(),
        source: err,
    })?;

    let cache_dir = cache_entry.dir();
    tokio::fs::create_dir_all(&cache_dir).await?;

    // Create a temporary directory for extraction
    let temp_dir = tempfile::tempdir_in(cache_dir.parent().unwrap())?;

    // If the user provided a custom certificate bundle, use it for the download
    let resp = client
        .for_host(&download_url.clone().into())
        .get(download_url)
        .send()
        .await?
        .error_for_status()
        .map_err(reqwest_middleware::Error::Reqwest)?;

    // Download the artifact
    let reader = resp
        .bytes_stream()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
        .into_async_read()
        .compat();

    let reader: Pin<Box<dyn tokio::io::AsyncRead + Send + Sync>> = if let Some(reporter) = reporter
    {
        Box::pin(ProgressReader::new(reader, 0, reporter))
    } else {
        Box::pin(reader)
    };

    // Determine the extension from the archive format field
    let ext = match artifact.archive_format.as_str() {
        "tar.gz" => SourceDistExtension::TarGz,
        "zip" => SourceDistExtension::Zip,
        other => return Err(Error::UnsupportedArchiveFormat(other.to_string())),
    };

    // Extract the binary
    uv_extract::stream::archive(reader, ext, temp_dir.path())
        .await
        .map_err(|e| Error::Extract {
            tool: binary.name().to_string(),
            source: e.into(),
        })?;

    // Find the binary in the extracted files
    let binary_name = binary.executable();
    let extracted_binary = find_binary_in_dir(temp_dir.path(), &binary_name)?;

    // Move the binary to the cache location
    match tokio::fs::rename(&extracted_binary, cache_entry.path()).await {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::CrossesDevices => {
            tokio::fs::copy(&extracted_binary, cache_entry.path()).await?;
            tokio::fs::remove_file(&extracted_binary).await?;
        }
        Err(err) => return Err(err.into()),
    }

    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(cache_entry.path()).await?.permissions();
        perms.set_mode(0o755);
        tokio::fs::set_permissions(cache_entry.path(), perms).await?;
    }

    Ok(cache_entry.into_path_buf())
}

/// Install a compatible version of uv based on version constraints.
pub async fn install_compatible_uv(
    constraints: &uv_pep440::VersionSpecifiers,
    client: &BaseClient,
    cache: &Cache,
    reporter: Option<&dyn Reporter>,
) -> Result<PathBuf, Error> {
    // Find a compatible version and its artifacts
    let (version, artifacts) = find_compatible_version(Binary::Uv, constraints, client).await?;

    // Install that version with artifact URLs
    install_binary_with_artifacts(Binary::Uv, &version, artifacts, client, cache, reporter).await
}

/// Cast platform types to the binary target triple format.
///
/// This performs some normalization to match cargo-dist's styling.
fn platform_name_for_binary(os: Os, arch: Arch, libc: Libc) -> String {
    use target_lexicon::{
        Architecture, ArmArchitecture, OperatingSystem, Riscv64Architecture, X86_32Architecture,
    };
    let arch_name = match arch.family() {
        // Special cases where Display doesn't match target triple
        Architecture::X86_32(X86_32Architecture::I686) => "i686".to_string(),
        Architecture::Riscv64(Riscv64Architecture::Riscv64) => "riscv64gc".to_string(),
        _ => arch.to_string(),
    };
    let vendor = match &*os {
        OperatingSystem::Darwin(_) => "apple",
        OperatingSystem::Windows => "pc",
        _ => "unknown",
    };
    let os_name = match &*os {
        OperatingSystem::Darwin(_) => "darwin",
        _ => &os.to_string(),
    };

    let abi = match (&*os, libc) {
        (OperatingSystem::Windows, _) => Some("msvc".to_string()),
        (OperatingSystem::Linux, Libc::Some(env)) => Some({
            // Special suffix for ARM with hardware float
            if matches!(arch.family(), Architecture::Arm(ArmArchitecture::Armv7)) {
                format!("{env}eabihf")
            } else {
                env.to_string()
            }
        }),
        _ => None,
    };

    format!(
        "{arch_name}-{vendor}-{os_name}{abi}",
        abi = abi.map(|abi| format!("-{abi}")).unwrap_or_default()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use uv_pep440::VersionSpecifiers;

    #[test]
    fn test_uv_binary_properties() {
        let uv_binary = Binary::Uv;

        assert_eq!(uv_binary.name(), "uv");
        assert_eq!(
            uv_binary.executable(),
            format!("uv{}", std::env::consts::EXE_SUFFIX)
        );

        let version = Version::from_str("0.5.0").unwrap();
        let platform = "x86_64-unknown-linux-gnu";
        let ext = SourceDistExtension::TarGz;

        let url = uv_binary.download_url(&version, platform, &ext).unwrap();
        assert!(url.as_str().contains("github.com/astral-sh/uv"));
        assert!(url.as_str().contains("releases/download/0.5.0"));
        assert!(url.as_str().contains("uv-x86_64-unknown-linux-gnu.tar.gz"));
    }

    #[test]
    fn test_ruff_binary_properties() {
        let ruff_binary = Binary::Ruff;

        assert_eq!(ruff_binary.name(), "ruff");
        assert_eq!(
            ruff_binary.executable(),
            format!("ruff{}", std::env::consts::EXE_SUFFIX)
        );

        let version = Version::from_str("0.12.5").unwrap();
        let platform = "x86_64-apple-darwin";
        let ext = SourceDistExtension::TarGz;

        let url = ruff_binary.download_url(&version, platform, &ext).unwrap();
        assert!(url.as_str().contains("github.com/astral-sh/ruff"));
        assert!(url.as_str().contains("releases/download/0.12.5"));
        assert!(url.as_str().contains("ruff-x86_64-apple-darwin.tar.gz"));
    }

    #[test]
    fn test_version_constraints_parsing() {
        // Test various version constraint formats that the function should handle
        let constraints = [">=0.5.0", "==0.5.0", ">=0.5.0,<1.0.0", "~=0.5.0"];

        for constraint_str in &constraints {
            let specifiers = VersionSpecifiers::from_str(constraint_str)
                .expect(&format!("Failed to parse: {}", constraint_str));

            // Should not panic when creating version constraints
            assert!(!specifiers.is_empty());
        }
    }
}
