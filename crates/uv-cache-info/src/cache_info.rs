use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use tracing::{debug, info_span, warn};

use uv_fs::Simplified;

use crate::git_info::{Commit, Tags};
use crate::glob::cluster_globs;
use crate::timestamp::Timestamp;

#[derive(Debug, thiserror::Error)]
pub enum CacheInfoError {
    #[error("Failed to parse glob patterns for `cache-keys`: {0}")]
    Glob(#[from] globwalk::GlobError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// The information used to determine whether a built distribution is up-to-date, based on the
/// timestamps of relevant files, the current commit of a repository, etc.
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CacheInfo {
    /// The timestamp of the most recent `ctime` of any relevant files, at the time of the build.
    /// The timestamp will typically be the maximum of the `ctime` values of the `pyproject.toml`,
    /// `setup.py`, and `setup.cfg` files, if they exist; however, users can provide additional
    /// files to timestamp via the `cache-keys` field.
    timestamp: Option<Timestamp>,
    /// The commit at which the distribution was built.
    commit: Option<Commit>,
    /// The Git tags present at the time of the build.
    tags: Option<Tags>,
    /// Environment variables to include in the cache key.
    #[serde(default)]
    env: BTreeMap<String, Option<String>>,
    /// The timestamp or inode of any directories that should be considered in the cache key.
    #[serde(default)]
    directories: BTreeMap<Cow<'static, str>, Option<DirectoryTimestamp>>,
}

enum PathMetadata {
    Found(std::fs::Metadata),
    Missing,
}

fn default_cache_keys() -> Vec<CacheKey> {
    vec![
        CacheKey::Path(Cow::Borrowed("pyproject.toml")),
        CacheKey::Path(Cow::Borrowed("setup.py")),
        CacheKey::Path(Cow::Borrowed("setup.cfg")),
        CacheKey::Directory {
            dir: Cow::Borrowed("src"),
        },
    ]
}

fn read_path_metadata(path: &Path, kind: &str) -> Option<PathMetadata> {
    match path.metadata() {
        Ok(metadata) => Some(PathMetadata::Found(metadata)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Some(PathMetadata::Missing),
        Err(err) => {
            warn!("Failed to read metadata for {kind}: {err}");
            None
        }
    }
}

fn load_cache_keys(pyproject_path: &Path) -> Option<Vec<CacheKey>> {
    let contents = fs_err::read_to_string(pyproject_path).ok()?;
    let result = info_span!("toml::from_str cache keys", path = %pyproject_path.display())
        .in_scope(|| toml::from_str::<PyProjectToml>(&contents));
    result
        .ok()?
        .tool
        .and_then(|tool| tool.uv)
        .and_then(|tool_uv| tool_uv.cache_keys)
}

fn read_repository_commit(directory: &Path) -> Option<Commit> {
    match Commit::from_repository(directory) {
        Ok(commit_info) => Some(commit_info),
        Err(err) => {
            debug!("Failed to read the current commit: {err}");
            None
        }
    }
}

fn read_repository_tags(directory: &Path) -> Option<Tags> {
    match Tags::from_repository(directory) {
        Ok(tags_info) => Some(tags_info),
        Err(err) => {
            debug!("Failed to read the current tags: {err}");
            None
        }
    }
}

fn update_last_changed(
    last_changed: &mut Option<(PathBuf, Timestamp)>,
    path: PathBuf,
    metadata: &std::fs::Metadata,
) {
    let timestamp = Timestamp::from_metadata(metadata);
    if last_changed
        .as_ref()
        .is_none_or(|(_, previous_timestamp)| *previous_timestamp < timestamp)
    {
        *last_changed = Some((path, timestamp));
    }
}

fn read_directory_timestamp(
    _path: &Path,
    metadata: &std::fs::Metadata,
) -> Option<DirectoryTimestamp> {
    if let Ok(created) = metadata.created() {
        // Prefer the creation time.
        return Some(DirectoryTimestamp::Timestamp(Timestamp::from(created)));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Some(DirectoryTimestamp::Inode(metadata.ino()))
    }
    #[cfg(not(unix))]
    {
        warn!(
            "Failed to read creation time for directory: `{}`",
            _path.display()
        );
        None
    }
}

impl CacheInfo {
    /// Return the [`CacheInfo`] for a given timestamp.
    pub fn from_timestamp(timestamp: Timestamp) -> Self {
        Self {
            timestamp: Some(timestamp),
            ..Self::default()
        }
    }

    /// Compute the cache info for a given path, which may be a file or a directory.
    pub fn from_path(path: &Path) -> Result<Self, CacheInfoError> {
        let metadata = fs_err::metadata(path)?;
        if metadata.is_file() {
            Ok(Self::from_file(path)?)
        } else {
            Self::from_directory(path)
        }
    }

    /// Compute the cache info for a given directory.
    pub fn from_directory(directory: &Path) -> Result<Self, CacheInfoError> {
        let mut commit = None;
        let mut tags = None;
        let mut last_changed: Option<(PathBuf, Timestamp)> = None;
        let mut directories = BTreeMap::new();
        let mut env = BTreeMap::new();

        // Read the cache keys.
        let pyproject_path = directory.join("pyproject.toml");
        let cache_keys = load_cache_keys(&pyproject_path).unwrap_or_else(default_cache_keys);

        // Incorporate timestamps from any direct filepaths.
        let mut globs = vec![];
        for cache_key in cache_keys {
            match cache_key {
                CacheKey::Path(file) | CacheKey::File { file } => {
                    if file
                        .as_ref()
                        .chars()
                        .any(|c| matches!(c, '*' | '?' | '[' | '{'))
                    {
                        // Defer globs to a separate pass.
                        globs.push(file);
                        continue;
                    }

                    // Treat the path as a file.
                    let path = directory.join(file.as_ref());
                    let Some(PathMetadata::Found(metadata)) = read_path_metadata(&path, "file")
                    else {
                        continue;
                    };
                    if !metadata.is_file() {
                        warn!(
                            "Expected file for cache key, but found directory: `{}`",
                            path.display()
                        );
                        continue;
                    }
                    update_last_changed(&mut last_changed, path, &metadata);
                }
                CacheKey::Directory { dir } => {
                    // Treat the path as a directory.
                    let path = directory.join(dir.as_ref());
                    let metadata = match read_path_metadata(&path, "directory") {
                        Some(PathMetadata::Found(metadata)) => metadata,
                        Some(PathMetadata::Missing) => {
                            directories.insert(dir, None);
                            continue;
                        }
                        None => continue,
                    };
                    if !metadata.is_dir() {
                        warn!(
                            "Expected directory for cache key, but found file: `{}`",
                            path.display()
                        );
                        continue;
                    }

                    directories.insert(dir, read_directory_timestamp(&path, &metadata));
                }
                CacheKey::Git {
                    git: GitPattern::Bool(true),
                } => commit = read_repository_commit(directory),
                CacheKey::Git {
                    git: GitPattern::Set(set),
                } => {
                    if set.commit.unwrap_or(false) {
                        commit = read_repository_commit(directory);
                    }
                    if set.tags.unwrap_or(false) {
                        tags = read_repository_tags(directory);
                    }
                }
                CacheKey::Git {
                    git: GitPattern::Bool(false),
                } => {}
                CacheKey::Environment { env: var } => {
                    let value = std::env::var(&var).ok();
                    env.insert(var, value);
                }
            }
        }

        // If we have any globs, first cluster them using LCP and then do a single pass on each group.
        if !globs.is_empty() {
            for (glob_base, glob_patterns) in cluster_globs(&globs) {
                let walker = globwalk::GlobWalkerBuilder::from_patterns(
                    directory.join(glob_base),
                    &glob_patterns,
                )
                .file_type(globwalk::FileType::FILE | globwalk::FileType::SYMLINK)
                .build()?;
                for entry in walker {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => {
                            warn!("Failed to read glob entry: {err}");
                            continue;
                        }
                    };
                    let metadata = if entry.path_is_symlink() {
                        // resolve symlinks for leaf entries without following symlinks while globbing
                        match fs_err::metadata(entry.path()) {
                            Ok(metadata) => metadata,
                            Err(err) => {
                                warn!("Failed to resolve symlink for glob entry: {err}");
                                continue;
                            }
                        }
                    } else {
                        match entry.metadata() {
                            Ok(metadata) => metadata,
                            Err(err) => {
                                warn!("Failed to read metadata for glob entry: {err}");
                                continue;
                            }
                        }
                    };
                    if !metadata.is_file() {
                        if !entry.path_is_symlink() {
                            // don't warn if it was a symlink - it may legitimately resolve to a directory
                            warn!(
                                "Expected file for cache key, but found directory: `{}`",
                                entry.path().display()
                            );
                        }
                        continue;
                    }
                    let timestamp = Timestamp::from_metadata(&metadata);
                    if last_changed.as_ref().is_none_or(|(_, prev_timestamp)| {
                        *prev_timestamp < Timestamp::from_metadata(&metadata)
                    }) {
                        last_changed = Some((entry.into_path(), timestamp));
                    }
                }
            }
        }

        let timestamp = if let Some((path, timestamp)) = last_changed {
            debug!(
                "Computed cache info: {timestamp:?}, {commit:?}, {tags:?}, {env:?}, {directories:?}. Most recently modified: {}",
                path.user_display()
            );
            Some(timestamp)
        } else {
            None
        };

        Ok(Self {
            timestamp,
            commit,
            tags,
            env,
            directories,
        })
    }

    /// Compute the cache info for a given file, assumed to be a binary or source distribution
    /// represented as (e.g.) a `.whl` or `.tar.gz` archive.
    pub fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let metadata = fs_err::metadata(path.as_ref())?;
        let timestamp = Timestamp::from_metadata(&metadata);
        Ok(Self {
            timestamp: Some(timestamp),
            ..Self::default()
        })
    }

    /// Returns `true` if the cache info is empty.
    pub fn is_empty(&self) -> bool {
        self.timestamp.is_none()
            && self.commit.is_none()
            && self.tags.is_none()
            && self.env.is_empty()
            && self.directories.is_empty()
    }
}

/// A `pyproject.toml` with an (optional) `[tool.uv]` section.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PyProjectToml {
    tool: Option<Tool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Tool {
    uv: Option<ToolUv>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ToolUv {
    cache_keys: Option<Vec<CacheKey>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(untagged, rename_all = "kebab-case", deny_unknown_fields)]
pub enum CacheKey {
    /// Ex) `"Cargo.lock"` or `"**/*.toml"`
    Path(Cow<'static, str>),
    /// Ex) `{ file = "Cargo.lock" }` or `{ file = "**/*.toml" }`
    File { file: Cow<'static, str> },
    /// Ex) `{ dir = "src" }`
    Directory { dir: Cow<'static, str> },
    /// Ex) `{ git = true }` or `{ git = { commit = true, tags = false } }`
    Git { git: GitPattern },
    /// Ex) `{ env = "UV_CACHE_INFO" }`
    Environment { env: String },
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(untagged, rename_all = "kebab-case", deny_unknown_fields)]
pub enum GitPattern {
    Bool(bool),
    Set(GitSet),
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GitSet {
    commit: Option<bool>,
    tags: Option<bool>,
}

pub enum FilePattern {
    Glob(String),
    Path(PathBuf),
}

/// A timestamp used to measure changes to a directory.
#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "kebab-case", deny_unknown_fields)]
enum DirectoryTimestamp {
    Timestamp(Timestamp),
    Inode(u64),
}

#[cfg(all(test, unix))]
mod tests_unix {
    use anyhow::Result;

    use super::{CacheInfo, Timestamp};

    #[test]
    fn test_cache_info_symlink_resolve() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let dir = dir.path().join("dir");
        fs_err::create_dir_all(&dir)?;

        let write_manifest = |cache_key: &str| {
            fs_err::write(
                dir.join("pyproject.toml"),
                format!(
                    r#"
                [tool.uv]
                cache-keys = [
                    "{cache_key}"
                ]
                "#
                ),
            )
        };

        let touch = |path: &str| -> Result<_> {
            let path = dir.join(path);
            fs_err::create_dir_all(path.parent().unwrap())?;
            fs_err::write(&path, "")?;
            Ok(Timestamp::from_metadata(&path.metadata()?))
        };

        let cache_timestamp = || -> Result<_> { Ok(CacheInfo::from_directory(&dir)?.timestamp) };

        write_manifest("x/**")?;
        assert_eq!(cache_timestamp()?, None);
        let y = touch("x/y")?;
        assert_eq!(cache_timestamp()?, Some(y));
        let z = touch("x/z")?;
        assert_eq!(cache_timestamp()?, Some(z));

        // leaf entry symlink should be resolved
        let a = touch("../a")?;
        fs_err::os::unix::fs::symlink(dir.join("../a"), dir.join("x/a"))?;
        assert_eq!(cache_timestamp()?, Some(a));

        // symlink directories should not be followed while globbing
        let c = touch("../b/c")?;
        fs_err::os::unix::fs::symlink(dir.join("../b"), dir.join("x/b"))?;
        assert_eq!(cache_timestamp()?, Some(a));

        // no globs, should work as expected
        write_manifest("x/y")?;
        assert_eq!(cache_timestamp()?, Some(y));
        write_manifest("x/a")?;
        assert_eq!(cache_timestamp()?, Some(a));
        write_manifest("x/b/c")?;
        assert_eq!(cache_timestamp()?, Some(c));

        // symlink pointing to a directory
        write_manifest("x/*b*")?;
        assert_eq!(cache_timestamp()?, None);

        Ok(())
    }

    #[test]
    fn test_cache_info_git_keys() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let dir = dir.path().join("repo");
        let git_dir = dir.join(".git");

        fs_err::create_dir_all(git_dir.join("refs/heads"))?;
        fs_err::create_dir_all(git_dir.join("refs/tags"))?;
        fs_err::write(
            dir.join("pyproject.toml"),
            r#"
            [tool.uv]
            cache-keys = [{ git = { commit = true, tags = true } }]
            "#,
        )?;
        fs_err::write(git_dir.join("HEAD"), "ref: refs/heads/main\n")?;
        fs_err::write(
            git_dir.join("refs/heads/main"),
            "0123456789abcdef0123456789abcdef01234567\n",
        )?;
        fs_err::write(
            git_dir.join("refs/tags/v1.0.0"),
            "0123456789abcdef0123456789abcdef01234567\n",
        )?;

        let cache_info = CacheInfo::from_directory(&dir)?;
        assert!(cache_info.commit.is_some());
        assert!(cache_info.tags.is_some());

        Ok(())
    }
}
