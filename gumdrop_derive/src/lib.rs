//! Provides `derive(Options)` for `gumdrop` crate
//!
//! # `derive(Options)`
//!
//! `derive(Options)` generates an implementation of the trait `Options`,
//! creating an option for each field of the decorated `struct`.
//!
//! See the `gumdrop` [documentation](https://docs.rs/gumdrop/) for an example
//! of its usage.
//!
//! ## `options` attribute
//!
//! Behavior of `derive(Options)` can be controlled by adding `#[options(...)]`
//! attributes to one or more fields within a decorated struct.
//!
//! Supported items are:
//!
//! * `count` marks a field as a counter value. The field will be incremented
//!   each time the option appears in the arguments, i.e. `field += 1;`
//! * `free` marks a field as the free argument container. Non-option arguments
//!   will be appended to this field using its `push` method.
//! * `short = "?"` sets the short option name to the given character
//! * `no_short` prevents a short option from being assigned to the field
//! * `long = "..."` sets the long option name to the given string
//! * `no_long` prevents a long option from being assigned to the field
//! * `help = "..."` sets help text returned from the `Options::usage` method
//! * `meta = "..."` sets the meta variable displayed in usage for options
//!   which accept an argument

#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use std::cmp::{max, min};
use std::iter::repeat;

use proc_macro::TokenStream;

use quote::{Tokens, ToTokens};
use syn::{
    Attribute, AttrStyle, Body, Ident, Lit,
    MetaItem, NestedMetaItem, Ty, VariantData,
};

