use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use fs_err as fs;
use fs_err::DirEntry;
use reflink_copy as reflink;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use tempfile::tempdir_in;
use tracing::{debug, instrument, trace};
use walkdir::WalkDir;

use uv_distribution_filename::WheelFilename;
use uv_fs::Simplified;
use uv_preview::{Preview, PreviewFeatures};
use uv_warnings::{warn_user, warn_user_once};

use crate::Error;

#[allow(clippy::struct_field_names)]
#[derive(Debug, Default)]
pub struct Locks {
    /// The parent directory of a file in a synchronized copy
    copy_dir_locks: Mutex<FxHashMap<PathBuf, Arc<Mutex<()>>>>,
    /// Top level modules (excluding namespaces) we write to.
    modules: Mutex<FxHashMap<OsString, WheelFilename>>,
    /// Preview settings for feature flags.
    preview: Preview,
}

impl Locks {
    /// Create a new Locks instance with the given preview settings.
    pub fn new(preview: Preview) -> Self {
        Self {
            copy_dir_locks: Mutex::new(FxHashMap::default()),
            modules: Mutex::new(FxHashMap::default()),
            preview,
        }
    }

    /// Warn when a module exists in multiple packages.
    fn warn_module_conflict(&self, module: &OsStr, wheel_a: &WheelFilename) {
        if let Some(wheel_b) = self
            .modules
            .lock()
            .unwrap()
            .insert(module.to_os_string(), wheel_a.clone())
        {
            // Only warn if the preview feature is enabled
            if !self
                .preview
                .is_enabled(PreviewFeatures::DETECT_MODULE_CONFLICTS)
            {
                return;
            }

            // Sort for consistent output, at least with two packages
            let (wheel_a, wheel_b) = if wheel_b.name > wheel_a.name {
                (&wheel_b, wheel_a)
            } else {
                (wheel_a, &wheel_b)
            };
            warn_user!(
                "The module `{}` is provided by more than one package, \
                which causes an install race condition and can result in a broken module. \
                Consider removing your dependency on either `{}` ({}) or `{}` ({}).",
                module.simplified_display().green(),
                wheel_a.name.cyan(),
                format!("v{}", wheel_a.version).cyan(),
                wheel_b.name.cyan(),
                format!("v{}", wheel_b.version).cyan()
            );
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum LinkMode {
    /// Clone (i.e., copy-on-write) packages from the wheel into the `site-packages` directory.
    Clone,
    /// Copy packages from the wheel into the `site-packages` directory.
    Copy,
    /// Hard link packages from the wheel into the `site-packages` directory.
    Hardlink,
    /// Symbolically link packages from the wheel into the `site-packages` directory.
    Symlink,
}

impl Default for LinkMode {
    fn default() -> Self {
        if cfg!(any(target_os = "macos", target_os = "ios")) {
            Self::Clone
        } else {
            Self::Hardlink
        }
    }
}

impl LinkMode {
    /// Extract a wheel by linking all of its files into site packages.
    #[instrument(skip_all)]
    pub fn link_wheel_files(
        self,
        site_packages: impl AsRef<Path>,
        wheel: impl AsRef<Path>,
        locks: &Locks,
        filename: &WheelFilename,
    ) -> Result<usize, Error> {
        match self {
            Self::Clone => clone_wheel_files(site_packages, wheel, locks, filename),
            Self::Copy => copy_wheel_files(site_packages, wheel, locks, filename),
            Self::Hardlink => hardlink_wheel_files(site_packages, wheel, locks, filename),
            Self::Symlink => symlink_wheel_files(site_packages, wheel, locks, filename),
        }
    }

    /// Returns `true` if the link mode is [`LinkMode::Symlink`].
    pub fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink)
    }
}

/// Extract a wheel by cloning all of its files into site packages. The files will be cloned
/// via copy-on-write, which is similar to a hard link, but allows the files to be modified
/// independently (that is, the file is copied upon modification).
///
/// This method uses `clonefile` on macOS, and `reflink` on Linux. See [`clone_recursive`] for
/// details.
fn clone_wheel_files(
    site_packages: impl AsRef<Path>,
    wheel: impl AsRef<Path>,
    locks: &Locks,
    filename: &WheelFilename,
) -> Result<usize, Error> {
    let wheel = wheel.as_ref();
    let mut count = 0usize;
    let mut attempt = Attempt::default();

    for entry in fs::read_dir(wheel)? {
        let entry = entry?;
        if entry.path().join("__init__.py").is_file() {
            locks.warn_module_conflict(
                entry
                    .path()
                    .strip_prefix(wheel)
                    .expect("wheel path starts with wheel root")
                    .as_os_str(),
                filename,
            );
        }
        clone_recursive(site_packages.as_ref(), wheel, locks, &entry, &mut attempt)?;
        count += 1;
    }

    // The directory mtime is not updated when cloning and the mtime is used by CPython's
    // import mechanisms to determine if it should look for new packages in a directory.
    // Here, we force the mtime to be updated to ensure that packages are importable without
    // manual cache invalidation.
    //
    // <https://github.com/python/cpython/blob/8336cb2b6f428246803b02a4e97fce49d0bb1e09/Lib/importlib/_bootstrap_external.py#L1601>
    let now = SystemTime::now();

    // `File.set_modified` is not available in `fs_err` yet
    #[allow(clippy::disallowed_types)]
    match std::fs::File::open(site_packages.as_ref()) {
        Ok(dir) => {
            if let Err(err) = dir.set_modified(now) {
                debug!(
                    "Failed to update mtime for {}: {err}",
                    site_packages.as_ref().display()
                );
            }
        }
        Err(err) => debug!(
            "Failed to open {} to update mtime: {err}",
            site_packages.as_ref().display()
        ),
    }

    Ok(count)
}

// Hard linking / reflinking might not be supported but we (afaik) can't detect this ahead of time,
// so we'll try hard linking / reflinking the first file - if this succeeds we'll know later
// errors are not due to lack of os/fs support. If it fails, we'll switch to copying for the rest of the
// install.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Attempt {
    #[default]
    Initial,
    Subsequent,
    UseCopyFallback,
}

