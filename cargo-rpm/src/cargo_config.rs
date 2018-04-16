//! Cargo.toml parser specialized for the `cargo rpm` use case

use failure::Error;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;

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
    /// Target configuration: a map of target binaries to their file config
    target: BTreeMap<String, FileConfig>,

    /// Extra files (taken from the `.rpm` directory) to include in the RPM
    file: Option<BTreeMap<String, FileConfig>>,
}

/// Properties of a file to be included in the final RPM
#[derive(Clone, Debug, Deserialize)]
pub struct FileConfig {
    /// Absolute path where the file should reside after installation
    path: PathBuf,
}