#[proc_macro_derive(Options, attributes(options))]
pub fn derive_options(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string())
        .expect("parse_derive_input");

    let fields = match ast.body {
        Body::Enum(_) => panic!("cannot derive Options for enum types"),
        Body::Struct(VariantData::Unit) =>
            panic!("cannot derive Options for unit struct types"),
        Body::Struct(VariantData::Tuple(_)) =>
            panic!("cannot derive Options for tuple struct types"),
        Body::Struct(VariantData::Struct(fields)) => fields
    };

    let mut pattern = Vec::new();
    let mut handle_opt = Vec::new();
    let mut short_names = Vec::new();
    let mut long_names = Vec::new();
    let mut free = None;
    let mut options = Vec::new();

    for field in fields {
        let mut opts = parse_attrs(&field.attrs);

        let ident = field.ident.as_ref().unwrap();

        if opts.free {
            if free.is_some() {
                panic!("duplicate declaration of `free` field");
            }

            free = Some(ident.clone());
            continue;
        }

        if opts.long.is_none() && !opts.no_long {
            opts.long = Some(make_long_name(ident.as_ref()));
        }

        if let Some(ref long) = opts.long {
            valid_long_name(long, &long_names);
            long_names.push(long.clone());
        }

        if let Some(short) = opts.short {
            valid_short_name(short, &short_names);
            short_names.push(short);
        }

        let action = if opts.count {
            Action::Count
        } else {
            infer_action(&field.ty).unwrap_or(Action::ParseArg)
        };

        if action.takes_arg() {
            if opts.meta.is_none() {
                opts.meta = Some(make_meta(ident.as_ref()));
            }
        } else if opts.meta.is_some() {
            panic!("`meta` value is invalid for option `{}`", ident.as_ref());
        }

        options.push(Opt{
            field: ident.clone(),
            action: action,
            long: opts.long.take(),
            short: opts.short,
            no_short: opts.no_short,
            meta: opts.meta.take(),
            help: opts.help.take(),
        });
    }

    // Assign short names after checking all options.
    // Thus, manual short names will take priority over automatic ones.
    for opt in &mut options {
        if opt.short.is_none() && !opt.no_short {
            let short = make_short_name(opt.field.as_ref(), &short_names);

            if let Some(short) = short {
                short_names.push(short);
            }

            opt.short = short;
        }
    }

    for opt in &options {
        let pat = match (opt.long.as_ref(), opt.short) {
            (Some(long), Some(short)) => quote!{
                ::gumdrop::Opt::Long(#long) | ::gumdrop::Opt::Short(#short)
            },
            (Some(long), None) => quote!{
                ::gumdrop::Opt::Long(#long)
            },
            (None, Some(short)) => quote!{
                ::gumdrop::Opt::Short(#short)
            },
            (None, None) => {
                panic!("option `{}` has no long or short flags", opt.field.as_ref());
            }
        };

        pattern.push(pat);
        handle_opt.push(make_action(&opt.field, opt.action));

        if let Some(ref long) = opt.long {
            let (pat, handle) = if opt.action.takes_arg() {
                (quote!{ ::gumdrop::Opt::LongWithArg(#long, arg) },
                    make_action_arg(&opt.field, opt.action))
            } else {
                (quote!{ ::gumdrop::Opt::LongWithArg(#long, _) },
                    quote!{ return ::std::result::Result::Err(
                        ::gumdrop::Error::unexpected_argument(opt)) })
            };

            pattern.push(pat);
            handle_opt.push(handle);
        }
    }

    let usage = Lit::from(make_usage(&options));

    let handle_free = if let Some(free) = free {
        quote!{
            _result.#free.push(::std::string::String::from(free));
        }
    } else {
        quote!{
            return ::std::result::Result::Err(
                ::gumdrop::Error::unexpected_free(free));
        }
    };

    let name = ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expr = quote!{
        impl #impl_generics ::gumdrop::Options for #name #ty_generics #where_clause {
            fn parse_args<__S: ::std::convert::AsRef<str>>(args: &[__S],
                    style: ::gumdrop::ParsingStyle)
                    -> ::std::result::Result<Self, ::gumdrop::Error> {
                let mut _result = <Self as ::std::default::Default>::default();
                let mut parser = ::gumdrop::Parser::new(args, style);

                while let ::std::option::Option::Some(opt) = parser.next_opt() {
                    match opt {
                        #( #pattern => { #handle_opt } )*
                        ::gumdrop::Opt::Free(free) => {
                            #handle_free
                        }
                        _ => {
                            return ::std::result::Result::Err(
                                ::gumdrop::Error::unrecognized_option(opt));
                        }
                    }
                }

                Ok(_result)
            }

            fn usage() -> &'static str {
                #usage
            }
        }
    };

    expr.to_string().parse().expect("parse quote!")
}

#[derive(Copy, Clone, Debug)]
enum Action {
    /// Append a parsed arg to a Vec
    Append,
    /// Increase count
    Count,
    /// Parse arg and set field
    ParseArg,
    /// Parse arg and set `Option<T>` field
    ParseArgOption,
    /// Set field to `true`
    Switch,
}

impl Action {
    fn takes_arg(&self) -> bool {
        use self::Action::*;

        match *self {
            Append | ParseArg | ParseArgOption => true,
            _ => false
        }
    }
}

fn infer_action(ty: &Ty) -> Option<Action> {
    match *ty {
        Ty::Path(_, ref path) => {
            match path.segments.last().unwrap().ident.as_ref() {
                "bool" => Some(Action::Switch),
                "Vec" => Some(Action::Append),
                "Option" => Some(Action::ParseArgOption),
                _ => None
            }
        }
        _ => None
    }
}

#[derive(Debug, Default)]
struct AttrOpts {
    long: Option<String>,
    short: Option<char>,
    free: bool,
    count: bool,
    no_short: bool,
    no_long: bool,
    help: Option<String>,
    meta: Option<String>,
}

impl AttrOpts {
    fn check(&self) {
        if self.free {
            if self.long.is_some() { panic!("`free` and `long` are mutually exclusive"); }
            if self.short.is_some() { panic!("`free` and `short` are mutually exclusive"); }
            if self.count { panic!("`free` and `count` are mutually exclusive"); }
            if self.no_short { panic!("`free` and `no_short` are mutually exclusive"); }
            if self.no_long { panic!("`free` and `no_long` are mutually exclusive"); }
            if self.help.is_some() { panic!("`free` and `help` are mutually exclusive"); }
            if self.meta.is_some() { panic!("`free` and `meta` are mutually exclusive"); }
        }

        if self.no_short && self.short.is_some() {
            panic!("`no_short` and `short` are mutually exclusive");
        }

        if self.no_long && self.long.is_some() {
            panic!("`no_long` and `long` are mutually exclusive");
        }
    }
}

#[derive(Debug)]
struct Opt {
    field: Ident,
    action: Action,
    long: Option<String>,
    short: Option<char>,
    no_short: bool,
    help: Option<String>,
    meta: Option<String>,
}

const MIN_WIDTH: usize = 8;
const MAX_WIDTH: usize = 30;

impl Opt {
    fn width(&self) -> usize {
        let short = self.short.map_or(0, |_| 1 + 1); // '-' + char
        let long = self.long.as_ref().map_or(0, |s| s.len() + 2); // "--" + str
        let sep = if short == 0 || long == 0 { 0 } else { 2 };
        let meta = self.meta.as_ref().map_or(0, |s| s.len() + 1); // ' ' + meta

        2 + short + long + sep + meta + 2 // total + spaces before and after
    }
}

fn parse_attrs(attrs: &[Attribute]) -> AttrOpts {
    let mut opts = AttrOpts::default();

    for attr in attrs {
        if attr.style == AttrStyle::Outer && attr.value.name() == "options" {
            match attr.value {
                MetaItem::Word(_) =>
                    panic!("#[options] is not a valid attribute"),
                MetaItem::NameValue(..) =>
                    panic!("#[options = ...] is not a valid attribute"),
                MetaItem::List(_, ref items) => {
                    for item in items {
                        match *item {
                            NestedMetaItem::Literal(_) =>
                                panic!("unexpected meta item `{}`", tokens_str(item)),
                            NestedMetaItem::MetaItem(ref item) => {
                                match *item {
                                    MetaItem::Word(ref w) => match w.as_ref() {
                                        "free" => opts.free = true,
                                        "count" => opts.count = true,
                                        "no_short" => opts.no_short = true,
                                        "no_long" => opts.no_long = true,
                                        _ => panic!("unexpected meta item `{}`", tokens_str(item))
                                    },
                                    MetaItem::List(..) => panic!("unexpected meta item `{}`", tokens_str(item)),
                                    MetaItem::NameValue(ref name, ref value) => {
                                        match name.as_ref() {
                                            "long" => opts.long = Some(lit_str(value)),
                                            "short" => opts.short = Some(lit_char(value)),
                                            "help" => opts.help = Some(lit_str(value)),
                                            "meta" => opts.meta = Some(lit_str(value)),
                                            _ => panic!("unexpected meta item `{}`", tokens_str(item))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    opts.check();

    opts
}

fn lit_str(lit: &Lit) -> String {
    match *lit {
        Lit::Str(ref s, _) => s.clone(),
        _ => panic!("unexpected literal `{}`", tokens_str(lit))
    }
}

fn lit_char(lit: &Lit) -> char {
    match *lit {
        // Character literals in attributes are not necessarily allowed
        Lit::Str(ref s, _) => {
            let mut chars = s.chars();

            let res = chars.next().expect("expected one-char string literal");
            if chars.next().is_some() {
                panic!("expected one-char string literal");
            }

            res
        }
        Lit::Char(ch) => ch,
        _ => panic!("unexpected literal `{}`", tokens_str(lit))
    }
}

fn tokens_str<T: ToTokens>(t: &T) -> String {
    let mut tok = Tokens::new();
    t.to_tokens(&mut tok);
    tok.into_string()
}

fn make_action(ident: &Ident, action: Action) -> Tokens {
    use self::Action::*;

    match action {
        Append => {
            let act = make_action_arg(ident, action);

            quote!{
                let arg = parser.next_arg()
                    .ok_or_else(|| ::gumdrop::Error::missing_argument(opt))?;
                #act
            }
        }
        Count => quote!{
            _result.#ident += 1;
        },
        ParseArg => {
            let act = make_action_arg(ident, action);
            quote!{
                let arg = parser.next_arg()
                    .ok_or_else(|| ::gumdrop::Error::missing_argument(opt))?;
                #act
            }
        }
        ParseArgOption => {
            let act = make_action_arg(ident, action);
            quote!{
                let arg = parser.next_arg()
                    .ok_or_else(|| ::gumdrop::Error::missing_argument(opt))?;
                #act
            }
        }
        Switch => quote!{
            _result.#ident = true;
        }
    }
}

fn make_action_arg(ident: &Ident, action: Action) -> Tokens {
    use self::Action::*;

    match action {
        Append => quote!{
            match ::std::str::FromStr::from_str(arg) {
                ::std::result::Result::Ok(v) => _result.#ident.push(v),
                ::std::result::Result::Err(ref e) =>
                    return ::std::result::Result::Err(
                        ::gumdrop::Error::failed_parse(opt,
                            ::std::string::ToString::to_string(e)))
            }
        },
        ParseArg => quote!{
            match ::std::str::FromStr::from_str(arg) {
                ::std::result::Result::Ok(v) => _result.#ident = v,
                ::std::result::Result::Err(ref e) =>
                    return ::std::result::Result::Err(
                        ::gumdrop::Error::failed_parse(opt,
                            ::std::string::ToString::to_string(e)))
            }
        },
        ParseArgOption => quote!{
            match ::std::str::FromStr::from_str(arg) {
                ::std::result::Result::Ok(v) =>
                    _result.#ident = ::std::option::Option::Some(v),
                ::std::result::Result::Err(ref e) =>
                    return ::std::result::Result::Err(
                        ::gumdrop::Error::failed_parse(opt,
                            ::std::string::ToString::to_string(e)))
            }
        },
        _ => unreachable!()
    }
}

fn make_long_name(name: &str) -> String {
    name.replace('_', "-")
}

fn make_short_name(name: &str, short: &[char]) -> Option<char> {
    let first = name.chars().next().expect("empty field name");

    if !short.contains(&first) {
        return Some(first);
    }

    let mut to_upper = first.to_uppercase();
    let upper = to_upper.next().expect("empty to_uppercase");

    if to_upper.next().is_some() {
        return None;
    }

    if !short.contains(&upper) {
        Some(upper)
    } else {
        None
    }
}

fn valid_long_name(name: &str, names: &[String]) {
    if name.is_empty() || name.starts_with('-') ||
            name.contains(|ch: char| ch.is_whitespace()) {
        panic!("`{}` is not a valid long option", name);
    }

    if names.iter().any(|n| n == name) {
        panic!("duplicate option name `--{}`", name);
    }
}

fn valid_short_name(ch: char, names: &[char]) {
    if ch == '-' || ch.is_whitespace() {
        panic!("`{}` is not a valid short option", ch);
    }

    if names.contains(&ch) {
        panic!("duplicate option name `-{}`", ch);
    }
}

fn make_meta(name: &str) -> String {
    name.replace('_', "-").to_uppercase()
}

fn make_usage(opts: &[Opt]) -> String {
    let mut res = String::new();

    let width = max(MIN_WIDTH, min(MAX_WIDTH,
        opts.iter().filter_map(|opt| {
            let w = opt.width();

            if w > MAX_WIDTH {
                None
            } else {
                Some(w)
            }
        }).max().unwrap_or(0)));

    for opt in opts {
        let mut line = String::from("  ");

        if let Some(short) = opt.short {
            line.push('-');
            line.push(short);
        }

        if opt.short.is_some() && opt.long.is_some() {
            line.push_str(", ");
        }

        if let Some(ref long) = opt.long {
            line.push_str("--");
            line.push_str(long);
        }

        if let Some(ref meta) = opt.meta {
            line.push(' ');
            line.push_str(meta);
        }

        if let Some(ref help) = opt.help {
            if line.len() < width {
                let n = width - line.len();
                line.extend(repeat(' ').take(n));
            } else {
                line.push('\n');
                line.extend(repeat(' ').take(width));
            }

            line.push_str(help);
        }

        res.push_str(&line);
        res.push('\n');
    }

    // Pop the last newline so the user may println!() the result.
    res.pop();

    res
}