/// Check if a file is a macOS executable that benefits from hardlinking.
///
/// On macOS, code signature verification is cached per-inode (vnode). When using reflinks
/// (copy-on-write), each cloned file gets a new inode, requiring full signature re-validation
/// on first execution. This causes significant cold-start performance issues.
///
/// To avoid this, we use hardlinks for executable files (which share the same inode as the
/// cached original), allowing signature validation to be reused.
///
/// This includes:
/// - Dynamic libraries (*.dylib)
/// - Shared objects (*.so)
/// - Mach-O binaries (detected by magic bytes)
///
/// Files that are NOT considered executables (and should continue to use reflinks):
/// - Python source files (*.py)
/// - Bytecode files (*.pyc)
/// - Static libraries (*.a)
/// - Data files, configs, resources
#[cfg(target_os = "macos")]
fn is_macos_executable(path: &Path) -> bool {
    use std::io::Read;

    // Check by extension first (fast path)
    if let Some(ext) = path.extension() {
        if ext == "dylib" || ext == "so" {
            return true;
        }
        // Skip known non-executable extensions
        if ext == "py" || ext == "pyc" || ext == "pyo" || ext == "pyd" || ext == "a" {
            return false;
        }
    }

    // Peek at magic bytes to detect Mach-O binaries
    let Ok(mut file) = std::fs::File::open(path) else {
        return false;
    };

    let mut magic = [0u8; 4];
    if file.read_exact(&mut magic).is_err() {
        return false;
    }

    // Mach-O magic bytes (big and little endian variants)
    matches!(
        magic,
        [0xFE, 0xED, 0xFA, 0xCE]    // MH_MAGIC (32-bit big-endian)
            | [0xCE, 0xFA, 0xED, 0xFE] // MH_CIGAM (32-bit little-endian)
            | [0xFE, 0xED, 0xFA, 0xCF] // MH_MAGIC_64 (64-bit big-endian)
            | [0xCF, 0xFA, 0xED, 0xFE] // MH_CIGAM_64 (64-bit little-endian)
            | [0xCA, 0xFE, 0xBA, 0xBE] // FAT_MAGIC (universal big-endian)
            | [0xBE, 0xBA, 0xFE, 0xCA] // FAT_CIGAM (universal little-endian)
    )
}

