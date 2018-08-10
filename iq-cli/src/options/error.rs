use std::error::Error as StdError;
use std::fmt::{self, Display};

use super::Opt;

/// Represents an error encountered during argument parsing
// TODO: derive or implement `Fail`
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Returns an error for a failed attempt at parsing an option value.
    pub fn failed_parse(opt: Opt, err: String) -> Error {
        Error {
            kind: ErrorKind::FailedParse(opt.to_string(), err),
        }
    }

    /// Returns an error for a failed attempt at parsing an option's default value.
    pub fn failed_parse_default(option: &'static str, value: &'static str, err: String) -> Error {
        Error {
            kind: ErrorKind::FailedParseDefault { option, value, err },
        }
    }

    /// Returns an error for an option expecting two or more arguments not
    /// receiving the expected number of arguments.
    pub fn insufficient_arguments(opt: Opt, expected: usize, found: usize) -> Error {
        let kind = ErrorKind::InsufficientArguments {
            option: opt.to_string(),
            expected,
            found,
        };

        Error { kind }
    }

    /// Returns an error for an option receiving an unexpected argument value,
    /// e.g. `--option=value`.
    pub fn unexpected_argument(opt: Opt) -> Error {
        Error {
            kind: ErrorKind::UnexpectedArgument(opt.to_string()),
        }
    }

    /// Returns an error for an option expecting two or more argument values
    /// receiving only one in the long form, e.g. `--option=value`.
    ///
    /// These options must be passed as, e.g. `--option value second-value [...]`.
    pub fn unexpected_single_argument(opt: Opt, n: usize) -> Error {
        Error {
            kind: ErrorKind::UnexpectedSingleArgument(opt.to_string(), n),
        }
    }

    /// Returns an error for a missing required argument.
    pub fn missing_argument(opt: Opt) -> Error {
        Error {
            kind: ErrorKind::MissingArgument(opt.to_string()),
        }
    }

    /// Returns an error for a missing command name.
    pub fn missing_command() -> Error {
        Error {
            kind: ErrorKind::MissingCommand,
        }
    }

    /// Returns an error for a missing required option.
    pub fn missing_required(opt: &str) -> Error {
        Error {
            kind: ErrorKind::MissingRequired(opt.to_owned()),
        }
    }

    /// Returns an error for a missing required command.
    pub fn missing_required_command() -> Error {
        Error {
            kind: ErrorKind::MissingRequiredCommand,
        }
    }

    /// Returns an error for a missing required free argument.
    pub fn missing_required_free() -> Error {
        Error {
            kind: ErrorKind::MissingRequiredFree,
        }
    }

    /// Returns an error when a free argument was encountered, but the options
    /// type does not support free arguments.
    pub fn unexpected_free(arg: &str) -> Error {
        Error {
            kind: ErrorKind::UnexpectedFree(arg.to_owned()),
        }
    }

    /// Returns an error for an unrecognized command.
    pub fn unrecognized_command(name: &str) -> Error {
        Error {
            kind: ErrorKind::UnrecognizedCommand(name.to_owned()),
        }
    }

    /// Returns an error for an unrecognized option.
    pub fn unrecognized_option(opt: Opt) -> Error {
        match opt {
            Opt::Short(short) => Error::unrecognized_short(short),
            Opt::Long(long) | Opt::LongWithArg(long, _) => Error::unrecognized_long(long),
            Opt::Free(_) => panic!("`Error::unrecognized_option` called with `Opt::Free` value"),
        }
    }

    /// Returns an error for an unrecognized long option, e.g. `--option`.
    pub fn unrecognized_long(opt: &str) -> Error {
        Error {
            kind: ErrorKind::UnrecognizedLongOption(opt.to_owned()),
        }
    }

    /// Returns an error for an unrecognized short option, e.g. `-o`.
    pub fn unrecognized_short(opt: char) -> Error {
        Error {
            kind: ErrorKind::UnrecognizedShortOption(opt),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorKind::*;

        match self.kind {
            FailedParse(ref opt, ref arg) => {
                write!(f, "invalid argument to option `{}`: {}", opt, arg)
            }
            FailedParseDefault {
                ref option,
                value,
                ref err,
            } => write!(
                f,
                "invalid default value for `{}` ({:?}): {}",
                option, value, err
            ),
            InsufficientArguments {
                ref option,
                expected,
                found,
            } => write!(
                f,
                "insufficient arguments to option `{}`: expected {}; found {}",
                option, expected, found
            ),
            MissingArgument(ref opt) => write!(f, "missing argument to option `{}`", opt),
            MissingCommand => f.write_str("missing command name"),
            MissingRequired(ref opt) => write!(f, "missing required option `{}`", opt),
            MissingRequiredCommand => f.write_str("missing required command"),
            MissingRequiredFree => f.write_str("missing required free argument"),
            UnexpectedArgument(ref opt) => {
                write!(f, "option `{}` does not accept an argument", opt)
            }
            UnexpectedSingleArgument(ref opt, n) => {
                write!(f, "option `{}` expects {} arguments; found 1", opt, n)
            }
            UnexpectedFree(ref arg) => write!(f, "unexpected free argument `{}`", arg),
            UnrecognizedCommand(ref cmd) => write!(f, "unrecognized command `{}`", cmd),
            UnrecognizedLongOption(ref opt) => write!(f, "unrecognized option `--{}`", opt),
            UnrecognizedShortOption(opt) => write!(f, "unrecognized option `-{}`", opt),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "failed to parse arguments"
    }
}

#[derive(Debug)]
enum ErrorKind {
    FailedParse(String, String),
    FailedParseDefault {
        option: &'static str,
        value: &'static str,
        err: String,
    },
    InsufficientArguments {
        option: String,
        expected: usize,
        found: usize,
    },
    MissingArgument(String),
    MissingCommand,
    MissingRequired(String),
    MissingRequiredCommand,
    MissingRequiredFree,
    UnexpectedArgument(String),
    UnexpectedSingleArgument(String, usize),
    UnexpectedFree(String),
    UnrecognizedCommand(String),
    UnrecognizedLongOption(String),
    UnrecognizedShortOption(char),
}
