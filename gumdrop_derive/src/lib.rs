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
//! * `help_flag` marks an option as a help flag. The field must be `bool` type.
//!   Options named `help` will automatically receive this option.
//! * `no_help_flag` prevents an option from being considered a help flag.
//! * `count` marks a field as a counter value. The field will be incremented
//!   each time the option appears in the arguments, i.e. `field += 1;`
//! * `free` marks a field as a positional argument field. Non-option arguments
//!   will be used to fill all `free` fields, in declared sequence.
//!   If the final `free` field is of type `Vec<T>`, it will contain all
//!   remaining free arguments.
//! * `short = "?"` sets the short option name to the given character
//! * `no_short` prevents a short option from being assigned to the field
//! * `long = "..."` sets the long option name to the given string
//! * `no_long` prevents a long option from being assigned to the field
//! * `required` will cause an error if the option is not present
//! * `not_required` will cancel a type-level `required` flag (see below).
//! * `help = "..."` sets help text returned from the `Options::usage` method
//! * `meta = "..."` sets the meta variable displayed in usage for options
//!   which accept an argument
//!
//! `#[options(...)]` may also be added at the type level. Only the flags
//! `no_help_flag`, `no_long`, `no_short`, and `required`
//! are supported at the type level.

#![recursion_limit = "256"]

extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use std::cmp::{max, min};
use std::iter::repeat;

use proc_macro::TokenStream;

use quote::{Tokens, ToTokens};
use syn::{
    Attribute, AttrStyle, Data, DataEnum, DataStruct, DeriveInput, Fields,
    GenericArgument, Ident, Lit, Meta, NestedMeta, Path, PathArguments, Type,
};

#[proc_macro_derive(Options, attributes(options))]
pub fn derive_options(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(ref data) =>
            derive_options_enum(&ast, data),
        Data::Struct(DataStruct{fields: Fields::Unit, ..}) =>
            panic!("cannot derive Options for unit struct types"),
        Data::Struct(DataStruct{fields: Fields::Unnamed(..), ..}) =>
            panic!("cannot derive Options for tuple struct types"),
        Data::Struct(DataStruct{ref fields, ..}) =>
            derive_options_struct(&ast, fields),
        Data::Union(_) =>
            panic!("cannot derive Options for union types"),
    }
}

fn derive_options_enum(ast: &DeriveInput, data: &DataEnum) -> TokenStream {
    let name = &ast.ident;
    let mut commands = Vec::new();
    let mut var_ty = Vec::new();

    for var in &data.variants {
        let ty = match var.fields {
            Fields::Unit | Fields::Named(_) =>
                panic!("command variants must be unary tuple variants"),
            Fields::Unnamed(ref fields) if fields.unnamed.len() != 1 =>
                panic!("command variants must be unary tuple variants"),
            Fields::Unnamed(ref fields) =>
                &fields.unnamed.first().unwrap().into_value().ty,
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
    let mut variant = Vec::new();
    let usage = make_cmd_usage(&commands);

    for cmd in commands {
        command.push(cmd.name);

        let var_name = cmd.variant_name;
        let ty = &cmd.ty;

        variant.push(var_name);

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

    let command_name_impl = {
        let name = repeat(name);

        quote!{
            match *self {
                #( #name::#variant(_) => ::std::option::Option::Some(#command), )*
            }
        }
    };

    let expr = quote!{
        impl #impl_generics ::gumdrop::Options for #name #ty_generics #where_clause {
            fn parse<__S: ::std::convert::AsRef<str>>(
                    parser: &mut ::gumdrop::Parser<__S>)
                    -> ::std::result::Result<Self, ::gumdrop::Error> {
                let arg = parser.next_arg()
                    .ok_or_else(::gumdrop::Error::missing_command)?;

                Self::parse_command(arg, parser)
            }

            fn command_name(&self) -> ::std::option::Option<&'static str> {
                #command_name_impl
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

            fn command_list() -> ::std::option::Option<&'static str> {
                ::std::option::Option::Some(<Self as ::gumdrop::Options>::usage())
            }

            fn command_usage(name: &str) -> ::std::option::Option<&'static str> {
                match name {
                    #( #command => ::std::option::Option::Some(
                        <#var_ty as ::gumdrop::Options>::usage()), )*
                    _ => ::std::option::Option::None
                }
            }
        }
    };

    expr.to_string().parse().expect("parse quote!")
}

