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
//! * `command` indicates that a field represents a subcommand. The field must
//!   be of type `Option<T>` where `T` is a type implementing `Options`.
//!   Typically, this type is an `enum` containing subcommand option types.
//! * `command_name` will contain the name of the command selected by the user.
//!   Its type must be `Option<String>`.
//! * `help_flag` marks an option as a help flag. The field must be `bool` type.
//!   Options named `help` will automatically receive this option.
//! * `no_help_flag` prevents an option from being considered a help flag.
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

#![recursion_limit = "256"]

extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use std::cmp::{max, min};
use std::iter::repeat;

use proc_macro::TokenStream;

use quote::{Tokens, ToTokens};
use syn::{
    Attribute, AttrStyle, Body, DeriveInput, Field, Ident, Lit,
    MetaItem, NestedMetaItem, PathParameters, Ty, Variant, VariantData,
};

#[proc_macro_derive(Options, attributes(options))]
pub fn derive_options(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string())
        .expect("parse_derive_input");

    match ast.body {
        Body::Enum(ref variants) =>
            derive_options_enum(&ast, variants),
        Body::Struct(VariantData::Unit) =>
            panic!("cannot derive Options for unit struct types"),
        Body::Struct(VariantData::Tuple(_)) =>
            panic!("cannot derive Options for tuple struct types"),
        Body::Struct(VariantData::Struct(ref fields)) =>
            derive_options_struct(&ast, fields)
    }
}

fn derive_options_enum(ast: &DeriveInput, variants: &[Variant]) -> TokenStream {
    let name = &ast.ident;
    let mut commands = Vec::new();
    let mut var_ty = Vec::new();

    for var in variants {
        let ty = match var.data {
            VariantData::Unit | VariantData::Struct(_) =>
                panic!("command variants must be unary tuple variants"),
            VariantData::Tuple(ref fields) if fields.len() != 1 =>
                panic!("command variants must be unary tuple variants"),
            VariantData::Tuple(ref fields) => &fields[0].ty,
        };

        let opts = parse_cmd_attrs(&var.attrs);

        let var_name = &var.ident;

        var_ty.push(ty);

        commands.push(Cmd{
            name: opts.name.unwrap_or_else(
                || make_command_name(var_name.as_ref())),
            help: opts.help,
            variant_name: var_name,
            ty: ty,
        });
    }

    let mut command = Vec::new();
    let mut handle_cmd = Vec::new();
    let mut help_req_impl = Vec::new();
    let usage = make_cmd_usage(&commands);

    for cmd in commands {
        command.push(Lit::from(cmd.name));

        let var_name = &cmd.variant_name;
        let ty = &cmd.ty;

        handle_cmd.push(quote!{
            #name::#var_name(<#ty as ::gumdrop::Options>::parse(parser)?)
        });

        help_req_impl.push(quote!{
            #name::#var_name(ref cmd) => { ::gumdrop::Options::help_requested(cmd) }
        });
    }

    // Borrow re-used items
    let command = &command;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expr = quote!{
        impl #impl_generics ::gumdrop::Options for #name #ty_generics #where_clause {
            fn parse<__S: ::std::convert::AsRef<str>>(
                    parser: &mut ::gumdrop::Parser<__S>)
                    -> ::std::result::Result<Self, ::gumdrop::Error> {
                let arg = parser.next_arg()
                    .ok_or_else(::gumdrop::Error::missing_command)?;

                Self::parse_command(arg, parser)
            }

            fn help_requested(&self) -> bool {
                match *self {
                    #( #help_req_impl )*
                }
            }

            fn parse_command<__S: ::std::convert::AsRef<str>>(name: &str,
                    parser: &mut ::gumdrop::Parser<__S>)
                    -> ::std::result::Result<Self, ::gumdrop::Error> {
                let cmd = match name {
                    #( #command => { #handle_cmd } )*
                    _ => return ::std::result::Result::Err(
                        ::gumdrop::Error::unrecognized_command(name))
                };

                ::std::result::Result::Ok(cmd)
            }

            fn usage() -> &'static str {
                #usage
            }

            fn command_usage(name: &str) -> ::std::option::Option<&'static str> {
                match name {
                    #( #command => ::std::option::Option::Some(
                        <#var_ty as ::gumdrop::Options>::usage()), )*
                    _ => None
                }
            }
        }
    };

    expr.to_string().parse().expect("parse quote!")
}