/// Clone or hardlink a file from `from` to `to` on macOS.
///
/// For executable files (Mach-O binaries, .dylib, .so), we use hardlinks to preserve the
/// inode and benefit from cached code signature validation. For non-executable files,
/// we use reflinks (copy-on-write) for space efficiency.
///
/// Returns `Ok(true)` if the file was successfully linked/cloned, `Ok(false)` if we should
/// fall back to copy, or an error if the operation failed irrecoverably.
#[cfg(target_os = "macos")]
fn clone_or_hardlink_file(
    from: &Path,
    to: &Path,
    site_packages: &Path,
    locks: &Locks,
    attempt: &mut Attempt,
) -> Result<bool, Error> {
    // For executable files, prefer hardlinks to preserve inode for cached signature validation
    if is_macos_executable(from) {
        match fs::hard_link(from, to) {
            Ok(()) => {
                trace!("Hardlinked executable {} to {}", from.display(), to.display());
                return Ok(true);
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                // File already exists, try to overwrite via temp file
                let tempdir = tempdir_in(site_packages)?;
                let tempfile = tempdir.path().join(from.file_name().unwrap());
                if fs::hard_link(from, &tempfile).is_ok() {
                    fs::rename(&tempfile, to)?;
                    trace!("Hardlinked executable (overwrite) {} to {}", from.display(), to.display());
                    return Ok(true);
                }
                // Fall through to try reflink
            }
            Err(err) => {
                // Hardlink failed (e.g., cross-device), fall back to reflink
                debug!(
                    "Failed to hardlink executable `{}` to `{}` ({}), falling back to reflink",
                    from.display(),
                    to.display(),
                    err
                );
            }
        }
    }

    // For non-executable files or if hardlink failed, use reflink
    match attempt {
        Attempt::Initial | Attempt::Subsequent => {
            match reflink::reflink(from, to) {
                Ok(()) => Ok(true),
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                    // File already exists, try to overwrite via temp file
                    let tempdir = tempdir_in(site_packages)?;
                    let tempfile = tempdir.path().join(from.file_name().unwrap());
                    match reflink::reflink(from, &tempfile) {
                        Ok(()) => {
                            fs::rename(&tempfile, to)?;
                            Ok(true)
                        }
                        Err(_) if *attempt == Attempt::Initial => {
                            debug!(
                                "Failed to clone `{}` to temporary location `{}`, falling back to copy",
                                from.display(),
                                tempfile.display(),
                            );
                            *attempt = Attempt::UseCopyFallback;
                            synchronized_copy(from, to, locks)?;
                            Ok(true)
                        }
                        Err(err) => Err(Error::Reflink {
                            from: from.to_path_buf(),
                            to: to.to_path_buf(),
                            err,
                        }),
                    }
                }
                Err(_) if *attempt == Attempt::Initial => {
                    debug!(
                        "Failed to clone `{}` to `{}`, falling back to copy",
                        from.display(),
                        to.display()
                    );
                    *attempt = Attempt::UseCopyFallback;
                    Ok(false)
                }
                Err(err) => Err(Error::Reflink {
                    from: from.to_path_buf(),
                    to: to.to_path_buf(),
                    err,
                }),
            }
        }
        Attempt::UseCopyFallback => {
            synchronized_copy(from, to, locks)?;
            Ok(true)
        }
    }
}

