//! The `cargo rpm build` subcommand

use failure::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{exit, Command};

use archive::Archive;
use config::{PackageConfig, CARGO_CONFIG_FILE};
use shell::{self, color};
use target;
use RPM_CONFIG_DIR;

/// Default build profile to use
pub const DEFAULT_PROFILE: &str = "release";

/// Placeholder string in the `.spec` file we use for the version
pub const VERSION_PLACEHOLDER: &str = "@@VERSION@@";

/// Options for the `cargo rpm build` subcommand
#[derive(Debug, Default, Options)]
pub struct BuildOpts {
    /// Print additional information about the build
    #[options(long = "verbose")]
    pub verbose: bool,
}

impl BuildOpts {
    /// Invoke the `cargo rpm build` subcommand
    pub fn call(&self) -> Result<(), Error> {
        // Calculate paths relative to the current directory
        let crate_root = PathBuf::from(".");
        let cargo_toml = crate_root.join(CARGO_CONFIG_FILE);
        let rpm_config_dir = crate_root.join(RPM_CONFIG_DIR);

        // Read Cargo.toml
        let package_config = PackageConfig::load(&cargo_toml)?;
        let rpm_metadata = package_config.rpm_metadata().unwrap_or_else(|| {
            shell::say_status(
                "error:",
                "No [package.metadata.rpm] in Cargo.toml!",
                color::RED,
                false,
            );

            println!("\nRun 'cargo rpm init' to configure crate for RPM builds");
            exit(1);
        });

        // Run "cargo build"
        let mut profile = DEFAULT_PROFILE.to_owned();
        let mut buildflags = vec![];

        if let Some(ref cargo) = rpm_metadata.cargo {
            if let Some(ref p) = cargo.profile {
                profile = p.to_owned();
            }

            if let Some(ref b) = cargo.buildflags {
                buildflags = b.clone();
            }
        };

        if self.verbose {
            shell::say_status(
                "Running",
                format!("cargo build {}", buildflags.join(" ")),
                color::GREEN,
                true,
            );
        }

        do_cargo_build(buildflags.as_slice())?;
        let base_target_dir = target::find_dir()?;

        let target_dir = base_target_dir.join(profile);
        let rpmbuild_dir = target_dir.join("rpmbuild");
        let sources_dir = rpmbuild_dir.join("SOURCES");
        fs::create_dir_all(&sources_dir)?;

        // Build a tarball containing the RPM's contents
        let archive_file = format!("{}-{}.tar.gz", package_config.name, package_config.version);
        let archive_path = sources_dir.join(&archive_file);

        shell::say_status("Creating", &archive_file, color::GREEN, true);
        Archive::new(&package_config, &rpm_config_dir, &target_dir)?.build(&archive_path)?;

        // Read the spec file from `.rpm`
        let spec_filename = format!("{}.spec", package_config.name);
        let mut spec_src = File::open(rpm_config_dir.join(&spec_filename))?;
        let mut spec_template = String::new();
        spec_src.read_to_string(&mut spec_template)?;

        // Replace `@@VERSION@@` with the crate's actual version
        let spec_rendered =
            str::replace(&spec_template, VERSION_PLACEHOLDER, &package_config.version);

        let spec_dir = rpmbuild_dir.join("SPEC");
        fs::create_dir_all(&spec_dir)?;
        File::create(spec_dir.join(&spec_filename))?.write_all(spec_rendered.as_bytes())?;

        Ok(())
    }
}

/// Compile the project with "cargo build"
fn do_cargo_build(flags: &[String]) -> Result<(), Error> {
    let status = Command::new("cargo").arg("build").args(flags).status()?;

    // Exit with the same exit code cargo used
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}
