#[macro_use] extern crate assert_matches;
#[macro_use] extern crate gumdrop;

use gumdrop::Options;

const EMPTY: &'static [&'static str] = &[];

#[derive(Debug, Default, Options)]
struct NoOpts { }

macro_rules! is_err {
    ( $e:expr , |$ident:ident| $expr:expr ) => {
        let $ident = $e.map(|_| ()).unwrap_err().to_string();
        assert!($expr,
            "error {:?} does not match `{}`", $ident, stringify!($expr));
    };
    ( $e:expr , $str:expr ) => {
        assert_eq!($e.map(|_| ()).unwrap_err().to_string(), $str)
    };
}

#[test]
fn test_hygiene() {
    // Define these aliases in local scope to ensure that generated code
    // is using absolute paths, i.e. `::std::result::Result`
    #[allow(dead_code)] struct AsRef;
    #[allow(dead_code)] struct Default;
    #[allow(dead_code)] struct FromStr;
    #[allow(dead_code)] struct Option;
    #[allow(dead_code)] struct Some;
    #[allow(dead_code)] struct None;
    #[allow(dead_code)] struct Options;
    #[allow(dead_code)] struct Result;
    #[allow(dead_code)] struct Ok;
    #[allow(dead_code)] struct Err;
    #[allow(dead_code)] struct String;
    #[allow(dead_code)] struct ToString;
    #[allow(dead_code)] struct Vec;

    #[derive(Default, Options)]
    struct Opts {
        a: i32,
        b: ::std::string::String,
        c: ::std::option::Option<::std::string::String>,
        d: ::std::option::Option<i32>,
        e: ::std::vec::Vec<i32>,
        f: ::std::vec::Vec<::std::string::String>,
        g: ::std::option::Option<(i32, i32)>,

        #[options(command)]
        cmd: ::std::option::Option<Cmd>,
    }

    #[derive(Options)]
    enum Cmd {
        Foo(FooOpts),
        Bar(BarOpts),
    }

    #[derive(Default, Options)]
    struct FooOpts {
        #[options(free)]
        free: ::std::vec::Vec<::std::string::String>,
        a: i32,
    }

    #[derive(Default, Options)]
    struct BarOpts {
        #[options(free)]
        first: ::std::option::Option<::std::string::String>,
        #[options(free)]
        rest: ::std::vec::Vec<::std::string::String>,
        a: i32,
    }

    // This is basically just a compile-pass test, so whatever.
}

#[test]
fn test_command() {
    #[derive(Default, Options)]
    struct Opts {
        help: bool,

        #[options(command)]
        command: Option<Command>,
    }

    #[derive(Debug, Options)]
    enum Command {
        Foo(FooOpts),
        Bar(BarOpts),
        #[options(name = "bzzz")]
        Baz(NoOpts),
        FooBar(NoOpts),
        FooXYZ(NoOpts),
    }

    #[derive(Debug, Default, Options)]
    struct FooOpts {
        foo: Option<String>,
    }

    #[derive(Debug, Default, Options)]
    struct BarOpts {
        #[options(free)]
        free: Vec<String>,
    }

    let opts = Opts::parse_args_default(EMPTY).unwrap();
    assert_eq!(opts.command.is_none(), true);

    let opts = Opts::parse_args_default(&["-h"]).unwrap();
    assert_eq!(opts.help, true);
    assert_eq!(opts.command.is_none(), true);

    let opts = Opts::parse_args_default(&["-h", "foo", "--foo", "x"]).unwrap();
    assert_eq!(opts.help, true);
    let cmd = opts.command.unwrap();
    assert_matches!(cmd, Command::Foo(FooOpts{foo: Some(ref foo)}) if foo == "x");

    let opts = Opts::parse_args_default(&["--", "foo"]).unwrap();
    assert_eq!(opts.help, false);
    let cmd = opts.command.unwrap();
    assert_matches!(cmd, Command::Foo(_));

    let opts = Opts::parse_args_default(&["bar", "free"]).unwrap();
    let cmd = opts.command.unwrap();
    assert_matches!(cmd, Command::Bar(ref bar) if bar.free == ["free"]);

    let opts = Opts::parse_args_default(&["bzzz"]).unwrap();
    let cmd = opts.command.unwrap();
    assert_matches!(cmd, Command::Baz(_));

    let opts = Opts::parse_args_default(&["foo-bar"]).unwrap();
    let cmd = opts.command.unwrap();
    assert_matches!(cmd, Command::FooBar(_));

    let opts = Opts::parse_args_default(&["foo-x-y-z"]).unwrap();
    let cmd = opts.command.unwrap();
    assert_matches!(cmd, Command::FooXYZ(_));

    is_err!(Opts::parse_args_default(&["foo", "-h"]),
        "unrecognized option `-h`");
    is_err!(Opts::parse_args_default(&["baz"]),
        "unrecognized command `baz`");
}

