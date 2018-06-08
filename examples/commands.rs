#[macro_use] extern crate gumdrop;

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
    // Parse options from the environment.
    // If there's an error or the user requests help,
    // the process will exit after giving the appropriate response.
    let opts = MyOptions::parse_args_default_or_exit();

    println!("{:#?}", opts);
}
