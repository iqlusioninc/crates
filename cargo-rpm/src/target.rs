//! Target-type autodetection for crates

use failure::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Locate the project's target directory
pub fn find_dir() -> Result<PathBuf, Error> {
    // Check all parents of the current directory for a target directory.
    // We could call `cargo metadata` to find it but this is much cheaper.
    let mut path = fs::canonicalize(".")?;

    loop {
        let target = path.join("target");
        if target.exists() {
            return Ok(target);
        }

        path = match path.parent() {
            Some(p) => p.to_path_buf(),
            None => bail!("couldn't find target directory!"),
        }
    }
}

/// Target types we can autodetect
pub enum TargetType {
    /// Library crate i.e. `lib.rs` (we don't support these yet)
    Lib,

    /// Binary crate with a single executable i.e. `main.rs`
    Bin,

    /// Crate with multiple binary targets i.e. `src/bin/*.rs`
    /// (we don't support these yet)
    MultiBin(Vec<String>),
}

impl TargetType {
    /// Autodetect the targets for this crate
    pub fn detect(base_path: &Path) -> Result<Self, Error> {
        if base_path.join("src/bin").exists() {
            let mut bins = vec![];

            for bin in fs::read_dir(base_path.join("src/bin"))? {
                let mut bin_str = bin?.path().display().to_string();

                if !bin_str.ends_with(".rs") {
                    bail!("unrecognized file in src/bin: {:?}", bin_str);
                }

                // Remove .rs extension
                let new_len = bin_str.len() - 3;
                bin_str.truncate(new_len);
                bins.push(bin_str);
            }

            Ok(TargetType::MultiBin(bins))
        } else if base_path.join("src/main.rs").exists() {
            Ok(TargetType::Bin)
        } else if base_path.join("src/lib.rs").exists() {
            Ok(TargetType::Lib)
        } else {
            bail!("couldn't detect crate type (no main.rs or lib.rs?)");
        }
    }
}
