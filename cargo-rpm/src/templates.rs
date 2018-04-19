//! Handlebars templates (for RPM specs, etc)

use failure::Error;
use handlebars::Handlebars;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use config::PackageConfig;
use license;

/// Default RPM spec template (in toplevel `template/spec.hbs`)
pub const DEFAULT_SPEC_TEMPLATE: &str = include_str!("../templates/spec.hbs");

/// Default systemd service unit template (in toplevel `template/service.hbs`)
pub const DEFAULT_SERVICE_TEMPLATE: &str = include_str!("../templates/service.hbs");

/// Parameters passed to the RPM spec template
#[derive(Serialize)]
pub struct SpecParams {
    /// Name of the RPM, sans ".rpm", e.g. "ripgrep"
    pub name: String,

    /// Description of the RPM
    pub summary: String,

    /// License of the *binary* contents of the RPM
    pub license: Option<String>,

    /// URL to a home page for this package
    pub url: Option<String>,

    /// Name of a systemd service unit (if enabled)
    pub service: Option<String>,

    /// Are we placing targets in sbin instead of bin?
    pub use_sbin: bool,
}

impl SpecParams {
    /// Create a new set of RPM spec template parameters
    pub fn new(package: &PackageConfig, service: Option<String>, use_sbin: bool) -> Self {
        let rpm_license = package.license.as_ref().map(|spdx_license| {
            license::convert(spdx_license).unwrap_or_else(|e| {
                status_warn!("couldn't parse license {:?}: {}", spdx_license, e);
                spdx_license.to_owned()
            })
        });

        Self {
            name: package.name.to_owned(),
            summary: package.description.to_owned(),
            license: rpm_license,
            url: package.homepage.to_owned(),
            service,
            use_sbin,
        }
    }

    /// Render an RPM spec template at the given path (or default)
    pub fn render(&self, template_path: Option<&Path>) -> Result<String, Error> {
        let name = match template_path {
            Some(p) => p.display().to_string(),
            None => "(default:spec.hbs)".to_owned(),
        };

        let template = load_template(template_path, DEFAULT_SPEC_TEMPLATE)?;
        render_template(&name, &template, self)
    }
}

/// Paramters passed to the systemd service unit template
#[derive(Serialize)]
pub struct ServiceParams {
    /// Description of the service
    pub description: String,

    /// Path to the binary for systemd to spawn (absolute)
    pub bin_path: PathBuf,
}

impl ServiceParams {
    /// Render a systemd service unit template at the given path (or default)
    pub fn render(&self, template_path: Option<&Path>) -> Result<String, Error> {
        let name = match template_path {
            Some(p) => p.display().to_string(),
            None => "(default:service.hbs)".to_owned(),
        };

        let template = load_template(template_path, DEFAULT_SERVICE_TEMPLATE)?;
        render_template(&name, &template, self)
    }
}

impl<'a> From<&'a PackageConfig> for ServiceParams {
    fn from(package: &'a PackageConfig) -> Self {
        Self {
            description: package.description.to_owned(),
            /// TODO: better handling of target binaries and their paths
            bin_path: PathBuf::from("/usr/sbin").join(&package.name),
        }
    }
}

/// Load a template
fn load_template(template_path: Option<&Path>, default_template: &str) -> Result<String, Error> {
    match template_path {
        Some(p) => {
            let mut file = File::open(p)?;
            let mut s = String::new();
            file.read_to_string(&mut s)?;
            Ok(s)
        }
        None => Ok(default_template.to_owned()),
    }
}

/// Render a template
fn render_template<T: Serialize>(name: &str, template: &str, data: &T) -> Result<String, Error> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string(name, template).unwrap();
    handlebars
        .render(name, data)
        .map_err(|e| format_err!("Error rendering template: {}", e))
}
