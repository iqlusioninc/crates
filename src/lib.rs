//! Option parser with custom derive support
//!
//! # Example
//!
//! ```no_run
//! extern crate gumdrop;
//! #[macro_use] extern crate gumdrop_derive;
//!
//! use std::env::args;
//! use gumdrop::Options;
//!
//! // Defines options that can be parsed from the command line.
//! //
//! // `derive(Options)` will generate an implementation of the trait `Options`.
//! // An implementation of `Default` (derived or otherwise) is required for the
//! // generated implementation.
//! //
//! // (`Debug` is only derived here for demonstration purposes.)
//! #[derive(Debug, Default, Options)]
//! struct MyOptions {
//!     // Contains "free" arguments -- those that are not options.
//!     // If no `free` field is declared, free arguments will result in an error.
//!     #[options(free)]
//!     free: Vec<String>,
//!
//!     // Boolean options are treated as flags, taking no additional values.
//!     // The optional `help` attribute is displayed in `usage` text.
//!     #[options(help = "print help message")]
//!     help: bool,
//!
//!     // Non-boolean fields will take a value from the command line.
//!     // Wrapping the type in an `Option` is not necessary, but provides clarity.
//!     #[options(help = "give a string argument")]
//!     string: Option<String>,
//!
//!     // A field can be any type that implements `FromStr`.
//!     // The optional `meta` attribute is displayed in `usage` text.
//!     #[options(help = "give a number as an argument", meta = "N")]
//!     number: Option<i32>,
//!
//!     // A `Vec` field will accumulate all values received from the command line.
//!     #[options(help = "give a list of string items")]
//!     item: Vec<String>,
//!
//!     // The `count` flag will treat the option as a counter.
//!     // Each time the option is encountered, the field is incremented.
//!     #[options(count, help = "increase a counting value")]
//!     count: u32,
//!
//!     // Option names are automatically generated from field names, but these
//!     // can be overriden. The attributes `short = "?"`, `long = "..."`,
//!     // `no_short`, and `no_long` are used to control option names.
//!     #[options(no_short, help = "this option has no short form")]
//!     long_option_only: bool,
//! }
//!
//! fn main() {
//!     let args: Vec<String> = args().collect();
//!
//!     // Remember to skip the first argument. That's the program name.
//!     let opts = match MyOptions::parse_args_default(&args[1..]) {
//!         Ok(opts) => opts,
//!         Err(e) => {
//!             println!("{}: {}", args[0], e);
//!             return;
//!         }
//!     };
//!
//!     if opts.help {
//!         // Printing usage text for the `--help` option is handled explicitly
//!         // by the program.
//!         // However, `derive(Options)` does generate information about all
//!         // defined options.
//!         println!("Usage: {} [OPTIONS] [ARGUMENTS]", args[0]);
//!         println!();
//!         println!("{}", MyOptions::usage());
//!     } else {
//!         println!("{:#?}", opts);
//!     }
//! }
//! ```

#![deny(missing_docs)]

#[cfg(test)] #[macro_use] extern crate assert_matches;

use std::error::Error as StdError;
use std::fmt;
use std::slice::Iter;
use std::str::Chars;

/// Parses arguments from the command line.
///
/// The first argument (the program name) should be omitted.
pub fn parse_args<T: Options>(args: &[String], style: ParsingStyle) -> Result<T, Error> {
    T::parse_args(args, style)
}

/// Parses arguments from the command line using the default parsing style.
///
/// The first argument (the program name) should be omitted.
pub fn parse_args_default<T: Options>(args: &[String]) -> Result<T, Error> {
    T::parse_args_default(args)
}

/// Represents an error encountered during argument parsing
#[derive(Debug)]
pub struct Error {
    inner: InnerError,
}

impl Error {
    /// Returns an error for a failed attempt at parsing an option value.
    pub fn failed_parse(opt: Opt, err: String) -> Error {
        Error{inner: InnerError::FailedParse(opt.to_string(), err)}
    }

    /// Returns an error for an option receiving an unexpected argument value,
    /// e.g. `--option=value`.
    pub fn unexpected_argument(opt: Opt) -> Error {
        Error{inner: InnerError::UnexpectedArgument(opt.to_string())}
    }

    /// Returns an error for a missing required argument.
    pub fn missing_argument(opt: Opt) -> Error {
        Error{inner: InnerError::MissingArgument(opt.to_string())}
    }

