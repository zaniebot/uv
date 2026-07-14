use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::vendor::CloneableSeekableReader;
use crate::{CompressionMethod, Error, insecure_no_validate, validate_archive_member_name};
use async_zip::base::read::seek::ZipFileReader;
use async_zip::error::ZipError;
use futures::executor::block_on;
use futures::io::{AllowStdIo, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use tracing::warn;
use uv_configuration::initialize_rayon_once;
use uv_warnings::warn_user_once;

/// Unzip a `.zip` archive into the target directory.
///
/// Returns the list of unpacked files and their sizes.
pub fn unzip(reader: fs_err::File, target: &Path) -> Result<Vec<(PathBuf, u64)>, Error> {
    let (reader, filename) = reader.into_parts();

    // Parse the central directory once, then clone the archive reader per Rayon worker so
    // extraction stays parallel for already-downloaded wheels.
    let archive = block_on(ZipFileReader::new(AllowStdIo::new(
        CloneableSeekableReader::new(reader),
    )))?;
    let directories = Mutex::new(FxHashSet::default());
    let skip_validation = insecure_no_validate();

    if !skip_validation {
        // Reject comments that appear to contain an embedded ZIP file, as in the streaming
        // extractor.
        let comment = archive.file().comment().as_bytes();
        if comment.iter().any(|&byte| (1..=8).contains(&byte)) {
            return Err(Error::ZipInZip);
        }

        // The seekable reader searches backwards for the end-of-central-directory record and
        // otherwise ignores bytes following it. Find the first structurally valid record so an
        // appended central-directory chain cannot hide trailing contents, while ignoring record
        // signatures that happen to occur in a comment.
        let mut scan_archive = archive.clone();
        let mut validation_archive = archive.clone();
        block_on(async {
            const EOCD_SIGNATURE: &[u8; 4] = b"PK\x05\x06";
            const EOCD_LENGTH: usize = 22;
            const CHUNK_LENGTH: usize = 64 * 1024;

            fn u16_at(bytes: &[u8], offset: usize) -> u16 {
                u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
            }

            fn u32_at(bytes: &[u8], offset: usize) -> u32 {
                u32::from_le_bytes([
                    bytes[offset],
                    bytes[offset + 1],
                    bytes[offset + 2],
                    bytes[offset + 3],
                ])
            }

            fn u64_at(bytes: &[u8], offset: usize) -> u64 {
                u64::from_le_bytes([
                    bytes[offset],
                    bytes[offset + 1],
                    bytes[offset + 2],
                    bytes[offset + 3],
                    bytes[offset + 4],
                    bytes[offset + 5],
                    bytes[offset + 6],
                    bytes[offset + 7],
                ])
            }

            let scan_reader = scan_archive.inner_mut();
            let validation_reader = validation_archive.inner_mut();
            let length = scan_reader
                .seek(SeekFrom::End(0))
                .await
                .map_err(Error::Io)?;
            scan_reader
                .seek(SeekFrom::Start(0))
                .await
                .map_err(Error::Io)?;

            // Search the complete archive in bounded chunks. Limiting this search to the
            // maximum comment length lets padding hide an earlier, independently valid EOCD.
            let mut buffer = vec![0; CHUNK_LENGTH + EOCD_LENGTH - 1];
            let mut carry = 0;
            let mut start = 0_u64;
            let mut record = None;
            'scan: loop {
                let read = scan_reader
                    .read(&mut buffer[carry..])
                    .await
                    .map_err(Error::Io)?;
                let available = carry + read;

                for offset in 0..available.saturating_sub(EOCD_LENGTH - 1) {
                    let candidate = &buffer[offset..offset + EOCD_LENGTH];
                    if &candidate[..EOCD_SIGNATURE.len()] != EOCD_SIGNATURE {
                        continue;
                    }

                    let Some(record_offset) = u64::try_from(offset)
                        .ok()
                        .and_then(|offset| start.checked_add(offset))
                    else {
                        continue;
                    };
                    let central_directory_size = u32_at(candidate, 12);
                    let central_directory_offset = u32_at(candidate, 16);

                    let standard_directory_end = u64::from(central_directory_offset)
                        .checked_add(u64::from(central_directory_size));
                    let valid = if standard_directory_end == Some(record_offset) {
                        true
                    } else {
                        const ZIP64_EOCD_SIGNATURE: &[u8; 4] = b"PK\x06\x06";
                        const ZIP64_LOCATOR_SIGNATURE: &[u8; 4] = b"PK\x06\x07";

                        let Some(locator_offset) = record_offset.checked_sub(20) else {
                            continue;
                        };
                        validation_reader
                            .seek(SeekFrom::Start(locator_offset))
                            .await
                            .map_err(Error::Io)?;
                        let mut locator = [0; 20];
                        validation_reader
                            .read_exact(&mut locator)
                            .await
                            .map_err(Error::Io)?;
                        if &locator[..ZIP64_LOCATOR_SIGNATURE.len()] != ZIP64_LOCATOR_SIGNATURE {
                            continue;
                        }

                        let zip64_offset = u64_at(&locator, 8);
                        if zip64_offset
                            .checked_add(56)
                            .is_none_or(|record_end| record_end > length)
                        {
                            continue;
                        }
                        validation_reader
                            .seek(SeekFrom::Start(zip64_offset))
                            .await
                            .map_err(Error::Io)?;
                        let mut zip64_record = [0; 56];
                        validation_reader
                            .read_exact(&mut zip64_record)
                            .await
                            .map_err(Error::Io)?;
                        if &zip64_record[..ZIP64_EOCD_SIGNATURE.len()] != ZIP64_EOCD_SIGNATURE {
                            continue;
                        }

                        let zip64_size = u64_at(&zip64_record, 4);
                        let directory_size = u64_at(&zip64_record, 40);
                        let directory_offset = u64_at(&zip64_record, 48);

                        directory_offset.checked_add(directory_size) == Some(zip64_offset)
                            && zip64_offset
                                .checked_add(12)
                                .and_then(|offset| offset.checked_add(zip64_size))
                                == Some(locator_offset)
                    };
                    if !valid {
                        continue;
                    }

                    let comment_length = u16_at(candidate, 20);
                    let Some(end) = record_offset
                        .checked_add(22)
                        .and_then(|offset| offset.checked_add(u64::from(comment_length)))
                    else {
                        continue;
                    };
                    if end <= length {
                        record = Some((record_offset, end));
                        break 'scan;
                    }
                }

                if read == 0 {
                    break;
                }
                carry = available.min(EOCD_LENGTH - 1);
                buffer.copy_within(available - carry..available, 0);
                let advanced = u64::try_from(available - carry)
                    .map_err(|_| ZipError::InvalidEntryDataRange)?;
                start = start
                    .checked_add(advanced)
                    .ok_or(ZipError::InvalidEntryDataRange)?;
            }

            let Some((record_offset, end)) = record else {
                return Err(Error::TrailingContents);
            };
            let comment_length = usize::try_from(end - record_offset - 22)
                .map_err(|_| ZipError::InvalidEntryDataRange)?;
            let mut comment = vec![0; comment_length];
            validation_reader
                .seek(SeekFrom::Start(record_offset + 22))
                .await
                .map_err(Error::Io)?;
            validation_reader
                .read_exact(&mut comment)
                .await
                .map_err(Error::Io)?;
            if comment.iter().any(|&byte| (1..=8).contains(&byte)) {
                return Err(Error::ZipInZip);
            }

            validation_reader
                .seek(SeekFrom::Start(end))
                .await
                .map_err(Error::Io)?;
            let mut has_trailing = false;
            loop {
                let read = validation_reader
                    .read(&mut buffer[..CHUNK_LENGTH])
                    .await
                    .map_err(Error::Io)?;
                if read == 0 {
                    break;
                }
                if buffer[..read].iter().any(|&byte| byte != 0) {
                    return Err(Error::TrailingContents);
                }
                has_trailing = true;
            }
            if has_trailing {
                warn!("Ignoring trailing null bytes in ZIP archive");
            }

            Ok::<(), Error>(())
        })?;
    }
    // Initialize the threadpool with the user settings.
    initialize_rayon_once();
    (0..archive.file().entries().len())
        .into_par_iter()
        .map(|file_number| {
            let mut archive = archive.clone();
            let entry = archive.file().entries()[file_number].clone();
            let file_name = match entry.filename().as_str() {
                Ok(file_name) => file_name,
                Err(ZipError::StringNotUtf8) => {
                    return Err(Error::CentralDirectoryEntryNotUtf8 {
                        index: file_number as u64,
                    });
                }
                Err(err) => return Err(err.into()),
            };

            let compression = CompressionMethod::from(entry.compression());
            if !compression.is_well_known() {
                warn_user_once!(
                    "One or more file entries in '{filename}' use the '{compression}' compression method, which is not widely supported. A future version of uv will reject ZIP archives containing entries compressed with this method. Entries must be compressed with the '{stored}', '{deflate}', or '{zstd}' compression methods.",
                    filename = filename.display(),
                    stored = CompressionMethod::Stored,
                    deflate = CompressionMethod::Deflated,
                    zstd = CompressionMethod::Zstd,
                );
            }

            if let Err(e) = validate_archive_member_name(file_name) {
                if !skip_validation {
                    return Err(e);
                }
            }

            // Determine the path of the file within the wheel.
            let Some(enclosed_name) = crate::stream::enclosed_name(file_name) else {
                warn!("Skipping unsafe file name: {file_name}");
                return Ok(None);
            };

            // The CRC in the local file header or data descriptor must agree with the
            // central-directory record. The seekable ZIP reader validates names and sizes, but
            // does not validate this field.
            if !skip_validation {
                let local_crc32 = block_on(async {
                    let reader = archive.inner_mut();
                    if entry.data_descriptor() {
                        reader
                            .seek(SeekFrom::Start(
                                entry
                                    .file_offset()
                                    .checked_add(26)
                                    .ok_or(ZipError::InvalidEntryDataRange)?,
                            ))
                            .await
                            .map_err(Error::Io)?;
                        let mut lengths = [0; 4];
                        reader.read_exact(&mut lengths).await.map_err(Error::Io)?;
                        let filename_length = u16::from_le_bytes([lengths[0], lengths[1]]);
                        let extra_length = u16::from_le_bytes([lengths[2], lengths[3]]);
                        let descriptor_offset = entry
                            .file_offset()
                            .checked_add(30)
                            .and_then(|offset| offset.checked_add(u64::from(filename_length)))
                            .and_then(|offset| offset.checked_add(u64::from(extra_length)))
                            .and_then(|offset| offset.checked_add(entry.compressed_size()))
                            .ok_or(ZipError::InvalidEntryDataRange)?;
                        reader
                            .seek(SeekFrom::Start(descriptor_offset))
                            .await
                            .map_err(Error::Io)?;
                        let mut checksum = [0; 4];
                        reader.read_exact(&mut checksum).await.map_err(Error::Io)?;
                        if checksum == *b"PK\x07\x08" {
                            reader.read_exact(&mut checksum).await.map_err(Error::Io)?;
                        }
                        Ok::<_, Error>(u32::from_le_bytes(checksum))
                    } else {
                        reader
                            .seek(SeekFrom::Start(
                                entry
                                    .file_offset()
                                    .checked_add(14)
                                    .ok_or(ZipError::InvalidEntryDataRange)?,
                            ))
                            .await
                            .map_err(Error::Io)?;
                        let mut checksum = [0; 4];
                        reader.read_exact(&mut checksum).await.map_err(Error::Io)?;
                        Ok::<_, Error>(u32::from_le_bytes(checksum))
                    }
                })?;
                if local_crc32 != entry.crc32() {
                    return Err(Error::ConflictingChecksums {
                        path: enclosed_name.clone(),
                        offset: entry.file_offset(),
                        local_crc32,
                        central_directory_crc32: entry.crc32(),
                    });
                }
            }

            // Create necessary parent directories.
            let path = target.join(&enclosed_name);
            if entry.dir()? {
                let mut directories = directories.lock().unwrap();
                if directories.insert(path.clone()) {
                    fs_err::create_dir_all(path).map_err(Error::Io)?;
                }
                return Ok(None);
            }

            if let Some(parent) = path.parent() {
                let mut directories = directories.lock().unwrap();
                if directories.insert(parent.to_path_buf()) {
                    fs_err::create_dir_all(parent).map_err(Error::Io)?;
                }
            }

            // Copy the file contents.
            let outfile = fs_err::File::create(&path).map_err(Error::Io)?;
            let size = entry.uncompressed_size();
            let writer = if let Ok(size) = usize::try_from(size) {
                std::io::BufWriter::with_capacity(std::cmp::min(size, 1024 * 1024), outfile)
            } else {
                std::io::BufWriter::new(outfile)
            };
            let (copied, computed_crc32) = block_on(async {
                let mut file = archive.reader_with_entry(file_number).await?;
                let mut writer = AllowStdIo::new(writer);
                let mut copied = 0;
                let mut buffer = vec![0; 128 * 1024];
                loop {
                    let read = file
                        .read(&mut buffer)
                        .await
                        .map_err(Error::io_or_compression)?;
                    if read == 0 {
                        break;
                    }
                    writer.write_all(&buffer[..read]).await.map_err(Error::Io)?;
                    copied += read as u64;
                }
                writer.flush().await.map_err(Error::Io)?;
                Ok::<_, Error>((copied, file.compute_hash()))
            })?;

            if copied != size && !skip_validation {
                return Err(Error::BadUncompressedSize {
                    path: enclosed_name.clone(),
                    computed: copied,
                    expected: size,
                });
            }

            if computed_crc32 != entry.crc32() && !skip_validation {
                return Err(Error::BadCrc32 {
                    path: enclosed_name.clone(),
                    computed: computed_crc32,
                    expected: entry.crc32(),
                });
            }

            // See `uv_extract::stream::unzip`. For simplicity, this is identical with the code there except for being
            // sync.
            #[cfg(unix)]
            {
                use std::fs::Permissions;
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = entry.unix_permissions() {
                    // https://github.com/pypa/pip/blob/3898741e29b7279e7bffe044ecfbe20f6a438b1e/src/pip/_internal/utils/unpacking.py#L88-L100
                    let has_any_executable_bit = mode & 0o111;
                    if has_any_executable_bit != 0 {
                        let permissions = fs_err::metadata(&path).map_err(Error::Io)?.permissions();
                        if permissions.mode() & 0o111 != 0o111 {
                            fs_err::set_permissions(
                                &path,
                                Permissions::from_mode(permissions.mode() | 0o111),
                            )
                            .map_err(Error::Io)?;
                        }
                    }
                }
            }

            Ok(Some((enclosed_name, size)))
        })
        // Filter out directories and skipped dangerous paths, we only want to collect the files.
        .filter_map(Result::transpose)
        .collect::<Result<_, Error>>()
}

/// Extract the top-level directory from an unpacked archive.
///
/// The specification says:
/// > A .tar.gz source distribution (sdist) contains a single top-level directory called
/// > `{name}-{version}` (e.g. foo-1.0), containing the source files of the package.
///
/// This function returns the path to that top-level directory.
pub fn strip_component(source: impl AsRef<Path>) -> Result<PathBuf, Error> {
    // TODO(konstin): Verify the name of the directory.
    let top_level = fs_err::read_dir(source.as_ref())
        .map_err(Error::Io)?
        .collect::<std::io::Result<Vec<fs_err::DirEntry>>>()
        .map_err(Error::Io)?;
    match top_level.as_slice() {
        [root] => Ok(root.path()),
        [] => Err(Error::EmptyArchive),
        _ => Err(Error::NonSingularArchive(
            top_level
                .into_iter()
                .map(|entry| entry.file_name())
                .collect(),
        )),
    }
}
