//! Configuration file parsing helper

use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};
use toml;

use error::{CliError, CliErrorKind};

/// Parse a TOML configuration file, returning the given configuration or
/// the error which occurred while reading it
pub fn load_toml<P, C>(filename: P) -> Result<C, CliError>
where
    P: AsRef<Path>,
    C: Deserialize,
{
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    toml::from_str(&data).map_err(|e| err!(CliErrorKind::ConfigError, e))
}