    /// Returns an error when a free argument was encountered, but the options
    /// type does not support free arguments.
    pub fn unexpected_free(arg: &str) -> Error {
        Error{inner: InnerError::UnexpectedFree(arg.to_owned())}
    }

    /// Returns an error for an unrecognized option.
    pub fn unrecognized_option(opt: Opt) -> Error {
        match opt {
            Opt::Short(short) => Error::unrecognized_short(short),
            Opt::Long(long) | Opt::LongWithArg(long, _) =>
                Error::unrecognized_long(long),
            Opt::Free(_) => panic!("`Error::unrecognized_option` called with `Opt::Free` value")
        }
    }

    /// Returns an error for an unrecognized long option, e.g. `--option`.
    pub fn unrecognized_long(opt: &str) -> Error {
        Error{inner: InnerError::UnrecognizedLongOption(opt.to_owned())}
    }

    /// Returns an error for an unrecognized short option, e.g. `-o`.
    pub fn unrecognized_short(opt: char) -> Error {
        Error{inner: InnerError::UnrecognizedShortOption(opt)}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InnerError::*;

        match self.inner {
            FailedParse(ref opt, ref arg) => write!(f, "invalid argument to option `{}`: {}", opt, arg),
            MissingArgument(ref opt) => write!(f, "missing argument to option `{}`", opt),
            UnexpectedArgument(ref opt) => write!(f, "option `{}` does not accept an argument", opt),
            UnexpectedFree(ref arg) => write!(f, "unexpected free argument `{}`", arg),
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
enum InnerError {
    FailedParse(String, String),
    MissingArgument(String),
    UnexpectedArgument(String),
    UnexpectedFree(String),
    UnrecognizedLongOption(String),
    UnrecognizedShortOption(char),
}

/// Parses options from a series of `&str`-like values.
pub struct Parser<'a, S: 'a> {
    args: Iter<'a, S>,
    cur: Option<Chars<'a>>,
    style: ParsingStyle,
    terminated: bool,
}

impl<'a, S: 'a + AsRef<str>> Parser<'a, S> {
    /// Returns a new parser for the given series of arguments.
    ///
    /// The given slice should **not** contain the program name as its first
    /// element.
    pub fn new(args: &'a [S], style: ParsingStyle) -> Parser<'a, S> {
        Parser{
            args: args.iter(),
            cur: None,
            style: style,
            terminated: false,
        }
    }

    /// Returns the next option or `None` if no options remain.
    ///
    /// If the previous option had an explicit argument, e.g. `--option=argument`,
    /// which was not consumed by a call to `next_arg()`, an error will be
    /// returned indicating that the argument was ignored.
    pub fn next_opt(&mut self) -> Option<Opt<'a>> {
        if let Some(mut cur) = self.cur.take() {
            if let Some(opt) = cur.next() {
                self.cur = Some(cur);
                return Some(Opt::Short(opt));
            }
        }

        if self.terminated {
            return self.args.next().map(|s| Opt::Free(s.as_ref()));
        }

        match self.args.next().map(|s| s.as_ref()) {
            Some(arg @ "-") => {
                if self.style == ParsingStyle::StopAtFirstFree {
                    self.terminated = true;
                }
                Some(Opt::Free(arg))
            }
            Some("--") => {
                self.terminated = true;
                self.args.next().map(|s| Opt::Free(s.as_ref()))
            }
            Some(long) if long.starts_with("--") => {
                match long.find('=') {
                    Some(pos) => Some(Opt::LongWithArg(
                        &long[2..pos], &long[pos + 1..])),
                    None => Some(Opt::Long(&long[2..]))
                }
            }
            Some(short) if short.starts_with('-') => {
                let mut chars = short[1..].chars();

                let res = chars.next().map(Opt::Short);

                self.cur = Some(chars);
                res
            }
            Some(free) => {
                if self.style == ParsingStyle::StopAtFirstFree {
                    self.terminated = true;
                }
                Some(Opt::Free(free))
            }
            None => None
        }
    }

    /// Returns the next argument to an option or `None` if none remain.
    pub fn next_arg(&mut self) -> Option<&'a str> {
        if let Some(cur) = self.cur.take() {
            let arg = cur.as_str();

            if !arg.is_empty() {
                return Some(arg);
            }
        }

        self.args.next().map(|s| s.as_ref())
    }
}

impl<'a, S: 'a> Clone for Parser<'a, S> {
    fn clone(&self) -> Parser<'a, S> {
        Parser{
            args: self.args.clone(),
            cur: self.cur.clone(),
            style: self.style,
            terminated: self.terminated,
        }
    }
}