fn derive_options_struct(ast: &DeriveInput, fields: &Fields) -> TokenStream {
    let mut pattern = Vec::new();
    let mut handle_opt = Vec::new();
    let mut short_names = Vec::new();
    let mut long_names = Vec::new();
    let mut free: Vec<FreeOpt> = Vec::new();
    let mut required = Vec::new();
    let mut required_err = Vec::new();
    let mut command = None;
    let mut command_ty = None;
    let mut command_required = false;
    let mut help_flag = Vec::new();
    let mut options = Vec::new();

    let default_opts = parse_type_attrs(&ast.attrs);

    for field in fields {
        let mut opts = parse_field_attrs(&field.attrs);
        opts.set_defaults(&default_opts);

        let ident = field.ident.as_ref().unwrap();

        if opts.command {
            if command.is_some() {
                panic!("duplicate declaration of `command` field");
            }
            if !free.is_empty() {
                panic!("`command` and `free` options are mutually exclusive");
            }

            command = Some(ident);
            command_ty = Some(first_ty_param(&field.ty).unwrap_or(&field.ty));
            command_required = opts.required;

            if opts.required {
                required.push(ident);
                required_err.push(quote!{
                    ::gumdrop::Error::missing_required_command() });
            }

            continue;
        }

        if opts.free {
            if command.is_some() {
                panic!("`command` and `free` options are mutually exclusive");
            }

            if let Some(last) = free.last() {
                if last.action == FreeAction::Push {
                    panic!("only the final `free` option may be of type `Vec<T>`");
                }
            }

            if opts.required {
                required.push(ident);
                required_err.push(quote!{
                    ::gumdrop::Error::missing_required_free() });
            }

            free.push(FreeOpt{
                field: ident,
                action: infer_free_action(&field.ty),
                required: opts.required,
                help: opts.help,
            });

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
            required: opts.required,
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
        if opt.required {
            required.push(opt.field);
            let display = opt.display_form();
            required_err.push(quote!{
                ::gumdrop::Error::missing_required(#display) });
        }

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
        handle_opt.push(make_action(opt));

        if let Some(ref long) = opt.long {
            let (pat, handle) = if let Some(n) = opt.action.tuple_len() {
                (quote!{ ::gumdrop::Opt::LongWithArg(#long, _) },
                    quote!{ return ::std::result::Result::Err(
                        ::gumdrop::Error::unexpected_single_argument(opt, #n)) })
            } else if opt.action.takes_arg() {
                (quote!{ ::gumdrop::Opt::LongWithArg(#long, arg) },
                    make_action_arg(opt))
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
    let usage = make_usage(&free, &options);

    let handle_free = if !free.is_empty() {
        let catch_all = if free.last().unwrap().action == FreeAction::Push {
            let last = free.pop().unwrap();

            let free = last.field;

            let mark_used = last.mark_used();

            quote!{
                match ::std::str::FromStr::from_str(free) {
                    ::std::result::Result::Ok(v) => {
                        #mark_used
                        _result.#free.push(v);
                    }
                    ::std::result::Result::Err(ref e) =>
                        return ::std::result::Result::Err(
                            ::gumdrop::Error::failed_parse(opt,
                                ::std::string::ToString::to_string(e)))
                }
            }
        } else {
            quote!{
                return ::std::result::Result::Err(
                    ::gumdrop::Error::unexpected_free(free))
            }
        };

        let num = 0..free.len();
        let action = free.iter().map(|free| {
            let field = free.field;

            let mark_used = free.mark_used();

            let assign = match free.action {
                FreeAction::Push => quote!{ _result.#field.push(v); },
                FreeAction::SetField => quote!{ _result.#field = v; },
                FreeAction::SetOption => quote!{
                    _result.#field = ::std::option::Option::Some(v);
                },
            };

            quote!{
                #mark_used
                #assign
            }
        }).collect::<Vec<_>>();

        quote!{
            match _free_counter {
                #( #num => {
                    _free_counter += 1;

                    match ::std::str::FromStr::from_str(free) {
                        ::std::result::Result::Ok(v) => { #action }
                        ::std::result::Result::Err(ref e) =>
                            return ::std::result::Result::Err(
                                ::gumdrop::Error::failed_parse(opt,
                                    ::std::string::ToString::to_string(e)))
                    }
                } )*
                _ => #catch_all
            }
        }
    } else if let Some(ident) = command {
        let mark_used = if command_required {
            quote!{ _used.#ident = true; }
        } else {
            quote!{ }
        };

        quote!{
            #mark_used
            _result.#ident = ::std::option::Option::Some(
                ::gumdrop::Options::parse_command(free, parser)?);
            break;
        }
    } else {
        quote!{
            return ::std::result::Result::Err(
                ::gumdrop::Error::unexpected_free(free));
        }
    };

    let command_name_impl = match command {
        None => quote!{ ::std::option::Option::None },
        Some(ref field) => quote!{
            ::std::option::Option::and_then(
                ::std::option::Option::as_ref(&self.#field),
                ::gumdrop::Options::command_name)
        }
    };

    let command_list = match command_ty {
        Some(ty) => quote!{
            ::std::option::Option::Some(
                <#ty as ::gumdrop::Options>::usage())
        },
        None => quote!{
            ::std::option::Option::None
        }
    };

    let command_usage = match command_ty {
        Some(ty) => quote!{
            <#ty as ::gumdrop::Options>::command_usage(_name)
        },
        None => quote!{
            ::std::option::Option::None
        }
    };

    let help_requested_impl = match (&help_flag, &command) {
        (flags, &None) => quote!{
            fn help_requested(&self) -> bool {
                false #( || self.#flags )*
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

    let required = &required;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expr = quote!{
        impl #impl_generics ::gumdrop::Options for #name #ty_generics #where_clause {
            fn parse<__S: ::std::convert::AsRef<str>>(
                    parser: &mut ::gumdrop::Parser<__S>)
                    -> ::std::result::Result<Self, ::gumdrop::Error> {
                #[derive(Default)]
                struct _Used {
                    #( #required: bool , )*
                }

                let mut _result = <Self as ::std::default::Default>::default();
                let mut _free_counter = 0usize;
                let mut _used = _Used::default();

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

                #( if !_used.#required {
                    return ::std::result::Result::Err(#required_err);
                } )*

                ::std::result::Result::Ok(_result)
            }

            fn command_name(&self) -> ::std::option::Option<&'static str> {
                #command_name_impl
            }

            #help_requested_impl

            fn parse_command<__S: ::std::convert::AsRef<str>>(name: &str,
                    _parser: &mut ::gumdrop::Parser<__S>)
                    -> ::std::result::Result<Self, ::gumdrop::Error> {
                ::std::result::Result::Err(
                    ::gumdrop::Error::unrecognized_command(name))
            }

            fn usage() -> &'static str {
                #usage
            }

            fn command_list() -> ::std::option::Option<&'static str> {
                #command_list
            }

            fn command_usage(_name: &str) -> ::std::option::Option<&'static str> {
                #command_usage
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FreeAction {
    Push,
    SetField,
    SetOption,
}

impl Action {
    fn takes_arg(&self) -> bool {
        use self::Action::*;

        match *self {
            Push(ty) | SetField(ty) | SetOption(ty) => ty.takes_arg(),
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
    fn takes_arg(&self) -> bool {
        match *self {
            ActionType::ParseTuple(0) => false,
            ActionType::ParseTuple(_) |
            ActionType::Parse => true
        }
    }

    fn tuple_len(&self) -> Option<usize> {
        match *self {
            ActionType::ParseTuple(n) => Some(n),
            _ => None
        }
    }
}

fn first_ty_param(ty: &Type) -> Option<&Type> {
    match *ty {
        Type::Path(ref path) => {
            let path = path.path.segments.last().unwrap().into_value();

            match path.arguments {
                PathArguments::AngleBracketed(ref data) =>
                    data.args.iter().filter_map(|arg| match arg {
                        &GenericArgument::Type(ref ty) => Some(ty),
                        _ => None
                    }).next(),
                _ => None
            }
        }
        _ => None
    }
}

fn infer_action(ty: &Type) -> Action {
    match *ty {
        Type::Path(ref path) => {
            let path = path.path.segments.last().unwrap().into_value();
            let param = first_ty_param(ty);

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

fn infer_action_type(ty: &Type) -> ActionType {
    match *ty {
        Type::Tuple(ref tup) => ActionType::ParseTuple(tup.elems.len()),
        _ => ActionType::Parse,
    }
}

fn infer_free_action(ty: &Type) -> FreeAction {
    match *ty {
        Type::Path(ref path) => {
            let path = path.path.segments.last().unwrap().into_value();

            match path.ident.as_ref() {
                "Option" => FreeAction::SetOption,
                "Vec" => FreeAction::Push,
                _ => FreeAction::SetField
            }
        }
        _ => FreeAction::SetField,
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
    required: bool,
    not_required: bool,
    help: Option<String>,
    meta: Option<String>,

    command: bool,
}

#[derive(Debug, Default)]
struct DefaultOpts {
    no_help_flag: bool,
    no_long: bool,
    no_short: bool,
    required: bool,
}

impl AttrOpts {
    fn check(&self) {
        if self.command {
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

        if self.free {
            if self.long.is_some() { panic!("`free` and `long` are mutually exclusive"); }
            if self.short.is_some() { panic!("`free` and `short` are mutually exclusive"); }
            if self.count { panic!("`free` and `count` are mutually exclusive"); }
            if self.help_flag { panic!("`free` and `help_flag` are mutually exclusive"); }
            if self.no_help_flag { panic!("`free` and `no_help_flag` are mutually exclusive"); }
            if self.no_short { panic!("`free` and `no_short` are mutually exclusive"); }
            if self.no_long { panic!("`free` and `no_long` are mutually exclusive"); }
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

        if self.required && self.not_required {
            panic!("`required` and `not_required` are mutually exclusive");
        }
    }

    fn set_defaults(&mut self, defaults: &DefaultOpts) {
        if !self.help_flag && defaults.no_help_flag {
            self.no_help_flag = true;
        }
        if self.short.is_none() && defaults.no_short {
            self.no_short = true;
        }
        if self.long.is_none() && defaults.no_long {
            self.no_long = true;
        }

        if self.not_required {
            self.required = false;
        } else if defaults.required {
            self.required = true;
        }
    }
}

#[derive(Debug)]
struct FreeOpt<'a> {
    field: &'a Ident,
    action: FreeAction,
    required: bool,
    help: Option<String>,
}

#[derive(Debug)]
struct Opt<'a> {
    field: &'a Ident,
    action: Action,
    long: Option<String>,
    short: Option<char>,
    no_short: bool,
    required: bool,
    help: Option<String>,
    meta: Option<String>,
}

const MIN_WIDTH: usize = 8;
const MAX_WIDTH: usize = 30;

impl<'a> FreeOpt<'a> {
    fn mark_used(&self) -> Tokens {
        if self.required {
            let field = self.field;
            quote!{ _used.#field = true; }
        } else {
            quote!{ }
        }
    }

    fn width(&self) -> usize {
        2 + self.field.as_ref().len() + 2 // name + spaces before and after
    }
}

impl<'a> Opt<'a> {
    fn display_form(&self) -> String {
        if let Some(ref long) = self.long {
            format!("--{}", long)
        } else {
            format!("-{}", self.short.unwrap())
        }
    }

    fn mark_used(&self) -> Tokens {
        if self.required {
            let field = self.field;
            quote!{ _used.#field = true; }
        } else {
            quote!{ }
        }
    }

    fn width(&self) -> usize {
        let short = self.short.map_or(0, |_| 1 + 1); // '-' + char
        let long = self.long.as_ref().map_or(0, |s| s.len() + 2); // "--" + str
        let sep = if short == 0 || long == 0 { 0 } else { 2 };
        let meta = self.meta.as_ref().map_or(0, |s| s.len() + 1); // ' ' + meta

        2 + short + long + sep + meta + 2 // total + spaces before and after
    }
}

fn parse_type_attrs(attrs: &[Attribute]) -> DefaultOpts {
    let mut opts = DefaultOpts::default();

    for attr in attrs {
        if is_outer(attr.style) && path_eq(&attr.path, "options") {
            let meta = attr.interpret_meta().unwrap_or_else(
                || panic!("invalid attribute: {}", tokens_str(attr)));

            match meta {
                Meta::Word(_) =>
                    panic!("#[options] is not a valid attribute"),
                Meta::NameValue(..) =>
                    panic!("#[options = ...] is not a valid attribute"),
                Meta::List(ref items) => {
                    for item in &items.nested {
                        match *item {
                            NestedMeta::Literal(_) =>
                                panic!("unexpected meta item `{}`", tokens_str(item)),
                            NestedMeta::Meta(ref item) => {
                                match *item {
                                    Meta::Word(ref w) => match w.as_ref() {
                                        "no_help_flag" => opts.no_help_flag = true,
                                        "no_short" => opts.no_short = true,
                                        "no_long" => opts.no_long = true,
                                        "required" => opts.required = true,
                                        _ => panic!("unexpected meta item `{}`", tokens_str(item))
                                    },
                                    Meta::List(..) | Meta::NameValue(..) =>
                                        panic!("unexpected meta item `{}`", tokens_str(item)),
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

fn parse_field_attrs(attrs: &[Attribute]) -> AttrOpts {
    let mut opts = AttrOpts::default();

    for attr in attrs {
        if is_outer(attr.style) && path_eq(&attr.path, "options") {
            let meta = attr.interpret_meta().unwrap_or_else(
                || panic!("invalid attribute: {}", tokens_str(attr)));

            match meta {
                Meta::Word(_) =>
                    panic!("#[options] is not a valid attribute"),
                Meta::NameValue(..) =>
                    panic!("#[options = ...] is not a valid attribute"),
                Meta::List(ref items) => {
                    for item in &items.nested {
                        match *item {
                            NestedMeta::Literal(_) =>
                                panic!("unexpected meta item `{}`", tokens_str(item)),
                            NestedMeta::Meta(ref item) => {
                                match *item {
                                    Meta::Word(ref w) => match w.as_ref() {
                                        "free" => opts.free = true,
                                        "command" => opts.command = true,
                                        "count" => opts.count = true,
                                        "help_flag" => opts.help_flag = true,
                                        "no_help_flag" => opts.no_help_flag = true,
                                        "no_short" => opts.no_short = true,
                                        "no_long" => opts.no_long = true,
                                        "required" => opts.required = true,
                                        "not_required" => opts.not_required = true,
                                        _ => panic!("unexpected meta item `{}`", tokens_str(item))
                                    },
                                    Meta::List(..) => panic!("unexpected meta item `{}`", tokens_str(item)),
                                    Meta::NameValue(ref nv) => {
                                        match nv.ident.as_ref() {
                                            "long" => opts.long = Some(lit_str(&nv.lit)),
                                            "short" => opts.short = Some(lit_char(&nv.lit)),
                                            "help" => opts.help = Some(lit_str(&nv.lit)),
                                            "meta" => opts.meta = Some(lit_str(&nv.lit)),
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

fn is_outer(style: AttrStyle) -> bool {
    match style {
        AttrStyle::Outer => true,
        _ => false
    }
}

struct Cmd<'a> {
    name: String,
    help: Option<String>,
    variant_name: &'a Ident,
    ty: &'a Type,
}

#[derive(Default)]
struct CmdOpts {
    name: Option<String>,
    help: Option<String>,
}

fn parse_cmd_attrs(attrs: &[Attribute]) -> CmdOpts {
    let mut opts = CmdOpts::default();

    for attr in attrs {
        if is_outer(attr.style) && path_eq(&attr.path, "options") {
            let meta = attr.interpret_meta().unwrap_or_else(
                || panic!("invalid attribute: {}", tokens_str(attr)));

            match meta {
                Meta::Word(_) =>
                    panic!("#[options] is not a valid attribute"),
                Meta::NameValue(..) =>
                    panic!("#[options = ...] is not a valid attribute"),
                Meta::List(ref items) => {
                    for item in &items.nested {
                        match *item {
                            NestedMeta::Literal(_) =>
                                panic!("unexpected meta item `{}`", tokens_str(item)),
                            NestedMeta::Meta(ref item) => {
                                match *item {
                                    Meta::Word(_) | Meta::List(..) =>
                                        panic!("unexpected meta item `{}`", tokens_str(item)),
                                    Meta::NameValue(ref nv) => {
                                        match nv.ident.as_ref() {
                                            "name" => opts.name = Some(lit_str(&nv.lit)),
                                            "help" => opts.help = Some(lit_str(&nv.lit)),
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
        Lit::Str(ref s) => s.value(),
        _ => panic!("unexpected literal `{}`", tokens_str(lit))
    }
}

fn lit_char(lit: &Lit) -> char {
    match *lit {
        // Character literals in attributes are not necessarily allowed
        Lit::Str(ref s) => {
            let s = s.value();
            let mut chars = s.chars();

            let res = chars.next().expect("expected one-char string literal");
            if chars.next().is_some() {
                panic!("expected one-char string literal");
            }

            res
        }
        Lit::Char(ref ch) => ch.value(),
        _ => panic!("unexpected literal `{}`", tokens_str(lit))
    }
}

fn path_eq(path: &Path, s: &str) -> bool {
    path.segments.len() == 1 && {
        let seg = path.segments.first().unwrap().into_value();

        match seg.arguments {
            PathArguments::None => seg.ident.as_ref() == s,
            _ => false
        }
    }
}

fn tokens_str<T: ToTokens>(t: &T) -> String {
    let mut tok = Tokens::new();
    t.to_tokens(&mut tok);
    tok.to_string()
}

fn make_action(opt: &Opt) -> Tokens {
    use self::Action::*;

    let field = opt.field;
    let mark_used = opt.mark_used();

    let action = match opt.action {
        Count => quote!{
            _result.#field += 1;
        },
        Push(ty) => {
            let act = make_action_type(ty);

            quote!{
                _result.#field.push(#act);
            }
        }
        SetField(ty) => {
            let act = make_action_type(ty);

            quote!{
                _result.#field = #act;
            }
        }
        SetOption(ty) => {
            let act = make_action_type(ty);

            quote!{
                _result.#field = ::std::option::Option::Some(#act);
            }
        }
        Switch => quote!{
            _result.#field = true;
        }
    };

    quote!{
        #mark_used
        #action
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

fn make_action_arg(opt: &Opt) -> Tokens {
    use self::Action::*;

    let field = opt.field;
    let mark_used = opt.mark_used();

    let action = match opt.action {
        Push(ty) => {
            let act = make_action_type_arg(ty);

            quote!{
                _result.#field.push(#act);
            }
        }
        SetField(ty) => {
            let act = make_action_type_arg(ty);

            quote!{
                _result.#field = #act;
            }
        }
        SetOption(ty) => {
            let act = make_action_type_arg(ty);

            quote!{
                _result.#field = ::std::option::Option::Some(#act);
            }
        }
        _ => unreachable!()
    };

    quote!{
        #mark_used
        #action
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

    let mut name = name.replace('_', "-").to_uppercase();

    match action.tuple_len() {
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

fn make_usage(free: &[FreeOpt], opts: &[Opt]) -> String {
    let mut res = String::new();

    let width = max(
        max_width(free, |opt| opt.width()),
        max_width(opts, |opt| opt.width()));

    if !free.is_empty() {
        res.push_str("Positional arguments:\n");

        for opt in free {
            let mut line = String::from("  ");

            line.push_str(opt.field.as_ref());

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
    }

    if !opts.is_empty() {
        if !res.is_empty() {
            res.push('\n');
        }

        res.push_str("Optional arguments:\n");

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
    }

    // Pop the last newline so the user may println!() the result.
    res.pop();

    res
}

fn max_width<T, F>(items: &[T], f: F) -> usize
        where F: Fn(&T) -> usize {
    max(MIN_WIDTH, min(MAX_WIDTH,
        items.iter().filter_map(|item| {
            let w = f(item);

            if w > MAX_WIDTH {
                None
            } else {
                Some(w)
            }
        }).max().unwrap_or(0)))
}

fn make_cmd_usage(cmds: &[Cmd]) -> String {
    let mut res = String::new();

    let width = max_width(cmds,
        // Two spaces each, before and after
        |cmd| cmd.name.len() + 4);

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
