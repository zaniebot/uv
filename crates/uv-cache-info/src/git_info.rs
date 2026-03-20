use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use tracing::warn;
use walkdir::WalkDir;

#[derive(Debug, thiserror::Error)]
pub(crate) enum GitInfoError {
    #[error("The repository at {0} is missing a `.git` directory")]
    MissingGitDir(PathBuf),
    #[error("The repository at {0} is missing a `HEAD` file")]
    MissingHead(PathBuf),
    #[error("The repository at {0} is missing a `refs` directory")]
    MissingRefs(PathBuf),
    #[error("The repository at {0} has an invalid reference: `{1}`")]
    InvalidRef(PathBuf, String),
    #[error("The discovered commit has an invalid length (expected 40 characters): `{0}`")]
    WrongLength(String),
    #[error("The discovered commit has an invalid character (expected hexadecimal): `{0}`")]
    WrongDigit(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// The current commit for a repository (i.e., a 40-character hexadecimal string).
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Commit(String);

fn discover_git_dir(path: &Path) -> Result<PathBuf, GitInfoError> {
    path.ancestors()
        .map(|ancestor| ancestor.join(".git"))
        .find(|git_dir| git_dir.exists())
        .ok_or_else(|| GitInfoError::MissingGitDir(path.to_path_buf()))
}

fn parse_commit(commit: String) -> Result<String, GitInfoError> {
    if commit.len() != 40 {
        return Err(GitInfoError::WrongLength(commit));
    }
    if commit
        .chars()
        .any(|character| !character.is_ascii_hexdigit())
    {
        return Err(GitInfoError::WrongDigit(commit));
    }
    Ok(commit)
}

impl Commit {
    /// Return the [`Commit`] for the repository at the given path.
    pub(crate) fn from_repository(path: &Path) -> Result<Self, GitInfoError> {
        let git_dir = discover_git_dir(path)?;

        let git_head_path =
            git_head(&git_dir).ok_or_else(|| GitInfoError::MissingHead(git_dir.clone()))?;
        let git_head_contents = fs_err::read_to_string(git_head_path)?;

        // The contents are either a commit or a reference in the following formats
        // - "<commit>" when the head is detached
        // - "ref <ref>" when working on a branch
        // If a commit, checking if the HEAD file has changed is sufficient
        // If a ref, we need to add the head file for that ref to rebuild on commit
        let mut git_ref_parts = git_head_contents.split_whitespace();
        let commit_or_ref = git_ref_parts
            .next()
            .ok_or_else(|| GitInfoError::InvalidRef(git_dir.clone(), git_head_contents.clone()))?;
        let commit = if let Some(git_ref) = git_ref_parts.next() {
            let git_ref_path = git_dir.join(git_ref);
            let commit = fs_err::read_to_string(git_ref_path)?;
            commit.trim().to_string()
        } else {
            commit_or_ref.to_string()
        };

        Ok(Self(parse_commit(commit)?))
    }
}

/// The set of tags visible in a repository.
#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Tags(BTreeMap<String, String>);

impl Tags {
    /// Return the [`Tags`] for the repository at the given path.
    pub(crate) fn from_repository(path: &Path) -> Result<Self, GitInfoError> {
        let git_dir = discover_git_dir(path)?;

        let git_tags_path = git_refs(&git_dir)
            .ok_or_else(|| GitInfoError::MissingRefs(git_dir.clone()))?
            .join("tags");

        let mut tags = BTreeMap::new();

        // Map each tag to its commit.
        for entry in WalkDir::new(&git_tags_path).contents_first(true) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    warn!("Failed to read Git tags: {err}");
                    continue;
                }
            };
            let path = entry.path();
            if !entry.file_type().is_file() {
                continue;
            }
            if let Ok(Some(tag)) = path.strip_prefix(&git_tags_path).map(|name| name.to_str()) {
                let commit = parse_commit(fs_err::read_to_string(path)?.trim().to_string())?;
                tags.insert(tag.to_string(), commit);
            }
        }

        Ok(Self(tags))
    }
}

