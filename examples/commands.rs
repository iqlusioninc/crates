extern crate gumdrop;
#[macro_use] extern crate gumdrop_derive;

use std::env::args;
use gumdrop::Options;

// Define options for the program.
#[derive(Debug, Default, Options)]
struct MyOptions {
    // Options here can be accepted with any command (or none at all),
    // but they must come before the command name.
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "be verbose")]
    verbose: bool,

    // The `command` option will delegate option parsing to the command type,
    // starting at the first free argument.
    #[options(command)]
    command: Option<Command>,
}

// The set of commands and the options each one accepts.
//
// Each variant of a command enum should be a unary tuple variant with only
// one field. This field must implement `Options` and is used to parse arguments
// that are given after the command name.
#[derive(Debug, Options)]
enum Command {
    // Command names are generated from variant names.
    // By default, a CamelCase name will be converted into a lowercase,
    // hyphen-separated name; e.g. `FooBar` becomes `foo-bar`.
    //
    // Names can be explicitly specified using `#[options(name = "...")]`
    #[options(help = "make stuff")]
    Make(MakeOpts),
    #[options(help = "install stuff")]
    Install(InstallOpts),
}

// Options accepted for the `make` command
#[derive(Debug, Default, Options)]
struct MakeOpts {
    #[options(help = "print help message")]
    help: bool,
    #[options(free)]
    free: Vec<String>,
    #[options(help = "number of jobs", meta = "N")]
    jobs: Option<u32>,
}

// Options accepted for the `install` command
#[derive(Debug, Default, Options)]
struct InstallOpts {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "target directory")]
    dir: Option<String>,
}

fn main() {
    let args: Vec<String> = args().collect();

    // Remember to skip the first argument. That's the program name.
    let opts = match MyOptions::parse_args_default(&args[1..]) {
        Ok(opts) => opts,
        Err(e) => {
            println!("{}: {}", args[0], e);
            return;
        }
    };

    // The `help_requested` method will tell you whether the `--help` option
    // was supplied in main options or in any subcommand.
    if opts.help_requested() {
        match opts.command_name() {
            None => {
                // Main options are printed in the usual way.
                // This does not include any mention of commands because that
                // information is held by the Command type itself.
                println!("Usage: {} [OPTIONS] [COMMAND] [ARGUMENTS]", args[0]);
                println!();
                println!("{}", MyOptions::usage());
                println!();

                // Help text for commands comes can be found in the `usage` method
                // of our Command enum.
                println!("Available commands:");
                println!();
                println!("{}", Command::usage());
            }
            Some(cmd) => {
                // The Command enum will also give us a list of a command's options
                // if we ask for it by name. These are the same strings you'd get
                // from the `usage` method on each option struct.
                if let Some(help) = Command::command_usage(cmd) {
                    if help.is_empty() {
                        println!("command `{}` has no options", cmd);
                    } else {
                        println!("command `{}` accepts the following options:", cmd);
                        println!();
                        println!("{}", help);
                    }
                } else {
                    println!("{}: unrecognized command: {}", args[0], cmd);
                }
            }
        }
    } else {
        println!("{:#?}", opts);
    }
}