/// Represents an option parsed from a `Parser`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Opt<'a> {
    /// Short option, e.g. `-o`
    Short(char),
    /// Long option, e.g. `--option`
    Long(&'a str),
    /// Long option with argument, e.g. `--option=value`
    LongWithArg(&'a str, &'a str),
    /// Free argument
    Free(&'a str),
}

impl<'a> Opt<'a> {
    fn to_string(&self) -> String {
        match *self {
            Opt::Short(ch) => format!("-{}", ch),
            Opt::Long(s) => format!("--{}", s),
            Opt::LongWithArg(opt, _) => format!("--{}", opt),
            Opt::Free(s) => s.to_owned()
        }
    }
}

/// Implements a set of options parsed from command line arguments.
///
/// An implementation of this trait can be generated with `#[derive(Options)]`
/// from the crate `gumdrop_derive`. Such a derived implementation requires that
/// the type implement the trait `Default`.
pub trait Options: Sized {
    /// Parses arguments received from the command line.
    ///
    /// The first argument (the program name) should be omitted.
    fn parse_args<S: AsRef<str>>(args: &[S], style: ParsingStyle) -> Result<Self, Error>;

    /// Parses arguments with the default `ParsingStyle`.
    fn parse_args_default<S: AsRef<str>>(args: &[S]) -> Result<Self, Error> {
        Self::parse_args(args, ParsingStyle::default())
    }

    /// Returns a string showing usage and help for each supported option.
    ///
    /// Option descriptions are separated by newlines. The returned string
    /// should **not** end with a newline.
    fn usage() -> &'static str;
}

/// Controls behavior of free arguments in `Parser`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParsingStyle {
    /// Process all option arguments that appear
    AllOptions,
    /// After the first "free" argument is encountered,
    /// all remaining arguments will be considered "free" arguments.
    StopAtFirstFree,
}

impl Default for ParsingStyle {
    /// Returns the default parsing style, `AllOptions`.
    fn default() -> ParsingStyle {
        ParsingStyle::AllOptions
    }
}

#[cfg(test)]
mod test {
    use super::{Opt, Parser, ParsingStyle};

    #[test]
    fn test_parser() {
        let args = &["-a", "b", "-cde", "arg", "-xfoo", "--long", "--opt=val",
            "--", "y", "-z"];

        let mut p = Parser::new(args, ParsingStyle::AllOptions);

        assert_matches!(p.next_opt(), Some(Opt::Short('a')));
        assert_matches!(p.next_opt(), Some(Opt::Free("b")));
        assert_matches!(p.next_opt(), Some(Opt::Short('c')));
        assert_matches!(p.next_opt(), Some(Opt::Short('d')));
        assert_matches!(p.next_opt(), Some(Opt::Short('e')));
        assert_matches!(p.next_arg(), Some("arg"));
        assert_matches!(p.next_opt(), Some(Opt::Short('x')));
        assert_matches!(p.next_arg(), Some("foo"));
        assert_matches!(p.next_opt(), Some(Opt::Long("long")));
        assert_matches!(p.next_opt(), Some(Opt::LongWithArg("opt", "val")));
        assert_matches!(p.next_opt(), Some(Opt::Free("y")));
        assert_matches!(p.next_opt(), Some(Opt::Free("-z")));
        assert_matches!(p.next_opt(), None);
    }

    #[test]
    fn test_parsing_style() {
        let args = &["-a", "b", "-c", "--d"];

        let mut p = Parser::new(args, ParsingStyle::AllOptions);

        assert_matches!(p.next_opt(), Some(Opt::Short('a')));
        assert_matches!(p.next_opt(), Some(Opt::Free("b")));
        assert_matches!(p.next_opt(), Some(Opt::Short('c')));
        assert_matches!(p.next_opt(), Some(Opt::Long("d")));
        assert_matches!(p.next_opt(), None);

        let mut p = Parser::new(args, ParsingStyle::StopAtFirstFree);

        assert_matches!(p.next_opt(), Some(Opt::Short('a')));
        assert_matches!(p.next_opt(), Some(Opt::Free("b")));
        assert_matches!(p.next_opt(), Some(Opt::Free("-c")));
        assert_matches!(p.next_opt(), Some(Opt::Free("--d")));
        assert_matches!(p.next_opt(), None);
    }
}
