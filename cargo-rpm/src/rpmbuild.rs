//! Wrapper for running the `rpmbuild` command

use failure::Error;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use shell::{self, color};

/// Path to the `rpmbuild` command
pub const DEFAULT_RPMBUILD_PATH: &str = "/usr/bin/rpmbuild";

/// Version of rpmbuild supported by this tool
pub const SUPPORTED_RPMBUILD_VERSION: &str = "RPM version 4";

/// Wrapper for the `rpmbuild` command
pub struct Rpmbuild {
    /// Path to rpmbuild
    pub path: PathBuf,

    /// Are we in verbose mode?
    pub verbose: bool,
}

impl Rpmbuild {
    /// Prepare `rpmbuild`, checking the correct version is installed
    pub fn new(verbose: bool) -> Result<Self, Error> {
        let rpmbuild = Self {
            path: DEFAULT_RPMBUILD_PATH.into(),
            verbose,
        };

        // Make sure we have a valid version of rpmbuild
        rpmbuild.version()?;
        Ok(rpmbuild)
    }

    /// Get version of `rpmbuild`
    pub fn version(&self) -> Result<String, Error> {
        let output = Command::new(&self.path)
            .args(&["--version"])
            .output()
            .map_err(|e| format_err!("error running {}: {}", self.path.display(), e))?;

        if !output.status.success() {
            bail!(
                "error running {} (exit status: {})",
                &self.path.display(),
                &output.status
            );
        }

        let vers = String::from_utf8(output.stdout)?;
        if !vers.starts_with(SUPPORTED_RPMBUILD_VERSION) {
            bail!("unexpected rpmbuild version string: {:?}", vers);
        }

        let parts: Vec<&str> = vers.split_whitespace().collect();
        Ok(parts[2].to_owned())
    }

    /// Execute `rpmbuild` with the given arguments
    pub fn exec<I, S>(&self, args: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut rpmbuild = Command::new(&self.path)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(if self.verbose {
                Stdio::inherit()
            } else {
                Stdio::null()
            })
            .spawn()
            .map_err(|e| format_err!("error running {}: {}", self.path.display(), e))?;

        self.read_rpmbuild_output(&mut rpmbuild)?;
        let status = rpmbuild.wait()?;

        if status.success() {
            Ok(())
        } else {
            bail!(
                "error running {} (exit status: {})",
                self.path.display(),
                status
            );
        }
    }

    /// Read stdout from rpmbuild, either displaying it or discarding it
    fn read_rpmbuild_output(&self, subprocess: &mut Child) -> Result<(), Error> {
        let mut reader = BufReader::new(subprocess.stdout.as_mut().unwrap());
        let mut line = String::new();

        while reader.read_line(&mut line)? != 0 {
            if self.verbose {
                shell::say_status("rpmbuild", &line, color::GREEN, true);
            }

            line.clear();
        }

        Ok(())
    }
}