/// Recursively clone the contents of `from` into `to`.
///
/// Note the behavior here is platform-dependent.
///
/// On macOS, directories can be recursively copied with a single `clonefile` call. So we only
/// need to iterate over the top-level of the directory, and copy each file or subdirectory
/// unless the subdirectory exists already in which case we'll need to recursively merge its
/// contents with the existing directory. For executable files (Mach-O binaries, .dylib, .so),
/// we use hardlinks instead of reflinks to preserve the inode and benefit from cached code
/// signature validation.
///
/// On Linux, we need to always reflink recursively, as `FICLONE` ioctl does not support
/// directories. Also note, that reflink is only supported on certain filesystems (btrfs, xfs,
/// ...), and only when it does not cross filesystem boundaries.
///
/// On Windows, we also always need to reflink recursively, as `FSCTL_DUPLICATE_EXTENTS_TO_FILE`
/// ioctl is not supported on directories. Also, it is only supported on certain filesystems
/// (ReFS, SMB, ...).
fn clone_recursive(
    site_packages: &Path,
    wheel: &Path,
    locks: &Locks,
    entry: &DirEntry,
    attempt: &mut Attempt,
) -> Result<(), Error> {
    // Determine the existing and destination paths.
    let from = entry.path();
    let to = site_packages.join(
        from.strip_prefix(wheel)
            .expect("wheel path starts with wheel root"),
    );

    trace!("Cloning {} to {}", from.display(), to.display());

    if (cfg!(windows) || cfg!(target_os = "linux")) && from.is_dir() {
        fs::create_dir_all(&to)?;
        for entry in fs::read_dir(from)? {
            clone_recursive(site_packages, wheel, locks, &entry?, attempt)?;
        }
        return Ok(());
    }

    // On macOS, use the optimized clone_or_hardlink_file for files
    #[cfg(target_os = "macos")]
    if !entry.file_type()?.is_dir() {
        let success = clone_or_hardlink_file(&from, &to, site_packages, locks, attempt)?;
        if !success {
            // Retry with copy fallback
            clone_recursive(site_packages, wheel, locks, entry, attempt)?;
        } else if *attempt == Attempt::UseCopyFallback {
            warn_user_once!(
                "Failed to clone files; falling back to full copy. This may lead to degraded performance.\n         If the cache and target directories are on different filesystems, reflinking may not be supported.\n         If this is intentional, set `export UV_LINK_MODE=copy` or use `--link-mode=copy` to suppress this warning."
            );
        }
        if *attempt == Attempt::Initial {
            *attempt = Attempt::Subsequent;
        }
        return Ok(());
    }

    match attempt {
        Attempt::Initial => {
            if let Err(err) = reflink::reflink(&from, &to) {
                if err.kind() == std::io::ErrorKind::AlreadyExists {
                    // If cloning or copying fails and the directory exists already, it must be
                    // merged recursively.
                    if entry.file_type()?.is_dir() {
                        for entry in fs::read_dir(from)? {
                            clone_recursive(site_packages, wheel, locks, &entry?, attempt)?;
                        }
                    } else {
                        // If file already exists, overwrite it.
                        let tempdir = tempdir_in(site_packages)?;
                        let tempfile = tempdir.path().join(from.file_name().unwrap());
                        if reflink::reflink(&from, &tempfile).is_ok() {
                            fs::rename(&tempfile, to)?;
                        } else {
                            debug!(
                                "Failed to clone `{}` to temporary location `{}`, attempting to copy files as a fallback",
                                from.display(),
                                tempfile.display(),
                            );
                            *attempt = Attempt::UseCopyFallback;
                            synchronized_copy(&from, &to, locks)?;
                        }
                    }
                } else {
                    debug!(
                        "Failed to clone `{}` to `{}`, attempting to copy files as a fallback",
                        from.display(),
                        to.display()
                    );
                    // Fallback to copying
                    *attempt = Attempt::UseCopyFallback;
                    clone_recursive(site_packages, wheel, locks, entry, attempt)?;
                }
            }
        }
        Attempt::Subsequent => {
            if let Err(err) = reflink::reflink(&from, &to) {
                if err.kind() == std::io::ErrorKind::AlreadyExists {
                    // If cloning/copying fails and the directory exists already, it must be merged recursively.
                    if entry.file_type()?.is_dir() {
                        for entry in fs::read_dir(from)? {
                            clone_recursive(site_packages, wheel, locks, &entry?, attempt)?;
                        }
                    } else {
                        // If file already exists, overwrite it.
                        let tempdir = tempdir_in(site_packages)?;
                        let tempfile = tempdir.path().join(from.file_name().unwrap());
                        reflink::reflink(&from, &tempfile)?;
                        fs::rename(&tempfile, to)?;
                    }
                } else {
                    return Err(Error::Reflink { from, to, err });
                }
            }
        }
        Attempt::UseCopyFallback => {
            if entry.file_type()?.is_dir() {
                fs::create_dir_all(&to)?;
                for entry in fs::read_dir(from)? {
                    clone_recursive(site_packages, wheel, locks, &entry?, attempt)?;
                }
            } else {
                synchronized_copy(&from, &to, locks)?;
            }
            warn_user_once!(
                "Failed to clone files; falling back to full copy. This may lead to degraded performance.\n         If the cache and target directories are on different filesystems, reflinking may not be supported.\n         If this is intentional, set `export UV_LINK_MODE=copy` or use `--link-mode=copy` to suppress this warning."
            );
        }
    }

    if *attempt == Attempt::Initial {
        *attempt = Attempt::Subsequent;
    }
    Ok(())
}

