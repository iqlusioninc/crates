//! The `cargo rpm init` subcommand

use failure::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use RPM_CONFIG_DIR;
use cargo_config::PackageConfig;
use shell::{self, color};
use target::TargetType;
use templates::{ServiceParams, SpecParams};

/// Name of the file containing cargo configuration. You know...
const CARGO_CONFIG_FILE: &str = "Cargo.toml";

/// Directory in which systemd service unit configs reside
const SYSTEMD_DIR: &str = "/usr/lib/systemd/system";

/// Options for the `cargo rpm init` subcommand
#[derive(Debug, Default, Options)]
pub struct InitOpts {
    /// Force (re-)generation, even if .rpm exists or the target type is unsupported
    #[options(long = "force")]
    pub force: bool,

    /// Place binaries in `/usr/sbin` instead of `/usr/bin`?
    #[options(no_short, long = "sbin")]
    pub sbin: bool,

    /// Path to the systemd service unit config template
    #[options(no_short, long = "service")]
    pub service: Option<String>,

    /// Configure this RPM as a systemd service unit
    #[options(short = "s", long = "systemd")]
    pub systemd: bool,

    /// Path to the RPM spec template
    #[options(long = "template")]
    pub template: Option<String>,
}

impl InitOpts {
    /// Invoke the `cargo rpm init` subcommand
    pub fn call(&self) -> Result<(), Error> {
        // Calculate paths relative to the current directory
        let crate_root = PathBuf::from(".");
        let cargo_toml = crate_root.join(CARGO_CONFIG_FILE);
        let rpm_config_dir = crate_root.join(RPM_CONFIG_DIR);

        // Read Cargo.toml
        let package_config = PackageConfig::load(&cargo_toml)?;

        // Check if `.rpm` already exists
        if rpm_config_dir.exists() {
            if self.force {
                let canonical_rpm_config_dir = rpm_config_dir.canonicalize()?;
                shell::say_status(
                    "Deleting",
                    format!("{} (forced)", canonical_rpm_config_dir.display()),
                    color::YELLOW,
                    true,
                );
                fs::remove_dir_all(&rpm_config_dir)?;
            } else {
                shell::exit_error(format!(
                    "destination `{}` already exists!",
                    rpm_config_dir.canonicalize().unwrap().display()
                ));
            }
        }

        // Check if we're creating a systemd service unit for this crate
        let service_name = if self.service.is_some() || self.systemd {
            Some(format!("{}.service", package_config.name))
        } else {
            None
        };

        // Autodetect whether to place target files in `/usr/bin` or `/usr/sbin`
        let use_sbin = self.sbin || service_name.is_some();

        // Autodetect target types
        let targets = match TargetType::detect(&crate_root)? {
            TargetType::Lib => {
                if self.force {
                    // If forced, just return an empty target list
                    vec![]
                } else {
                    shell::exit_error("detected unsupported library crate (-f to override)");
                }
            }
            TargetType::Bin => vec![package_config.name.clone()],
            TargetType::MultiBin(targets) => targets,
        };

        // Create `.rpm` directory
        fs::create_dir(&rpm_config_dir)?;
        shell::say_status(
            "Created",
            rpm_config_dir.canonicalize().unwrap().display(),
            color::GREEN,
            true,
        );

        // Render `.rpm/<cratename>.spec`
        let spec_path = rpm_config_dir.join(format!("{}.spec", package_config.name));
        render_spec(&spec_path, &self.template, &package_config, &service_name)?;

        // (Optional) Render `.rpm/<cratename>.service` (systemd service unit config)
        if let Some(ref service) = service_name {
            render_service(
                &rpm_config_dir.join(service),
                &self.service,
                &package_config,
            )?;
        }

        // Update Cargo.toml with RPM metadata
        if package_config.rpm_metadata().is_some() && !self.force {
            shell::warning(
                "not updating Cargo.toml because [package.metadata.rpm] already present",
            );
        } else {
            let mut extra_files = vec![];
            if let Some(ref service) = service_name {
                extra_files.push(PathBuf::from(SYSTEMD_DIR).join(service.clone()));
            }

            let bin_dir: PathBuf = if use_sbin { "/usr/sbin" } else { "/usr/bin" }.into();
            update_cargo_metadata(&cargo_toml, &targets, &extra_files, &bin_dir)?;
        }

        shell::say_status(
            "Finished",
            format!(
                "{} configured (type \"cargo rpm build\" to build)",
                package_config.name
            ),
            color::GREEN,
            true,
        );

        Ok(())
    }
}

/// Render this package's RPM spec
fn render_spec(
    spec_path: &Path,
    template_path_str: &Option<String>,
    package_config: &PackageConfig,
    service_name: &Option<String>,
) -> Result<(), Error> {
    let mut spec_params = SpecParams::from(package_config);
    spec_params.service = service_name.clone();

    let template_path = template_path_str.as_ref().map(|t| PathBuf::from(t));
    let spec_rendered = spec_params.render(template_path.as_ref().map(|t| t.as_ref()))?;

    let mut spec_file = File::create(spec_path)?;
    spec_file.write_all(spec_rendered.as_bytes())?;

    shell::say_status(
        "Rendered",
        spec_path.canonicalize().unwrap().display(),
        color::GREEN,
        true,
    );

    Ok(())
}

/// Render this package's systemd service unit config (if enabled)
fn render_service(
    service_path: &Path,
    template_path_str: &Option<String>,
    package_config: &PackageConfig,
) -> Result<(), Error> {
    let service_params = ServiceParams::from(package_config);
    let template_path = template_path_str.as_ref().map(|t| PathBuf::from(t));
    let service_rendered = service_params.render(template_path.as_ref().map(|t| t.as_ref()))?;

    let mut service_file = File::create(service_path)?;
    service_file.write_all(service_rendered.as_bytes())?;

    shell::say_status(
        "Rendered",
        service_path.canonicalize().unwrap().display(),
        color::GREEN,
        true,
    );

    Ok(())
}

/// Render `package.metadata.rpm` section to include in Cargo.toml
fn update_cargo_metadata(
    path: &Path,
    targets: &[String],
    extra_files: &[PathBuf],
    bin_dir: &Path,
) -> Result<(), Error> {
    assert!(!targets.is_empty(), "no target configuration?!");

    shell::say_status(
        "Updating",
        path.canonicalize().unwrap().display(),
        color::BRIGHT_CYAN,
        true,
    );

    // TODO: use real serde serializer?
    let mut cargo_toml = OpenOptions::new().append(true).open(path)?;
    writeln!(cargo_toml, "\n[package.metadata.rpm.target]")?;

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
        writeln!(cargo_toml, "\n[package.metadata.rpm.file]")?;

        for path in extra_files {
            if !path.is_absolute() {
                shell::exit_error(format!("path is not absolute: {}", path.display()));
            }

            let file = path.file_name().unwrap_or_else(|| {
                shell::exit_error(format!("path has no filename: {}", path.display()));
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
