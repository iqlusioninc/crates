extern crate gumdrop;
#[macro_use] extern crate gumdrop_derive;

use gumdrop::Options;

#[test]
fn test_hygiene() {
    // Define these aliases in local scope to ensure that generated code
    // is using absolute paths, i.e. `::std::result::Result`
    #[allow(dead_code)] type AsRef = ();
    #[allow(dead_code)] type Default = ();
    #[allow(dead_code)] type Option = ();
    #[allow(dead_code)] type Some = ();
    #[allow(dead_code)] type None = ();
    #[allow(dead_code)] type Options = ();
    #[allow(dead_code)] type Result = ();
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
    }

    // This is basically just a compile-pass test, but whatever.
    let empty: &[&str] = &[];
    let _ = Opts::parse_args_default(empty).unwrap();
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
fn test_opt_append() {
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
        #[options(no_short, help = "long option help")]
        very_very_long_option_with_very_very_long_name: bool,
    }

    assert_eq!(Opts::usage(),
"  -a, --alpha    alpha help
  --bravo BRAVO  bravo help
  -c             charlie help
  -d, --delta X  delta help
  --very-very-long-option-with-very-very-long-name
                 long option help");
}
