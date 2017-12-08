#[macro_use] extern crate assert_matches;
extern crate gumdrop;
#[macro_use] extern crate gumdrop_derive;

use gumdrop::Options;

#[test]
fn test_hygiene() {
    // Define these aliases in local scope to ensure that generated code
    // is using absolute paths, i.e. `::std::result::Result`
    #[allow(dead_code)] type AsRef = ();
    #[allow(dead_code)] type Default = ();
    #[allow(dead_code)] type FromStr = ();
    #[allow(dead_code)] type Option = ();
    #[allow(dead_code)] type Some = ();
    #[allow(dead_code)] type None = ();
    #[allow(dead_code)] type Options = ();
    #[allow(dead_code)] type Result = ();
    #[allow(dead_code)] type Ok = ();
    #[allow(dead_code)] type Err = ();
    #[allow(dead_code)] type String = ();
    #[allow(dead_code)] type ToString = ();
    #[allow(dead_code)] type Vec = ();

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
    }

    #[derive(Default, Options)]
    struct FooOpts {
        #[options(free)]
        free: ::std::vec::Vec<::std::string::String>,
        a: i32,
    }

    // This is basically just a compile-pass test, but whatever.
    let empty: &[&str] = &[];
    let _ = Opts::parse_args_default(empty).unwrap();
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

    #[derive(Debug, Default, Options)]
    struct NoOpts { }

    let empty: &[&str] = &[];
    let opts = Opts::parse_args_default(empty).unwrap();
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

    assert!(Opts::parse_args_default(&["foo", "-h"]).is_err());
    assert!(Opts::parse_args_default(&["baz"]).is_err());
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

    #[derive(Debug, Default, Options)]
    struct NoOpts { }

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
fn test_opt_bool() {
    #[derive(Default, Options)]
    struct Opts {
        switch: bool,
    }

    let opts = Opts::parse_args_default(&["--switch"]).unwrap();
    assert_eq!(opts.switch, true);

    let opts = Opts::parse_args_default(&["-s"]).unwrap();
    assert_eq!(opts.switch, true);

    assert!(Opts::parse_args_default(&["--switch=x"]).is_err());
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

    assert!(Opts::parse_args_default(&["--number", "fail"]).is_err());
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

    assert!(Opts::parse_args_default(&["--foo"]).is_err());
    assert!(Opts::parse_args_default(&["--foo=0", "1"]).is_err());
    assert!(Opts::parse_args_default(&["--foo", "0"]).is_err());
}

#[test]
fn test_opt_push() {
    #[derive(Default, Options)]
    struct Opts {
        thing: Vec<String>,
    }

    let empty: &[&str] = &[];
    let opts = Opts::parse_args_default(empty).unwrap();
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

    let empty: &[&str] = &[];
    let opts = Opts::parse_args_default(empty).unwrap();
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

    assert!(Opts::parse_args_default(&["-f"]).is_err());
    assert!(Opts::parse_args_default(&["--foo"]).is_err());
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

    assert!(Opts::parse_args_default(&["-f"]).is_err());
    assert!(Opts::parse_args_default(&["--foo"]).is_err());
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

    let empty: &[&str] = &[];
    assert!(Opts::parse_args_default(empty).is_ok());
    assert!(Opts::parse_args_default(&["a"]).is_err());
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
}

#[test]
fn test_help_flag() {
    #[derive(Default, Options)]
    struct Opts {
        help: bool,
    }

    let empty: &[&str] = &[];
    let opts = Opts::parse_args_default(empty).unwrap();
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

    let empty: &[&str] = &[];
    let opts = Opts::parse_args_default(empty).unwrap();
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

    let empty: &[&str] = &[];
    let opts = Opts::parse_args_default(empty).unwrap();
    assert_eq!(opts.help_requested(), false);

    let opts = Opts::parse_args_default(&["-h"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts::parse_args_default(&["foo", "-h"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts::parse_args_default(&["bar", "-h"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts::parse_args_default(&["baz", "-h"]).unwrap();
    assert_eq!(opts.help_requested(), true);

    let opts = Opts2::parse_args_default(empty).unwrap();
    assert_eq!(opts.help_requested(), false);

    let opts = Opts3::parse_args_default(empty).unwrap();
    assert_eq!(opts.help_requested(), false);
}
