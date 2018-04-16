//! cargo-rpm: Cargo subcommand for creating RPM releases of Rust projects

#[macro_use]
extern crate failure;
extern crate gumdrop;
#[macro_use]
extern crate gumdrop_derive;
extern crate handlebars;
extern crate iq_cli;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

/// Cargo.toml parser
pub mod cargo_config;

/// License format converter
pub mod license;

/// The `cargo rpm init` subcommand
pub mod init;

/// Shell support (for printing status messages)
pub mod shell;

/// Target type autodetection for crates
pub mod target;

/// Handlebars templates (for RPM specs, etc)
pub mod templates;

/// Subdirectory of a Rust project in which we keep RPM-related configs
pub const RPM_CONFIG_DIR: &str = ".rpm";

use gumdrop::Options;
use std::env;
use std::process::exit;

use init::InitOpts;

/// Command line arguments (parsed by gumdrop)
#[derive(Debug, Options)]
enum Opts {
    #[options(help = "Build RPMs from Rust projects using Cargo")]
    Rpm(RpmOpts),
}

/// Options for the `cargo rpm` subcommand
#[derive(Debug, Options)]
enum RpmOpts {
    #[options(help = "Show help for a command")]
    Help(HelpOpts),

    #[options(help = "Initialize a Rust project with RPM support")]
    Init(InitOpts),
}

/// Options for the `help` command
#[derive(Debug, Default, Options)]
struct HelpOpts {
    #[options(free)]
    free: Vec<String>,
}

/// Main entry point
fn main() {
    let args: Vec<_> = env::args().collect();

    let Opts::Rpm(rpm_opts) = Opts::parse_args_default(&args[1..]).unwrap_or_else(|e| {
        match e.to_string().as_ref() {
            // Show usage if no command name is given or if "help" is given
            "missing command name" => help(),
            string => eprintln!("{}: {}", args[0], string),
        }

        exit(2);
    });

    match rpm_opts {
        RpmOpts::Help(_commands) => help(),
        RpmOpts::Init(init) => init.call(),
    }.unwrap_or_else(|e| shell::exit_error(e));

    exit(0);
}

/// Print help message
fn help() -> ! {
    println!("Usage: cargo rpm [COMMAND] [OPTIONS]");
    println!();
    println!("Available commands:");
    println!();
    println!("{}", RpmOpts::command_list().unwrap());
    println!();
    exit(2);
}