/// Return the path to the `HEAD` file of a Git repository, taking worktrees into account.
fn git_head(git_dir: &Path) -> Option<PathBuf> {
    // The typical case is a standard git repository.
    let git_head_path = git_dir.join("HEAD");
    if git_head_path.exists() {
        return Some(git_head_path);
    }
    if !git_dir.is_file() {
        return None;
    }
    // If `.git/HEAD` doesn't exist and `.git` is actually a file,
    // then let's try to attempt to read it as a worktree. If it's
    // a worktree, then its contents will look like this, e.g.:
    //
    //     gitdir: /home/andrew/astral/uv/main/.git/worktrees/pr2
    //
    // And the HEAD file we want to watch will be at:
    //
    //     /home/andrew/astral/uv/main/.git/worktrees/pr2/HEAD
    let contents = fs_err::read_to_string(git_dir).ok()?;
    let (label, worktree_path) = contents.split_once(':')?;
    if label != "gitdir" {
        return None;
    }
    let worktree_path = worktree_path.trim();
    Some(PathBuf::from(worktree_path).join("HEAD"))
}

/// Return the path to the `refs` directory of a Git repository, taking worktrees into account.
fn git_refs(git_dir: &Path) -> Option<PathBuf> {
    // The typical case is a standard git repository.
    let git_head_path = git_dir.join("refs");
    if git_head_path.exists() {
        return Some(git_head_path);
    }
    if !git_dir.is_file() {
        return None;
    }
    // If `.git/refs` doesn't exist and `.git` is actually a file,
    // then let's try to attempt to read it as a worktree. If it's
    // a worktree, then its contents will look like this, e.g.:
    //
    //     gitdir: /home/andrew/astral/uv/main/.git/worktrees/pr2
    //
    // And the HEAD refs we want to watch will be at:
    //
    //     /home/andrew/astral/uv/main/.git/refs
    let contents = fs_err::read_to_string(git_dir).ok()?;
    let (label, worktree_path) = contents.split_once(':')?;
    if label != "gitdir" {
        return None;
    }
    let worktree_path = PathBuf::from(worktree_path.trim());
    let refs_path = worktree_path.parent()?.parent()?.join("refs");
    Some(refs_path)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;

    const COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";

    #[test]
    fn commit_and_tags_walk_up_to_the_repository_root() -> Result<()> {
        let temp_dir = tempdir()?;
        let repository = temp_dir.path().join("repo");
        let git_dir = repository.join(".git");
        let nested = repository.join("src/module");

        fs_err::create_dir_all(git_dir.join("refs/heads"))?;
        fs_err::create_dir_all(git_dir.join("refs/tags"))?;
        fs_err::create_dir_all(&nested)?;
        fs_err::write(git_dir.join("HEAD"), "ref: refs/heads/main\n")?;
        fs_err::write(git_dir.join("refs/heads/main"), format!("{COMMIT}\n"))?;
        fs_err::write(git_dir.join("refs/tags/v1.0.0"), format!("{COMMIT}\n"))?;

        assert_eq!(
            Commit::from_repository(&nested)?,
            Commit(COMMIT.to_string())
        );
        assert_eq!(
            Tags::from_repository(&nested)?,
            Tags(BTreeMap::from([(
                String::from("v1.0.0"),
                String::from(COMMIT)
            )]))
        );

        Ok(())
    }

    #[test]
    fn git_head_and_refs_support_worktree_git_files() -> Result<()> {
        let temp_dir = tempdir()?;
        let repository = temp_dir.path().join("repo");
        let main_git_dir = temp_dir.path().join("main/.git");
        let worktree_dir = main_git_dir.join("worktrees/feature");

        fs_err::create_dir_all(repository.join("src"))?;
        fs_err::create_dir_all(main_git_dir.join("refs"))?;
        fs_err::create_dir_all(&worktree_dir)?;
        fs_err::write(
            repository.join(".git"),
            format!("gitdir: {}\n", worktree_dir.display()),
        )?;
        fs_err::write(worktree_dir.join("HEAD"), "ref: refs/heads/feature\n")?;

        let git_dir = discover_git_dir(&repository.join("src"))?;
        assert_eq!(git_head(&git_dir), Some(worktree_dir.join("HEAD")));
        assert_eq!(git_refs(&git_dir), Some(main_git_dir.join("refs")));

        Ok(())
    }
}
