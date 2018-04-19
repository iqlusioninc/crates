//! The `cargo rpm build` subcommand

use failure::Error;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::time::SystemTime;

use archive::Archive;
use config::{PackageConfig, RpmConfig, CARGO_CONFIG_FILE};
use rpmbuild::Rpmbuild;
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
        let target_dir = target::find_dir()?;

        Builder::new(package_config, self.verbose, &rpm_config_dir, &target_dir).build()?;
        Ok(())
    }
}

/// Build RPMs from Rust projects
pub struct Builder {
    /// Cargo.toml configuration
    pub config: PackageConfig,

    /// Are we in verbose mode?
    pub verbose: bool,

    /// RPM configuration directory (i.e. `.rpm`)
    pub rpm_config_dir: PathBuf,

    /// Path to the target directory
    pub target_dir: PathBuf,

    /// Path to the rpmbuild directory
    pub rpmbuild_dir: PathBuf,
}

impl Builder {
    /// Create a new RPM builder
    pub fn new(
        config: PackageConfig,
        verbose: bool,
        rpm_config_dir: &Path,
        base_target_dir: &Path,
    ) -> Self {
        let mut profile = DEFAULT_PROFILE.to_owned();

        {
            let rpm_metadata = config.rpm_metadata().unwrap_or_else(|| {
                shell::say_status(
                    "error:",
                    "No [package.metadata.rpm] in Cargo.toml!",
                    color::RED,
                    false,
                );

                println!("\nRun 'cargo rpm init' to configure crate for RPM builds");
                exit(1);
            });

            if let Some(ref cargo) = rpm_metadata.cargo {
                if let Some(ref p) = cargo.profile {
                    profile = p.to_owned();
                }
            }
        }

        let target_dir = base_target_dir.join(profile);
        let rpmbuild_dir = target_dir.join("rpmbuild");

        Self {
            config,
            verbose,
            rpm_config_dir: rpm_config_dir.into(),
            target_dir,
            rpmbuild_dir,
        }
    }

    /// Build an RPM for this package
    pub fn build(&self) -> Result<(), Error> {
        let began_at = SystemTime::now();

        self.cargo_build()?;
        self.create_archive()?;
        self.render_spec()?;
        self.rpmbuild()?;

        shell::say_status(
            "Finished",
            format!(
                "{}-{}.rpm built in {} secs",
                self.config.name,
                self.config.version,
                began_at.elapsed()?.as_secs()
            ),
            color::GREEN,
            true,
        );
        Ok(())
    }

    /// Retrieve the RPM metadata for this crate
    fn rpm_metadata(&self) -> &RpmConfig {
        self.config.rpm_metadata().unwrap()
    }

    /// Compile the project with "cargo build"
    fn cargo_build(&self) -> Result<(), Error> {
        let mut buildflags = vec![];

        if let Some(ref cargo) = self.rpm_metadata().cargo {
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

        let status = Command::new("cargo")
            .arg("build")
            .args(&buildflags)
            .status()?;

        // Exit with the same exit code cargo used
        if !status.success() {
            exit(status.code().unwrap_or(1));
        }

        Ok(())
    }

    /// Create the archive (i.e. tarball) containing targets and additional files
    fn create_archive(&self) -> Result<(), Error> {
        let sources_dir = self.rpmbuild_dir.join("SOURCES");
        fs::create_dir_all(&sources_dir)?;

        // Build a tarball containing the RPM's contents
        let archive_file = format!("{}-{}.tar.gz", self.config.name, self.config.version);
        let archive_path = sources_dir.join(&archive_file);

        if self.verbose {
            shell::say_status(
                "Creating",
                format!("release archive ({})", &archive_file),
                color::GREEN,
                true,
            );
        }

        Archive::new(&self.config, &self.rpm_config_dir, &self.target_dir)?.build(&archive_path)?;

        Ok(())
    }

    /// Render the package's RPM spec file
    fn render_spec(&self) -> Result<(), Error> {
        // Read the spec file from `.rpm`
        let spec_filename = format!("{}.spec", self.config.name);
        let mut spec_src = File::open(self.rpm_config_dir.join(&spec_filename))?;
        let mut spec_template = String::new();
        spec_src.read_to_string(&mut spec_template)?;

        // Replace `@@VERSION@@` with the crate's actual version
        let spec_rendered = str::replace(&spec_template, VERSION_PLACEHOLDER, &self.config.version);

        let spec_dir = self.rpmbuild_dir.join("SPECS");
        fs::create_dir_all(&spec_dir)?;

        let mut spec_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(spec_dir.join(&spec_filename))?;

        spec_file.write_all(spec_rendered.as_bytes())?;

        Ok(())
    }

    /// Run rpmbuild
    fn rpmbuild(&self) -> Result<(), Error> {
        let rpm_file = format!("{}-{}.rpm", self.config.name, self.config.version);
        let cmd = Rpmbuild::new(self.verbose)?;

        shell::say_status(
            "Building",
            format!("{} (using rpmbuild {})", rpm_file, cmd.version().unwrap()),
            color::GREEN,
            true,
        );

        // Create directories needed by rpmbuild
        for dir in &["RPMS", "SRPMS", "BUILD", "SOURCES", "SPECS", "tmp"] {
            fs::create_dir_all(self.rpmbuild_dir.join(dir))?;
        }

        // Change directory to `target/<profile>/rpmbuild`
        env::set_current_dir(&self.rpmbuild_dir)?;

        // Calculate rpmbuild arguments
        let spec_path = format!("SPECS/{}.spec", self.config.name);
        let topdir_macro = format!("_topdir {}", self.rpmbuild_dir.display());
        let tmppath_macro = format!("_tmppath {}", self.rpmbuild_dir.join("tmp").display());

        // Calculate rpmbuild arguments
        let args = ["-D", &topdir_macro, "-D", &tmppath_macro, "-ba", &spec_path];

        if self.verbose {
            shell::say_status(
                "Running",
                format!("{} {}", cmd.path.display(), &args.join(" ")),
                color::GREEN,
                true,
            );
        }

        // Actually run rpmbuild
        cmd.exec(&args)
    }
}