/// Extract a wheel by copying all of its files into site packages.
fn copy_wheel_files(
    site_packages: impl AsRef<Path>,
    wheel: impl AsRef<Path>,
    locks: &Locks,
    filename: &WheelFilename,
) -> Result<usize, Error> {
    let mut count = 0usize;

    // Walk over the directory.
    for entry in WalkDir::new(&wheel) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(&wheel).expect("walkdir starts with root");
        let out_path = site_packages.as_ref().join(relative);

        warn_module_conflict(locks, filename, relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&out_path)?;
            continue;
        }

        synchronized_copy(path, &out_path, locks)?;

        count += 1;
    }

    Ok(count)
}

/// Extract a wheel by hard-linking all of its files into site packages.
fn hardlink_wheel_files(
    site_packages: impl AsRef<Path>,
    wheel: impl AsRef<Path>,
    locks: &Locks,
    filename: &WheelFilename,
) -> Result<usize, Error> {
    let mut attempt = Attempt::default();
    let mut count = 0usize;

    // Walk over the directory.
    for entry in WalkDir::new(&wheel) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(&wheel).expect("walkdir starts with root");
        let out_path = site_packages.as_ref().join(relative);

        warn_module_conflict(locks, filename, relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&out_path)?;
            continue;
        }

        // The `RECORD` file is modified during installation, so we copy it instead of hard-linking.
        if path.ends_with("RECORD") {
            synchronized_copy(path, &out_path, locks)?;
            count += 1;
            continue;
        }

        // Fallback to copying if hardlinks aren't supported for this installation.
        match attempt {
            Attempt::Initial => {
                // Once https://github.com/rust-lang/rust/issues/86442 is stable, use that.
                attempt = Attempt::Subsequent;
                if let Err(err) = fs::hard_link(path, &out_path) {
                    // If the file already exists, remove it and try again.
                    if err.kind() == std::io::ErrorKind::AlreadyExists {
                        debug!(
                            "File already exists (initial attempt), overwriting: {}",
                            out_path.display()
                        );
                        // Removing and recreating would lead to race conditions.
                        let tempdir = tempdir_in(&site_packages)?;
                        let tempfile = tempdir.path().join(entry.file_name());
                        if fs::hard_link(path, &tempfile).is_ok() {
                            fs_err::rename(&tempfile, &out_path)?;
                        } else {
                            debug!(
                                "Failed to hardlink `{}` to `{}`, attempting to copy files as a fallback",
                                out_path.display(),
                                path.display()
                            );
                            synchronized_copy(path, &out_path, locks)?;
                            attempt = Attempt::UseCopyFallback;
                        }
                    } else {
                        debug!(
                            "Failed to hardlink `{}` to `{}`, attempting to copy files as a fallback",
                            out_path.display(),
                            path.display()
                        );
                        synchronized_copy(path, &out_path, locks)?;
                        attempt = Attempt::UseCopyFallback;
                    }
                }
            }
            Attempt::Subsequent => {
                if let Err(err) = fs::hard_link(path, &out_path) {
                    // If the file already exists, remove it and try again.
                    if err.kind() == std::io::ErrorKind::AlreadyExists {
                        debug!(
                            "File already exists (subsequent attempt), overwriting: {}",
                            out_path.display()
                        );
                        // Removing and recreating would lead to race conditions.
                        let tempdir = tempdir_in(&site_packages)?;
                        let tempfile = tempdir.path().join(entry.file_name());
                        fs::hard_link(path, &tempfile)?;
                        fs_err::rename(&tempfile, &out_path)?;
                    } else {
                        return Err(err.into());
                    }
                }
            }
            Attempt::UseCopyFallback => {
                synchronized_copy(path, &out_path, locks)?;
                warn_user_once!(
                    "Failed to hardlink files; falling back to full copy. This may lead to degraded performance.\n         If the cache and target directories are on different filesystems, hardlinking may not be supported.\n         If this is intentional, set `export UV_LINK_MODE=copy` or use `--link-mode=copy` to suppress this warning."
                );
            }
        }

        count += 1;
    }

    Ok(count)
}

