//! Macros related to command-line option parsing

/// Print the current package's name and version to STDOUT
#[macro_export]
macro_rules! print_package_version {
    () => {
        println!(concat!(
            env!("CARGO_PKG_NAME"),
            " ",
            env!("CARGO_PKG_VERSION")
        ));
    };
}

/// Print the current package's authors (comma separated) to STDOUT
#[macro_export]
macro_rules! print_package_authors {
    () => {
        let authors: Vec<_> = env!("CARGO_PKG_AUTHORS").split(':').collect();
        println!("{}", authors.join(", "));
    };
}

/// Print a description for a subcommand
// TODO: less hax way of doing this
#[macro_export]
macro_rules! print_subcommand_description {
    ($command:tt, $subcommand:expr) => {
        for command_info in $command::command_list().unwrap().split('\n') {
            let mut command_info_parts = command_info.split_whitespace();

            if $subcommand != command_info_parts.next().unwrap() {
                continue;
            }

            let command_description: Vec<_> = command_info_parts.collect();
            println!("{}", command_description.join(" "));
        }
    };
}

/// Print flags for a subcommand
// TODO: less hax way of doing this
#[macro_export]
macro_rules! print_subcommand_flags {
    ($command:tt, $subcommand:expr) => {
        let usage = $command::command_usage($subcommand)
            .unwrap()
            .replace("Optional arguments:", "OPTIONS:");

        // TODO: descriptions for each command
        println!("{}", usage);
    };
}

/// Print subcommand usage
// TODO: less hax way of doing this
#[macro_export]
macro_rules! print_subcommand_usage {
    ($command:tt, $subcommand:expr) => {
        print_subcommand_description!($command, $subcommand);
        println!();
        println!("USAGE:");
        println!(
            concat!("  ", env!("CARGO_PKG_NAME"), " {} [OPTIONS]"),
            $subcommand
        );
        println!();
        print_subcommand_flags!($command, $subcommand);

        ::std::process::exit(2);
    };
}

/// Print usage information for the given command to STDOUT and then exit with
/// a usage status code (i.e. `2`).
///
/// `$command` is expected to be an `enum` which implements `iq_cli::Options`
#[macro_export]
macro_rules! print_usage {
    // TODO: move (most of?) this into a function rather than a macro
    ($command:tt) => {
        let args: &[String] = &[];
        print_usage!($command, args);
    };
    ($command:tt, $args:expr) => {
        use $crate::Options;
        print_package_version!();
        print_package_authors!();

        if $args.len() == 1 {
            print_subcommand_usage!($command, &$args[0]);
        }

        println!(env!("CARGO_PKG_DESCRIPTION"));
        println!();
        println!("USAGE:");
        println!(concat!("  ", env!("CARGO_PKG_NAME"), " <SUBCOMMAND>"));
        println!();
        println!("FLAGS:");
        println!("  -h, --help     Prints help information");
        println!("  -V, --version  Prints version information");
        println!();
        println!("SUBCOMMANDS:");
        println!("{}", $command::command_list().unwrap());

        ::std::process::exit(2);
    };
}

/// Implement the `from_args` and `from_env_args` methods for a command
// TODO: less hax way of doing this (move into `derive(Options)`?)
#[macro_export]
macro_rules! impl_command {
    ($command:tt) => {
        impl $command {
            /// Parse command-line arguments from a string iterator
            pub fn from_args<A: IntoIterator<Item = String>>(into_args: A) -> Self {
                let args: Vec<_> = into_args.into_iter().collect();

                if args.len() == 1 {
                    match args[0].as_ref() {
                        "-h" | "--help" => {
                            print_usage!(Self);
                        }
                        "-V" | "--version" => {
                            print_package_version!();
                            ::std::process::exit(0);
                        }
                        _ => (),
                    }
                }

                Self::parse_args_default(args.as_slice()).unwrap_or_else(|e| {
                    match e.to_string().as_ref() {
                        // Show usage if no command name is given or if "help" is given
                        // TODO: don't gate on a string, handle the error properly
                        "missing command name" => {
                            print_usage!(Self);
                        }
                        string => eprintln!("{}: {}", args[0], string),
                    }

                    ::std::process::exit(2);
                })
            }

            /// Parse command-line arguments from the environment
            pub fn from_env_args() -> Self {
                let mut args = ::std::env::args();
                assert!(args.next().is_some(), "expected one argument but got zero");
                Self::from_args(args)
            }
        }
    };
}