#[test]
fn test_command_name() {
    #[derive(Default, Options)]
    struct Opts {
        help: bool,

        #[options(command)]
        command: Option<Command>,
    }

    #[derive(Debug, Options)]
    enum Command {
        Foo(NoOpts),
        Bar(NoOpts),
        #[options(name = "bzzz")]
        Baz(NoOpts),
        BoopyDoop(NoOpts),
    }


    let opts = Opts::parse_args_default(&["foo"]).unwrap();
    assert_matches!(opts.command_name(), Some("foo"));

    let opts = Opts::parse_args_default(&["bar"]).unwrap();
    assert_matches!(opts.command_name(), Some("bar"));

    let opts = Opts::parse_args_default(&["bzzz"]).unwrap();
    assert_matches!(opts.command_name(), Some("bzzz"));

    let opts = Opts::parse_args_default(&["boopy-doop"]).unwrap();
    assert_matches!(opts.command_name(), Some("boopy-doop"));
}

#[test]
fn test_command_usage() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(help = "help me!")]
        help: bool,

        #[options(command)]
        command: Option<Command>,
    }

    #[derive(Options)]
    enum Command {
        #[options(help = "foo help")]
        Foo(NoOpts),
        #[options(help = "bar help")]
        Bar(NoOpts),
        #[options(help = "baz help")]
        #[options(name = "bzzz")]
        Baz(NoOpts),
    }

    assert_eq!(Command::usage(), &"
  foo   foo help
  bar   bar help
  bzzz  baz help"
        // Skip leading newline
        [1..]);

    assert_eq!(Command::command_list(), Some(Command::usage()));
    assert_eq!(Opts::command_list(), Some(Command::usage()));
}

#[test]
fn test_opt_bool() {
    #[derive(Default, Options)]
    struct Opts {
        switch: bool,
    }

    let opts = Opts::parse_args_default(&["--switch"]).unwrap();
    assert_eq!(opts.switch, true);

    let opts = Opts::parse_args_default(&["-s"]).unwrap();
    assert_eq!(opts.switch, true);

    is_err!(Opts::parse_args_default(&["--switch=x"]),
        "option `--switch` does not accept an argument");
}

#[test]
fn test_opt_string() {
    #[derive(Default, Options)]
    struct Opts {
        foo: String,
    }

    let opts = Opts::parse_args_default(&["--foo", "value"]).unwrap();
    assert_eq!(opts.foo, "value");

    let opts = Opts::parse_args_default(&["-f", "value"]).unwrap();
    assert_eq!(opts.foo, "value");

    let opts = Opts::parse_args_default(&["-fvalue"]).unwrap();
    assert_eq!(opts.foo, "value");
}

#[test]
fn test_opt_int() {
    #[derive(Default, Options)]
    struct Opts {
        number: i32,
    }

    let opts = Opts::parse_args_default(&["--number", "123"]).unwrap();
    assert_eq!(opts.number, 123);

    let opts = Opts::parse_args_default(&["-n", "123"]).unwrap();
    assert_eq!(opts.number, 123);

    let opts = Opts::parse_args_default(&["-n123"]).unwrap();
    assert_eq!(opts.number, 123);

    is_err!(Opts::parse_args_default(&["-nfail"]),
        |e| e.starts_with("invalid argument to option `-n`: "));
    is_err!(Opts::parse_args_default(&["--number", "fail"]),
        |e| e.starts_with("invalid argument to option `--number`: "));
    is_err!(Opts::parse_args_default(&["--number=fail"]),
        |e| e.starts_with("invalid argument to option `--number`: "));
}