fn derive_options_struct(ast: &DeriveInput, fields: &[Field]) -> TokenStream {
    let mut pattern = Vec::new();
    let mut handle_opt = Vec::new();
    let mut short_names = Vec::new();
    let mut long_names = Vec::new();
    let mut free = None;
    let mut command = None;
    let mut command_name = None;
    let mut help_flag = Vec::new();
    let mut options = Vec::new();

    for field in fields {
        let mut opts = parse_attrs(&field.attrs);

        let ident = field.ident.as_ref().unwrap();

        if opts.command {
            if command.is_some() {
                panic!("duplicate declaration of `command` field");
            }
            if free.is_some() {
                panic!("`command` and `free` options are mutually exclusive");
            }

            command = Some(ident);
            continue;
        }

        if opts.command_name {
            if command_name.is_some() {
                panic!("duplicate declaration of `command_name` field");
            }

            command_name = Some(ident);
            continue;
        }

        if opts.free {
            if command.is_some() {
                panic!("`command` and `free` options are mutually exclusive");
            }
            if free.is_some() {
                panic!("duplicate declaration of `free` field");
            }

            free = Some(ident);
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

        if opts.help_flag || (!opts.no_help_flag &&
                opts.long.as_ref().map(|s| &s[..]) == Some("help")) {
            help_flag.push(ident);
        }

        let action = if opts.count {
            Action::Count
        } else {
            infer_action(&field.ty)
        };

        if action.takes_arg() {
            if opts.meta.is_none() {
                opts.meta = Some(make_meta(ident.as_ref(), action));
            }
        } else if opts.meta.is_some() {
            panic!("`meta` value is invalid for option `{}`", ident.as_ref());
        }

        options.push(Opt{
            field: ident,
            action: action,
            long: opts.long.take(),
            short: opts.short,
            no_short: opts.no_short,
            meta: opts.meta.take(),
            help: opts.help.take(),
        });
    }

    if command_name.is_some() && command.is_none() {
        panic!("cannot declare `command_name` without `command`");
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
            let (pat, handle) = if let Some(n) = opt.action.tuple_len() {
                (quote!{ ::gumdrop::Opt::LongWithArg(#long, _) },
                    quote!{ return ::std::result::Result::Err(
                        ::gumdrop::Error::unexpected_single_argument(opt, #n)) })
            } else if opt.action.takes_arg() {
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

    let name = &ast.ident;
    let usage = Lit::from(make_usage(&options));

    let handle_free = if let Some(free) = free {
        quote!{
            match ::std::str::FromStr::from_str(free) {
                ::std::result::Result::Ok(v) => _result.#free.push(v),
                ::std::result::Result::Err(ref e) =>
                    return ::std::result::Result::Err(
                        ::gumdrop::Error::failed_parse(opt,
                            ::std::string::ToString::to_string(e)))
            }
        }
    } else if let Some(ident) = command {
        if let Some(name_ident) = command_name {
            quote!{
                _result.#name_ident = Some(::std::string::ToString::to_string(free));
                _result.#ident = ::std::option::Option::Some(
                    ::gumdrop::Options::parse_command(free, parser)?);
                break;
            }
        } else {
            quote!{
                _result.#ident = ::std::option::Option::Some(
                    ::gumdrop::Options::parse_command(free, parser)?);
                break;
            }
        }
    } else {
        quote!{
            return ::std::result::Result::Err(
                ::gumdrop::Error::unexpected_free(free));
        }
    };

    let help_requested_impl = match (&help_flag, &command) {
        (flags, &None) if flags.is_empty() => quote!{ },
        (flags, &None) => quote!{
            fn help_requested(&self) -> bool {
                #( self.#flags )||*
            }
        },
        (flags, &Some(ref cmd)) if flags.is_empty() => quote!{
            fn help_requested(&self) -> bool {
                ::std::option::Option::map_or(
                    ::std::option::Option::as_ref(&self.#cmd),
                    false, ::gumdrop::Options::help_requested)
            }
        },
        (flags, &Some(ref cmd)) => quote!{
            fn help_requested(&self) -> bool {
                #( self.#flags || )*
                ::std::option::Option::map_or(
                    ::std::option::Option::as_ref(&self.#cmd),
                    false, ::gumdrop::Options::help_requested)
            }
        }
    };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expr = quote!{
        impl #impl_generics ::gumdrop::Options for #name #ty_generics #where_clause {
            fn parse<__S: ::std::convert::AsRef<str>>(
                    parser: &mut ::gumdrop::Parser<__S>)
                    -> ::std::result::Result<Self, ::gumdrop::Error> {
                let mut _result = <Self as ::std::default::Default>::default();

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

            #help_requested_impl

            fn parse_command<__S: ::std::convert::AsRef<str>>(name: &str,
                    _parser: &mut ::gumdrop::Parser<__S>)
                    -> ::std::result::Result<Self, ::gumdrop::Error> {
                Err(::gumdrop::Error::unrecognized_command(name))
            }

            fn usage() -> &'static str {
                #usage
            }

            fn command_usage(_name: &str) -> ::std::option::Option<&'static str> {
                None
            }
        }
    };

    expr.to_string().parse().expect("parse quote!")
}

#[derive(Copy, Clone, Debug)]
enum Action {
    /// Increase count
    Count,
    /// Push an argument to a `Vec<T>` field
    Push(ActionType),
    /// Set field
    SetField(ActionType),
    /// Set `Option<T>` field
    SetOption(ActionType),
    /// Set field to `true`
    Switch,
}

#[derive(Copy, Clone, Debug)]
enum ActionType {
    /// Parse using `FromStr`
    Parse,
    /// Parse `n` tuple fields, each using `FromStr`
    ParseTuple(usize),
}

impl Action {
    fn takes_arg(&self) -> bool {
        use self::Action::*;

        match *self {
            Push(_) | SetField(_) | SetOption(_) => true,
            _ => false
        }
    }

    fn tuple_len(&self) -> Option<usize> {
        use self::Action::*;

        match *self {
            Push(ty) | SetField(ty) | SetOption(ty) => ty.tuple_len(),
            _ => None
        }
    }
}

impl ActionType {
    fn tuple_len(&self) -> Option<usize> {
        match *self {
            ActionType::ParseTuple(n) => Some(n),
            _ => None
        }
    }
}

fn infer_action(ty: &Ty) -> Action {
    match *ty {
        Ty::Path(_, ref path) => {
            let path = path.segments.last().unwrap();

            let param = match path.parameters {
                PathParameters::AngleBracketed(ref data) => data.types.get(0),
                _ => None
            };

            match path.ident.as_ref() {
                "bool" => Action::Switch,
                "Vec" => Action::Push(infer_action_type(
                    param.expect("expected type parameter for `Vec`"))),
                "Option" => Action::SetOption(infer_action_type(
                    param.expect("expected type parameter for `Option`"))),
                _ => Action::SetField(ActionType::Parse),
            }
        }
        _ => Action::SetField(infer_action_type(ty))
    }
}

fn infer_action_type(ty: &Ty) -> ActionType {
    match *ty {
        Ty::Tup(ref fields) => ActionType::ParseTuple(fields.len()),
        _ => ActionType::Parse,
    }
}

#[derive(Debug, Default)]
struct AttrOpts {
    long: Option<String>,
    short: Option<char>,
    free: bool,
    count: bool,
    help_flag: bool,
    no_help_flag: bool,
    no_short: bool,
    no_long: bool,
    help: Option<String>,
    meta: Option<String>,

    command: bool,
    command_name: bool,
}

impl AttrOpts {
    fn check(&self) {
        if self.command {
            if self.command_name { panic!("`command` and `command_name` are mutually exclusive"); }
            if self.free { panic!("`command` and `free` are mutually exclusive"); }
            if self.long.is_some() { panic!("`command` and `long` are mutually exclusive"); }
            if self.short.is_some() { panic!("`command` and `short` are mutually exclusive"); }
            if self.count { panic!("`command` and `count` are mutually exclusive"); }
            if self.help_flag { panic!("`command` and `help_flag` are mutually exclusive"); }
            if self.no_help_flag { panic!("`command` and `no_help_flag` are mutually exclusive"); }
            if self.no_short { panic!("`command` and `no_short` are mutually exclusive"); }
            if self.no_long { panic!("`command` and `no_long` are mutually exclusive"); }
            if self.help.is_some() { panic!("`command` and `help` are mutually exclusive"); }
            if self.meta.is_some() { panic!("`command` and `meta` are mutually exclusive"); }
        }

        if self.command_name {
            if self.free { panic!("`command_name` and `free` are mutually exclusive"); }
            if self.long.is_some() { panic!("`command_name` and `long` are mutually exclusive"); }
            if self.short.is_some() { panic!("`command_name` and `short` are mutually exclusive"); }
            if self.count { panic!("`command_name` and `count` are mutually exclusive"); }
            if self.help_flag { panic!("`command_name` and `help_flag` are mutually exclusive"); }
            if self.no_help_flag { panic!("`command_name` and `no_help_flag` are mutually exclusive"); }
            if self.no_short { panic!("`command_name` and `no_short` are mutually exclusive"); }
            if self.no_long { panic!("`command_name` and `no_long` are mutually exclusive"); }
            if self.help.is_some() { panic!("`command_name` and `help` are mutually exclusive"); }
            if self.meta.is_some() { panic!("`command_name` and `meta` are mutually exclusive"); }
        }

        if self.free {
            if self.long.is_some() { panic!("`free` and `long` are mutually exclusive"); }
            if self.short.is_some() { panic!("`free` and `short` are mutually exclusive"); }
            if self.count { panic!("`free` and `count` are mutually exclusive"); }
            if self.help_flag { panic!("`free` and `help_flag` are mutually exclusive"); }
            if self.no_help_flag { panic!("`free` and `no_help_flag` are mutually exclusive"); }
            if self.no_short { panic!("`free` and `no_short` are mutually exclusive"); }
            if self.no_long { panic!("`free` and `no_long` are mutually exclusive"); }
            if self.help.is_some() { panic!("`free` and `help` are mutually exclusive"); }
            if self.meta.is_some() { panic!("`free` and `meta` are mutually exclusive"); }
        }

        if self.help_flag && self.no_help_flag {
            panic!("`help_flag` and `no_help_flag` are mutually exclusive");
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
struct Opt<'a> {
    field: &'a Ident,
    action: Action,
    long: Option<String>,
    short: Option<char>,
    no_short: bool,
    help: Option<String>,
    meta: Option<String>,
}

const MIN_WIDTH: usize = 8;
const MAX_WIDTH: usize = 30;

impl<'a> Opt<'a> {
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
                                        "command" => opts.command = true,
                                        "command_name" => opts.command_name = true,
                                        "count" => opts.count = true,
                                        "help_flag" => opts.help_flag = true,
                                        "no_help_flag" => opts.no_help_flag = true,
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

#[derive(Debug)]
struct Cmd<'a> {
    name: String,
    help: Option<String>,
    variant_name: &'a Ident,
    ty: &'a Ty,
}

#[derive(Default)]
struct CmdOpts {
    name: Option<String>,
    help: Option<String>,
}

fn parse_cmd_attrs(attrs: &[Attribute]) -> CmdOpts {
    let mut opts = CmdOpts::default();

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
                                    MetaItem::Word(_) | MetaItem::List(..) =>
                                        panic!("unexpected meta item `{}`", tokens_str(item)),
                                    MetaItem::NameValue(ref name, ref value) => {
                                        match name.as_ref() {
                                            "name" => opts.name = Some(lit_str(value)),
                                            "help" => opts.help = Some(lit_str(value)),
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
        Count => quote!{
            _result.#ident += 1;
        },
        Push(ty) => {
            let act = make_action_type(ty);

            quote!{
                _result.#ident.push(#act);
            }
        }
        SetField(ty) => {
            let act = make_action_type(ty);

            quote!{
                _result.#ident = #act;
            }
        }
        SetOption(ty) => {
            let act = make_action_type(ty);

            quote!{
                _result.#ident = ::std::option::Option::Some(#act);
            }
        }
        Switch => quote!{
            _result.#ident = true;
        }
    }
}

fn make_action_type(action_type: ActionType) -> Tokens {
    use self::ActionType::*;

    match action_type {
        Parse => quote!{ {
            let arg = parser.next_arg()
                .ok_or_else(|| ::gumdrop::Error::missing_argument(opt))?;

            ::std::str::FromStr::from_str(arg)
                .map_err(|e| ::gumdrop::Error::failed_parse(
                    opt, ::std::string::ToString::to_string(&e)))?
        } },
        ParseTuple(n) => {
            let num = 0..n;
            let n = repeat(n);

            quote!{
                ( #( {
                    let found = #num;
                    let arg = parser.next_arg()
                        .ok_or_else(|| ::gumdrop::Error::insufficient_arguments(
                            opt, #n, found))?;

                    ::std::str::FromStr::from_str(arg)
                        .map_err(|e| ::gumdrop::Error::failed_parse(
                            opt, ::std::string::ToString::to_string(&e)))?
                } , )* )
            }
        }
    }
}

fn make_action_arg(ident: &Ident, action: Action) -> Tokens {
    use self::Action::*;

    match action {
        Push(ty) => {
            let act = make_action_type_arg(ty);

            quote!{
                _result.#ident.push(#act);
            }
        }
        SetField(ty) => {
            let act = make_action_type_arg(ty);

            quote!{
                _result.#ident = #act;
            }
        }
        SetOption(ty) => {
            let act = make_action_type_arg(ty);

            quote!{
                _result.#ident = ::std::option::Option::Some(#act);
            }
        }
        _ => unreachable!()
    }
}

fn make_action_type_arg(action_type: ActionType) -> Tokens {
    use self::ActionType::*;

    match action_type {
        Parse => quote!{
            ::std::str::FromStr::from_str(arg)
                .map_err(|e| ::gumdrop::Error::failed_parse(
                    opt, ::std::string::ToString::to_string(&e)))?
        },
        ParseTuple(_) => unreachable!()
    }
}

fn make_command_name(name: &str) -> String {
    let mut res = String::with_capacity(name.len());

    for ch in name.chars() {
        if ch.is_lowercase() {
            res.push(ch);
        } else {
            if !res.is_empty() {
                res.push('-');
            }

            res.extend(ch.to_lowercase());
        }
    }

    res
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

fn make_meta(name: &str, action: Action) -> String {
    use std::fmt::Write;

    let tuple_len = action.tuple_len();

    if tuple_len == Some(0) {
        return String::new();
    }

    let mut name = name.replace('_', "-").to_uppercase();

    match action.tuple_len() {
        // Handled above with early return
        Some(0) => unreachable!(),
        Some(1) | None => (),
        Some(2) => {
            name.push_str(" VALUE");
        }
        Some(n) => {
            for i in 1..n {
                let _ = write!(name, " VALUE{}", i - 1);
            }
        }
    }

    name
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

fn make_cmd_usage(cmds: &[Cmd]) -> String {
    let mut res = String::new();

    let width = max(MIN_WIDTH, min(MAX_WIDTH,
        cmds.iter().filter_map(|cmd| {
            let w = cmd.name.len() + 4; // Two spaces each, before and after

            if w > MAX_WIDTH {
                None
            } else {
                Some(w)
            }
        }).max().unwrap_or(0)));

    for cmd in cmds {
        let mut line = String::from("  ");

        line.push_str(&cmd.name);

        if let Some(ref help) = cmd.help {
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

    // Pop the last newline
    res.pop();

    res
}
