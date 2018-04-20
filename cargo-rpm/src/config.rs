//! Cargo.toml parser specialized for the `cargo rpm` use case

use failure::Error;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use toml;

/// Name of the file containing cargo configuration. You know...
pub const CARGO_CONFIG_FILE: &str = "Cargo.toml";

/// Struct representing a `Cargo.toml` file
#[derive(Debug, Deserialize)]
pub struct CargoConfig {
    /// Cargo package configuration
    pub package: PackageConfig,
}

/// Struct representing a `Cargo.toml` file's `[package]` section
#[derive(Clone, Debug, Deserialize)]
pub struct PackageConfig {
    /// Name of the package
    pub name: String,

    /// Description of the package
    pub description: String,

    /// Version of the package
    pub version: String,

    /// License of the package
    pub license: Option<String>,

    /// Homepage of the package
    pub homepage: Option<String>,

    /// Package metadata table
    pub metadata: Option<PackageMetadata>,
}

impl PackageConfig {
    /// Parse the given path (i.e. `Cargo.toml`), returning a PackageConfig struct
    pub fn load<P>(filename: P) -> Result<PackageConfig, Error>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(filename.as_ref())?;
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let config: CargoConfig = toml::from_str(&data)
            .map_err(|e| format_err!("error parsing {}: {}", filename.as_ref().display(), e))?;

        Ok(config.package)
    }

    /// Get the RpmConfig for this package (if present)
    pub fn rpm_metadata(&self) -> Option<&RpmConfig> {
        match self.metadata {
            Some(ref m) => m.rpm.as_ref(),
            None => None,
        }
    }
}

/// The `[package.metadata]` table: ignored by Cargo, but we can put stuff there
#[derive(Clone, Debug, Deserialize)]
pub struct PackageMetadata {
    /// Our custom RPM metadata extension to `Cargo.toml`
    pub rpm: Option<RpmConfig>,
}

/// Our `[package.metadata.rpm]` extension to `Cargo.toml`
#[derive(Clone, Debug, Deserialize)]
pub struct RpmConfig {
    /// Options for creating the release artifact
    pub cargo: Option<CargoFlags>,

    /// Target configuration: a map of target binaries to their file config
    pub targets: BTreeMap<String, FileConfig>,

    /// Extra files (taken from the `.rpm` directory) to include in the RPM
    pub files: Option<BTreeMap<String, FileConfig>>,
}

/// Options for creating the release artifact
#[derive(Clone, Debug, Deserialize)]
pub struct CargoFlags {
    /// Release profile to use (default "release")
    pub profile: Option<String>,

    /// Flags to pass to cargo build
    pub buildflags: Option<Vec<String>>,
}

/// Properties of a file to be included in the final RPM
#[derive(Clone, Debug, Deserialize)]
pub struct FileConfig {
    /// Absolute path where the file should reside after installation
    pub path: PathBuf,

    /// Username of the owner of the file
    pub username: Option<String>,

    /// Groupname of the owner of the file
    pub groupname: Option<String>,

    /// Mode of the file (default 755 for targets, 644 for extra files)
    pub mode: Option<String>,
}

/// Render `package.metadata.rpm` section to include in Cargo.toml
pub fn append_rpm_metadata(
    path: &Path,
    targets: &[String],
    extra_files: &[PathBuf],
    bin_dir: &Path,
) -> Result<(), Error> {
    assert!(!targets.is_empty(), "no target configuration?!");

    status_ok!("Updating", path.canonicalize().unwrap().display());

    let mut cargo_toml = OpenOptions::new().append(true).open(path)?;

    // Flags to pass to cargo when doing a release
    // TODO: use serde serializer?
    writeln!(cargo_toml, "\n[package.metadata.rpm.cargo]")?;
    writeln!(cargo_toml, "buildflags = [\"--release\"]")?;

    // Target files to include in an archive
    writeln!(cargo_toml, "\n[package.metadata.rpm.targets]")?;

    for target in targets {
        writeln!(
            cargo_toml,
            "{} = {{ path = {:?} }}",
            target,
            bin_dir.join(target)
        )?;
    }

    // These files come from the .rpm directory
    if !extra_files.is_empty() {
        writeln!(cargo_toml, "\n[package.metadata.rpm.files]")?;

        for path in extra_files {
            if !path.is_absolute() {
                status_error!("path is not absolute: {}", path.display());
                exit(1);
            }

            let file = path.file_name().unwrap_or_else(|| {
                status_error!("path has no filename: {}", path.display());
                exit(1);
            });

            writeln!(
                cargo_toml,
                "{:?} = {{ path = {:?} }}",
                file.to_str().unwrap(),
                path.display()
            )?;
        }
    }

    Ok(())
}