#[test]
fn test_opt_tuple() {
    #[derive(Default, Options)]
    struct Opts {
        alpha: (i32, i32),
        bravo: Option<(i32, i32, i32)>,
        charlie: Vec<(i32, i32, i32, i32)>,
        #[options(free)]
        free: Vec<String>,
    }

    let opts = Opts::parse_args_default(&[
        "--alpha", "1", "2",
        "--bravo", "11", "12", "13",
        "--charlie", "21", "22", "23", "24",
        "--charlie", "31", "32", "33", "34",
        "free",
    ]).unwrap();

    assert_eq!(opts.alpha, (1, 2));
    assert_eq!(opts.bravo, Some((11, 12, 13)));
    assert_eq!(opts.charlie, vec![
        (21, 22, 23, 24),
        (31, 32, 33, 34),
    ]);
    assert_eq!(opts.free, vec!["free".to_owned()]);
}

#[test]
fn test_opt_tuple_error() {
    #[derive(Default, Options)]
    struct Opts {
        foo: Option<(i32, i32)>,
    }

    is_err!(Opts::parse_args_default(&["--foo"]),
        "insufficient arguments to option `--foo`: expected 2; found 0");
    is_err!(Opts::parse_args_default(&["--foo=0", "1"]),
        "option `--foo` expects 2 arguments; found 1");
    is_err!(Opts::parse_args_default(&["--foo", "0"]),
        "insufficient arguments to option `--foo`: expected 2; found 1");
}

#[test]
fn test_opt_push() {
    #[derive(Default, Options)]
    struct Opts {
        thing: Vec<String>,
    }

    let opts = Opts::parse_args_default(EMPTY).unwrap();
    assert!(opts.thing.is_empty());

    let opts = Opts::parse_args_default(
        &["-t", "a", "-tb", "--thing=c", "--thing", "d"]).unwrap();
    assert_eq!(opts.thing, ["a", "b", "c", "d"]);
}

#[test]
fn test_opt_count() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(count)]
        number: i32,
    }

    let opts = Opts::parse_args_default(EMPTY).unwrap();
    assert_eq!(opts.number, 0);

    let opts = Opts::parse_args_default(&["--number"]).unwrap();
    assert_eq!(opts.number, 1);

    let opts = Opts::parse_args_default(&["-nnn"]).unwrap();
    assert_eq!(opts.number, 3);
}

#[test]
fn test_opt_long() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(long = "thing", no_short)]
        foo: bool,
    }

    let opts = Opts::parse_args_default(&["--thing"]).unwrap();
    assert_eq!(opts.foo, true);

    is_err!(Opts::parse_args_default(&["-f"]),
        "unrecognized option `-f`");
    is_err!(Opts::parse_args_default(&["--foo"]),
        "unrecognized option `--foo`");
}

#[test]
fn test_opt_short() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(short = "x", no_long)]
        foo: bool,
    }

    let opts = Opts::parse_args_default(&["-x"]).unwrap();
    assert_eq!(opts.foo, true);

    is_err!(Opts::parse_args_default(&["-f"]),
        "unrecognized option `-f`");
    is_err!(Opts::parse_args_default(&["--foo"]),
        "unrecognized option `--foo`");
}

#[test]
fn test_opt_short_override() {
    // Ensures that the generated code sees the manual assignment of short
    // option for `option_1` before generating a short option for `option_0`.
    // Thus, giving `option_0` an automatic short option of `O`,
    // rather than causing a collision.
    #[derive(Default, Options)]
    struct Opts {
        #[options(no_long)]
        option_0: bool,
        #[options(short = "o", no_long)]
        option_1: bool,
    }

    let opts = Opts::parse_args_default(&["-o"]).unwrap();
    assert_eq!(opts.option_0, false);
    assert_eq!(opts.option_1, true);

    let opts = Opts::parse_args_default(&["-O"]).unwrap();
    assert_eq!(opts.option_0, true);
    assert_eq!(opts.option_1, false);
}

#[test]
fn test_opt_free() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(free)]
        free: Vec<String>,
    }

    let opts = Opts::parse_args_default(&["a", "b", "c"]).unwrap();
    assert_eq!(opts.free, ["a", "b", "c"]);
}

#[test]
fn test_opt_no_free() {
    #[derive(Default, Options)]
    struct Opts {
    }

    assert!(Opts::parse_args_default(EMPTY).is_ok());
    is_err!(Opts::parse_args_default(&["a"]),
        "unexpected free argument `a`");
}

