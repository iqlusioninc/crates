//! Tarball builder for release archives (to be passed to rpmbuild)
//!
//! Arguably we should be building a cpio archive instead of a tarball, but
//! Rust support for tar is presently (as of writing) better.

use failure::{self, Error};
use flate2::{write::GzEncoder, Compression};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use tar::{Builder, Header};

use config::{FileConfig, PackageConfig};

/// Default user that owns files in the archive
const DEFAULT_USERNAME: &str = "root";

/// Default group that owns files in the archive
const DEFAULT_GROUPNAME: &str = "root";

/// Default permissions for target binaries
const DEFAULT_TARGET_MODE: u32 = 0o755;

/// Default permissions for other files in the archive
const DEFAULT_FILE_MODE: u32 = 0o644;

/// Files within the release archive
#[derive(Debug)]
pub struct ArchiveFile {
    /// Path to source file on the local filesystem
    src_path: PathBuf,

    /// Path to use in the resulting archive (absolute)
    archive_path: PathBuf,

    /// User that owns the given file
    pub username: String,

    /// Group that owns the given file
    pub groupname: String,

    /// Mode of the file
    pub mode: u32,
}

impl ArchiveFile {
    /// Create a new archive file with the given config
    pub fn new(
        src_path: &Path,
        base_dir: &Path,
        file_config: &FileConfig,
        default_mode: u32,
    ) -> Result<Self, Error> {
        let archive_path = base_dir.join(file_config.path.strip_prefix("/")?);

        let username = match file_config.username {
            Some(ref u) => u.clone(),
            None => DEFAULT_USERNAME.to_owned(),
        };

        let groupname = match file_config.groupname {
            Some(ref g) => g.clone(),
            None => DEFAULT_GROUPNAME.to_owned(),
        };

        let mode = match file_config.mode {
            Some(ref m) => u32::from_str_radix(m, 8)?,
            None => default_mode,
        };

        Ok(Self {
            src_path: src_path.to_owned(),
            archive_path,
            username,
            groupname,
            mode,
        })
    }

    /// Append this file to the given archive builder
    pub fn append_to(&self, builder: &mut Builder<GzEncoder<File>>) -> Result<(), Error> {
        let mut header = Header::new_gnu();
        header.set_path(&self.archive_path)?;

        let src_file = File::open(&self.src_path)?;
        let src_metadata = src_file.metadata()?;
        header.set_size(src_metadata.len());
        header.set_mtime(
            src_metadata
                .modified()?
                .duration_since(UNIX_EPOCH)?
                .as_secs(),
        );
        header.set_username(&self.username)?;
        header.set_groupname(&self.groupname)?;
        header.set_mode(self.mode);
        header.set_cksum();

        builder.append(&header, src_file)?;
        Ok(())
    }
}

/// Tarball builder for Rust RPMs
#[derive(Debug)]
pub struct Archive {
    /// Files to include in the archive
    files: Vec<ArchiveFile>,
}

impl Archive {
    /// Process the package config and prepare to build the archive
    pub fn new(
        config: &PackageConfig,
        rpm_config_dir: &Path,
        target_dir: &Path,
    ) -> Result<Self, Error> {
        let base_dir = PathBuf::from(format!("{}-{}", config.name, config.version));
        let rpm_metadata = config
            .rpm_metadata()
            .ok_or_else(|| failure::err_msg("no [package.metadata.rpm] in Cargo.toml!"))?;

        let mut archive_files: Vec<ArchiveFile> = rpm_metadata
            .targets
            .iter()
            .map(|(name, config)| {
                ArchiveFile::new(
                    &target_dir.join(name),
                    &base_dir,
                    config,
                    DEFAULT_TARGET_MODE,
                ).unwrap()
            })
            .collect();

        if let Some(ref extra_files) = rpm_metadata.files {
            for (name, config) in extra_files {
                archive_files.push(ArchiveFile::new(
                    &rpm_config_dir.join(name),
                    &base_dir,
                    config,
                    DEFAULT_FILE_MODE,
                )?);
            }
        }

        Ok(Self {
            files: archive_files,
        })
    }

    /// Build the archive, placing the resulting file at the given path
    pub fn build(&self, output_file: &Path) -> Result<(), Error> {
        let archive = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output_file)?;

        let gzipper = GzEncoder::new(archive, Compression::default());
        let mut builder = Builder::new(gzipper);

        for file in &self.files {
            file.append_to(&mut builder)?;
        }

        Ok(())
    }
}