/// Extract a wheel by symbolically-linking all of its files into site packages.
fn symlink_wheel_files(
    site_packages: impl AsRef<Path>,
    wheel: impl AsRef<Path>,
    locks: &Locks,
    filename: &WheelFilename,
) -> Result<usize, Error> {
    let mut attempt = Attempt::default();
    let mut count = 0usize;

    // Walk over the directory.
    for entry in WalkDir::new(&wheel) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(&wheel).unwrap();
        let out_path = site_packages.as_ref().join(relative);

        warn_module_conflict(locks, filename, relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&out_path)?;
            continue;
        }

        // The `RECORD` file is modified during installation, so we copy it instead of symlinking.
        if path.ends_with("RECORD") {
            synchronized_copy(path, &out_path, locks)?;
            count += 1;
            continue;
        }

        // Fallback to copying if symlinks aren't supported for this installation.
        match attempt {
            Attempt::Initial => {
                // Once https://github.com/rust-lang/rust/issues/86442 is stable, use that.
                attempt = Attempt::Subsequent;
                if let Err(err) = create_symlink(path, &out_path) {
                    // If the file already exists, remove it and try again.
                    if err.kind() == std::io::ErrorKind::AlreadyExists {
                        debug!(
                            "File already exists (initial attempt), overwriting: {}",
                            out_path.display()
                        );
                        // Removing and recreating would lead to race conditions.
                        let tempdir = tempdir_in(&site_packages)?;
                        let tempfile = tempdir.path().join(entry.file_name());
                        if create_symlink(path, &tempfile).is_ok() {
                            fs::rename(&tempfile, &out_path)?;
                        } else {
                            debug!(
                                "Failed to symlink `{}` to `{}`, attempting to copy files as a fallback",
                                out_path.display(),
                                path.display()
                            );
                            synchronized_copy(path, &out_path, locks)?;
                            attempt = Attempt::UseCopyFallback;
                        }
                    } else {
                        debug!(
                            "Failed to symlink `{}` to `{}`, attempting to copy files as a fallback",
                            out_path.display(),
                            path.display()
                        );
                        synchronized_copy(path, &out_path, locks)?;
                        attempt = Attempt::UseCopyFallback;
                    }
                }
            }
            Attempt::Subsequent => {
                if let Err(err) = create_symlink(path, &out_path) {
                    // If the file already exists, remove it and try again.
                    if err.kind() == std::io::ErrorKind::AlreadyExists {
                        debug!(
                            "File already exists (subsequent attempt), overwriting: {}",
                            out_path.display()
                        );
                        // Removing and recreating would lead to race conditions.
                        let tempdir = tempdir_in(&site_packages)?;
                        let tempfile = tempdir.path().join(entry.file_name());
                        create_symlink(path, &tempfile)?;
                        fs::rename(&tempfile, &out_path)?;
                    } else {
                        return Err(err.into());
                    }
                }
            }
            Attempt::UseCopyFallback => {
                synchronized_copy(path, &out_path, locks)?;
                warn_user_once!(
                    "Failed to symlink files; falling back to full copy. This may lead to degraded performance.\n         If the cache and target directories are on different filesystems, symlinking may not be supported.\n         If this is intentional, set `export UV_LINK_MODE=copy` or use `--link-mode=copy` to suppress this warning."
                );
            }
        }

        count += 1;
    }

    Ok(count)
}

/// Copy from `from` to `to`, ensuring that the parent directory is locked. Avoids simultaneous
/// writes to the same file, which can lead to corruption.
///
/// See: <https://github.com/astral-sh/uv/issues/4831>
fn synchronized_copy(from: &Path, to: &Path, locks: &Locks) -> std::io::Result<()> {
    // Ensure we have a lock for the directory.
    let dir_lock = {
        let mut locks_guard = locks.copy_dir_locks.lock().unwrap();
        locks_guard
            .entry(to.parent().unwrap().to_path_buf())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    };

    // Acquire a lock on the directory.
    let _dir_guard = dir_lock.lock().unwrap();

    // Copy the file, which will also set its permissions.
    fs::copy(from, to)?;

    Ok(())
}