#[test]
fn test_typed_free() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(free)]
        free: Vec<i32>,
    }

    let opts = Opts::parse_args_default(&["1", "2", "3"]).unwrap();
    assert_eq!(opts.free, [1, 2, 3]);
}

#[test]
fn test_multi_free() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(free, help = "alpha help")]
        alpha: u32,
        #[options(free, help = "bravo help")]
        bravo: Option<String>,
        #[options(free, help = "charlie help")]
        charlie: Option<u32>,
    }

    let opts = Opts::parse_args_default(EMPTY).unwrap();

    assert_eq!(opts.alpha, 0);
    assert_eq!(opts.bravo, None);
    assert_eq!(opts.charlie, None);

    let opts = Opts::parse_args_default(&["1"]).unwrap();

    assert_eq!(opts.alpha, 1);
    assert_eq!(opts.bravo, None);
    assert_eq!(opts.charlie, None);

    let opts = Opts::parse_args_default(&["1", "two", "3"]).unwrap();

    assert_eq!(opts.alpha, 1);
    assert_eq!(opts.bravo, Some("two".to_owned()));
    assert_eq!(opts.charlie, Some(3));

    is_err!(Opts::parse_args_default(&["1", "two", "3", "4"]),
        "unexpected free argument `4`");

    assert_eq!(Opts::usage(), &"
Positional arguments:
  alpha    alpha help
  bravo    bravo help
  charlie  charlie help"
        // Skip leading newline
        [1..]);

    #[derive(Default, Options)]
    struct ManyOpts {
        #[options(free, help = "alpha help")]
        alpha: u32,
        #[options(free, help = "bravo help")]
        bravo: Option<String>,
        #[options(free, help = "charlie help")]
        charlie: Option<u32>,
        #[options(free)]
        rest: Vec<String>,
    }

    let opts = ManyOpts::parse_args_default(EMPTY).unwrap();

    assert_eq!(opts.alpha, 0);
    assert_eq!(opts.bravo, None);
    assert_eq!(opts.charlie, None);
    assert_eq!(opts.rest, Vec::<String>::new());

    let opts = ManyOpts::parse_args_default(&["1", "two", "3", "4", "five", "VI"]).unwrap();

    assert_eq!(opts.alpha, 1);
    assert_eq!(opts.bravo, Some("two".to_owned()));
    assert_eq!(opts.charlie, Some(3));
    assert_eq!(opts.rest, vec!["4".to_owned(), "five".to_owned(), "VI".to_owned()]);
}

#[test]
fn test_usage() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(help = "alpha help")]
        alpha: bool,
        #[options(no_short, help = "bravo help")]
        bravo: String,
        #[options(no_long, help = "charlie help")]
        charlie: bool,
        #[options(help = "delta help", meta = "X")]
        delta: i32,
        #[options(help = "echo help", meta = "Y")]
        echo: Vec<String>,
        #[options(no_short, help = "long option help")]
        very_very_long_option_with_very_very_long_name: bool,
    }

    assert_eq!(Opts::usage(), &"
Optional arguments:
  -a, --alpha    alpha help
  --bravo BRAVO  bravo help
  -c             charlie help
  -d, --delta X  delta help
  -e, --echo Y   echo help
  --very-very-long-option-with-very-very-long-name
                 long option help"
        // Skip leading newline
        [1..]);

    #[derive(Default, Options)]
    struct TupleOpts {
        #[options(help = "alpha help")]
        alpha: (),
        #[options(help = "bravo help")]
        bravo: (i32,),
        #[options(help = "charlie help")]
        charlie: (i32, i32),
        #[options(help = "delta help")]
        delta: (i32, i32, i32),
        #[options(help = "echo help")]
        echo: (i32, i32, i32, i32),
    }

    assert_eq!(TupleOpts::usage(), &"
Optional arguments:
  -a, --alpha        alpha help
  -b, --bravo BRAVO  bravo help
  -c, --charlie CHARLIE VALUE
                     charlie help
  -d, --delta DELTA VALUE0 VALUE1
                     delta help
  -e, --echo ECHO VALUE0 VALUE1 VALUE2
                     echo help"
        // Skip leading newline
        [1..]);

    #[derive(Default, Options)]
    struct FreeOpts {
        #[options(free, help = "a help")]
        a: u32,
        #[options(free, help = "b help")]
        b: u32,
        #[options(free, help = "c help")]
        c: u32,

        #[options(help = "option help")]
        option: bool,
    }

    assert_eq!(FreeOpts::usage(), &"
Positional arguments:
  a             a help
  b             b help
  c             c help

Optional arguments:
  -o, --option  option help"
        // Skip leading newline
        [1..]);
}