/// Warn when a module exists in multiple packages.
fn warn_module_conflict(locks: &Locks, filename: &WheelFilename, relative: &Path) {
    // Check for `__init__.py` to account for namespace packages.
    // TODO(konsti): We need to warn for overlapping namespace packages, too.
    if relative.components().count() == 2
        && relative.components().next_back().unwrap().as_os_str() == "__init__.py"
    {
        // Modules must be UTF-8, but we can skip the conversion using OsStr.
        locks.warn_module_conflict(relative.components().next().unwrap().as_os_str(), filename);
    }
}

#[cfg(unix)]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> std::io::Result<()> {
    fs_err::os::unix::fs::symlink(original, link)
}

#[cfg(windows)]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> std::io::Result<()> {
    if original.as_ref().is_dir() {
        fs_err::os::windows::fs::symlink_dir(original, link)
    } else {
        fs_err::os::windows::fs::symlink_file(original, link)
    }
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    /// Create a test file with the given content
    fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.path().join(name);
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }

    #[test]
    fn test_is_macos_executable_by_extension() {
        let dir = TempDir::new().unwrap();

        // .dylib files should be detected as executables
        let dylib = create_test_file(&dir, "lib.dylib", b"not real content");
        assert!(is_macos_executable(&dylib));

        // .so files should be detected as executables
        let so = create_test_file(&dir, "lib.so", b"not real content");
        assert!(is_macos_executable(&so));

        // .py files should NOT be detected as executables
        let py = create_test_file(&dir, "script.py", b"print('hello')");
        assert!(!is_macos_executable(&py));

        // .pyc files should NOT be detected as executables
        let pyc = create_test_file(&dir, "script.pyc", b"\x00\x00\x00\x00");
        assert!(!is_macos_executable(&pyc));

        // .a files (static libraries) should NOT be detected as executables
        let a = create_test_file(&dir, "lib.a", b"not real content");
        assert!(!is_macos_executable(&a));
    }

    #[test]
    fn test_is_macos_executable_by_magic_bytes() {
        let dir = TempDir::new().unwrap();

        // 64-bit Mach-O little-endian (most common on modern macOS)
        let macho64_le = create_test_file(&dir, "binary64_le", b"\xCF\xFA\xED\xFE");
        assert!(is_macos_executable(&macho64_le));

        // 64-bit Mach-O big-endian
        let macho64_be = create_test_file(&dir, "binary64_be", b"\xFE\xED\xFA\xCF");
        assert!(is_macos_executable(&macho64_be));

        // 32-bit Mach-O little-endian
        let macho32_le = create_test_file(&dir, "binary32_le", b"\xCE\xFA\xED\xFE");
        assert!(is_macos_executable(&macho32_le));

        // 32-bit Mach-O big-endian
        let macho32_be = create_test_file(&dir, "binary32_be", b"\xFE\xED\xFA\xCE");
        assert!(is_macos_executable(&macho32_be));

        // Universal (fat) binary big-endian
        let fat_be = create_test_file(&dir, "universal_be", b"\xCA\xFE\xBA\xBE");
        assert!(is_macos_executable(&fat_be));

        // Universal (fat) binary little-endian
        let fat_le = create_test_file(&dir, "universal_le", b"\xBE\xBA\xFE\xCA");
        assert!(is_macos_executable(&fat_le));

        // Random data should NOT be detected as executable
        let random = create_test_file(&dir, "random", b"\x00\x01\x02\x03more data");
        assert!(!is_macos_executable(&random));

        // Empty file should NOT be detected as executable
        let empty = create_test_file(&dir, "empty", b"");
        assert!(!is_macos_executable(&empty));

        // File too short should NOT be detected as executable
        let short = create_test_file(&dir, "short", b"\xCF\xFA");
        assert!(!is_macos_executable(&short));
    }

    #[test]
    fn test_is_macos_executable_extensionless_binary() {
        let dir = TempDir::new().unwrap();

        // A binary without extension (like executables in bin/)
        // Should be detected by magic bytes
        let binary = create_test_file(&dir, "python3", b"\xCF\xFA\xED\xFErest of binary");
        assert!(is_macos_executable(&binary));

        // A text file without extension should NOT be detected
        let text = create_test_file(&dir, "README", b"This is a readme file");
        assert!(!is_macos_executable(&text));
    }

    #[test]
    fn test_is_macos_executable_nonexistent_file() {
        // Non-existent file should return false, not error
        let path = PathBuf::from("/nonexistent/path/to/file");
        assert!(!is_macos_executable(&path));
    }
}