#[test]
fn test_help_flag() {
    #[derive(Default, Options)]
    struct Opts {
        help: bool,
    }

    let opts = Opts::parse_args_default(EMPTY).unwrap();
    assert_eq!(opts.help_requested(), false);

    let opts = Opts::parse_args_default(&["--help"]).unwrap();
    assert_eq!(opts.help_requested(), true);
}

#[test]
fn test_no_help_flag() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(no_help_flag)]
        help: bool,
    }

    let opts = Opts::parse_args_default(&["--help"]).unwrap();
    assert_eq!(opts.help_requested(), false);
}

#[test]
fn test_many_help_flags() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(help_flag)]
        help: bool,
        #[options(help_flag)]
        halp: bool,
        #[options(help_flag)]
        help_please: bool,
    }

    let opts = Opts::parse_args_default(EMPTY).unwrap();
    assert_eq!(opts.help_requested(), false);

    let opts = Opts::parse_args_default(&["--help"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts::parse_args_default(&["--halp"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts::parse_args_default(&["--help-please"]).unwrap();
    assert_eq!(opts.help_requested(), true);
}

#[test]
fn test_help_flag_command() {
    #[derive(Default, Options)]
    struct Opts {
        help: bool,

        #[options(command)]
        cmd: Option<Cmd>,
    }

    #[derive(Default, Options)]
    struct Opts2 {
        #[options(command)]
        cmd: Option<Cmd>,
    }

    #[derive(Default, Options)]
    struct Opts3 {
        help: bool,
        #[options(help_flag)]
        help2: bool,

        #[options(command)]
        cmd: Option<Cmd>,
    }

    #[derive(Options)]
    enum Cmd {
        Foo(CmdOpts),
        Bar(CmdOpts),
        Baz(CmdOpts),
    }

    #[derive(Default, Options)]
    struct CmdOpts {
        help: bool,
    }

    let opts = Opts::parse_args_default(EMPTY).unwrap();
    assert_eq!(opts.help_requested(), false);

    let opts = Opts::parse_args_default(&["-h"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts::parse_args_default(&["foo", "-h"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts::parse_args_default(&["bar", "-h"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts::parse_args_default(&["baz", "-h"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts2::parse_args_default(EMPTY).unwrap();
    assert_eq!(opts.help_requested(), false);

    let opts = Opts3::parse_args_default(EMPTY).unwrap();
    assert_eq!(opts.help_requested(), false);
}

#[test]
fn test_type_attrs() {
    #[derive(Default, Options)]
    #[options(no_help_flag, no_short, no_long)]
    struct Opts {
        #[options(long = "help")]
        help: bool,
        #[options(long = "foo")]
        foo: bool,
        #[options(short = "b")]
        bar: bool,
    }

    is_err!(Opts::parse_args_default(&["-f"]),
        "unrecognized option `-f`");
    is_err!(Opts::parse_args_default(&["--bar"]),
        "unrecognized option `--bar`");
    is_err!(Opts::parse_args_default(&["-h"]),
        "unrecognized option `-h`");

    let opts = Opts::parse_args_default(&["--help"]).unwrap();
    assert_eq!(opts.help, true);
    assert_eq!(opts.help_requested(), false);

    let opts = Opts::parse_args_default(&["--foo"]).unwrap();
    assert_eq!(opts.foo, true);

    let opts = Opts::parse_args_default(&["-b"]).unwrap();
    assert_eq!(opts.bar, true);

    #[derive(Default, Options)]
    #[options(no_short)]
    struct Opts2 {
        foo: bool,
        #[options(short = "b")]
        bar: bool,
    }

    is_err!(Opts2::parse_args_default(&["-f"]),
        "unrecognized option `-f`");

    let opts = Opts2::parse_args_default(&["--foo", "-b"]).unwrap();
    assert_eq!(opts.foo, true);
    assert_eq!(opts.bar, true);

    let opts = Opts2::parse_args_default(&["--bar"]).unwrap();
    assert_eq!(opts.bar, true);

    #[derive(Default, Options)]
    #[options(no_long)]
    struct Opts3 {
        foo: bool,
        #[options(long = "bar")]
        bar: bool,
    }

    is_err!(Opts3::parse_args_default(&["--foo"]),
        "unrecognized option `--foo`");

    let opts = Opts3::parse_args_default(&["--bar"]).unwrap();
    assert_eq!(opts.bar, true);

    let opts = Opts3::parse_args_default(&["-f", "-b"]).unwrap();
    assert_eq!(opts.foo, true);
    assert_eq!(opts.bar, true);

    #[derive(Default, Options)]
    #[options(no_help_flag)]
    struct Opts4 {
        #[options(help_flag)]
        help: bool,
    }

    let opts = Opts4::parse_args_default(&["-h"]).unwrap();
    assert_eq!(opts.help, true);
    assert_eq!(opts.help_requested(), true);

    #[derive(Default, Options)]
    #[options(required)]
    struct Opts5 {
        #[options(no_long)]
        foo: i32,
        #[options(not_required)]
        bar: i32,
    }

    is_err!(Opts5::parse_args_default(EMPTY),
        "missing required option `-f`");

    let opts = Opts5::parse_args_default(&["-f", "1"]).unwrap();
    assert_eq!(opts.foo, 1);
    assert_eq!(opts.bar, 0);

    let opts = Opts5::parse_args_default(&["-f", "1", "--bar", "2"]).unwrap();
    assert_eq!(opts.foo, 1);
    assert_eq!(opts.bar, 2);
}

#[test]
fn test_required() {
    #[derive(Default, Options)]
    struct Opts {
        #[options(required)]
        foo: i32,
        optional: i32,
    }

    #[derive(Default, Options)]
    struct Opts2 {
        #[options(command, required)]
        command: Option<Cmd>,
        optional: i32,
    }

    #[derive(Options)]
    enum Cmd {
        Foo(NoOpts),
    }

    #[derive(Default, Options)]
    struct Opts3 {
        #[options(free, required)]
        bar: i32,
        optional: i32,
    }

    is_err!(Opts::parse_args_default(EMPTY),
        "missing required option `--foo`");
    is_err!(Opts2::parse_args_default(EMPTY),
        "missing required command");
    is_err!(Opts3::parse_args_default(EMPTY),
        "missing required free argument");

    let opts = Opts::parse_args_default(&["-f", "1"]).unwrap();
    assert_eq!(opts.foo, 1);
    let opts = Opts::parse_args_default(&["-f1"]).unwrap();
    assert_eq!(opts.foo, 1);
    let opts = Opts::parse_args_default(&["--foo", "1"]).unwrap();
    assert_eq!(opts.foo, 1);
    let opts = Opts::parse_args_default(&["--foo=1"]).unwrap();
    assert_eq!(opts.foo, 1);

    let opts = Opts2::parse_args_default(&["foo"]).unwrap();
    assert!(opts.command.is_some());

    let opts = Opts3::parse_args_default(&["1"]).unwrap();
    assert_eq!(opts.bar, 1);
}

#[test]
fn test_parse() {
    use std::str::FromStr;

    #[derive(Default, Options)]
    struct Opts {
        #[options(help = "foo", parse(from_str = "parse_foo"))]
        foo: Option<Foo>,
        #[options(help = "bar", parse(try_from_str = "parse_bar"))]
        bar: Option<Bar>,
    }

    #[derive(Debug)]
    struct Foo(String);
    #[derive(Debug)]
    struct Bar(u32);

    fn parse_foo(s: &str) -> Foo { Foo(s.to_owned()) }
    fn parse_bar(s: &str) -> Result<Bar, <u32 as FromStr>::Err> { s.parse().map(Bar) }

    let opts = Opts::parse_args_default(&["-ffoo", "--bar=123"]).unwrap();
    assert_matches!(opts.foo, Some(Foo(ref s)) if s == "foo");
    assert_matches!(opts.bar, Some(Bar(123)));

    is_err!(Opts::parse_args_default(&["-b", "xyz"]),
        |e| e.starts_with("invalid argument to option `-b`: "));
}
