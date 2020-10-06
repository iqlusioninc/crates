//! Custom derive support for `zeroize`

#![crate_type = "proc-macro"]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, trivial_casts, unused_qualifications)]

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Meta, NestedMeta};
use synstructure::{decl_derive, BindStyle};

decl_derive!(
    [Zeroize, attributes(zeroize)] =>

    /// Derive the `Zeroize` trait.
    ///
    /// Supports the following attribute:
    ///
    /// - `#[zeroize(drop)]`: derives the `Drop` trait, calling `zeroize()`
    ///   when this item is dropped.
    derive_zeroize
);

/// Name of zeroize-related attributes
const ZEROIZE_ATTR: &str = "zeroize";

/// Custom derive for `Zeroize`
fn derive_zeroize(s: synstructure::Structure<'_>) -> TokenStream {
    let attributes = ZeroizeAttrs::parse(&s);

    // NOTE: These are split into named functions to simplify testing with
    // synstructure's `test_derive!` macro.
    if attributes.drop {
        derive_zeroize_with_drop(s)
    } else {
        derive_zeroize_without_drop(s)
    }
}

/// Custom derive attributes for `Zeroize`
#[derive(Default)]
struct ZeroizeAttrs {
    /// Derive a `Drop` impl which calls zeroize on this type
    drop: bool,
}

impl ZeroizeAttrs {
    /// Parse attributes from the incoming AST
    fn parse(s: &synstructure::Structure<'_>) -> Self {
        let mut result = Self::default();

        for v in s.variants().iter() {
            for attr in v.ast().attrs.iter() {
                result.parse_attr(attr);
            }
        }

        result
    }

    /// Parse attribute and handle `#[zeroize(...)]` attributes
    fn parse_attr(&mut self, attr: &Attribute) {
        let meta_list = match attr
            .parse_meta()
            .unwrap_or_else(|e| panic!("error parsing attribute: {:?} ({})", attr, e))
        {
            Meta::List(list) => list,
            _ => return,
        };

        // Ignore any non-zeroize attributes
        if !meta_list.path.is_ident(ZEROIZE_ATTR) {
            return;
        }

        for nested_meta in &meta_list.nested {
            if let NestedMeta::Meta(meta) = nested_meta {
                self.parse_meta(meta);
            } else {
                panic!("malformed #[zeroize] attribute: {:?}", nested_meta);
            }
        }
    }

    /// Parse `#[zeroize(...)]` attribute metadata (e.g. `drop`)
    fn parse_meta(&mut self, meta: &Meta) {
        if meta.path().is_ident("drop") {
            assert!(!self.drop, "duplicate #[zeroize] drop flags");
            self.drop = true;
        } else {
            panic!("unknown #[zeroize] attribute type: {:?}", meta.path());
        }
    }
}

/// Custom derive for `Zeroize` (without `Drop`)
fn derive_zeroize_without_drop(mut s: synstructure::Structure<'_>) -> TokenStream {
    s.bind_with(|_| BindStyle::RefMut);

    let zeroizers = s.each(|bi| quote! { #bi.zeroize(); });

    s.bound_impl(
        quote!(zeroize::Zeroize),
        quote! {
            fn zeroize(&mut self) {
                match self {
                    #zeroizers
                }
            }
        },
    )
}

/// Custom derive for `Zeroize` and `Drop`
fn derive_zeroize_with_drop(s: synstructure::Structure<'_>) -> TokenStream {
    let drop_impl = s.gen_impl(quote! {
        gen impl Drop for @Self {
            fn drop(&mut self) {
                self.zeroize();
            }
        }
    });

    let zeroize_impl = derive_zeroize_without_drop(s);

    quote! {
        #zeroize_impl

        #[doc(hidden)]
        #drop_impl
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use synstructure::test_derive;

    #[test]
    fn zeroize_without_drop() {
        test_derive! {
            derive_zeroize_without_drop {
                struct Z {
                    a: String,
                    b: Vec<u8>,
                    c: [u8; 3],
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                const _DERIVE_zeroize_Zeroize_FOR_Z: () = {
                    extern crate zeroize;
                    impl zeroize::Zeroize for Z {
                        fn zeroize(&mut self) {
                            match self {
                                Z {
                                    a: ref mut __binding_0,
                                    b: ref mut __binding_1,
                                    c: ref mut __binding_2,
                                } => {
                                    { __binding_0.zeroize(); }
                                    { __binding_1.zeroize(); }
                                    { __binding_2.zeroize(); }
                                }
                            }
                        }
                    }
                };
            }
            no_build // tests the code compiles are in the `zeroize` crate
        }
    }

    #[test]
    fn zeroize_with_drop() {
        test_derive! {
            derive_zeroize_with_drop {
                struct Z {
                    a: String,
                    b: Vec<u8>,
                    c: [u8; 3],
                }
            }
            expands to {
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                const _DERIVE_zeroize_Zeroize_FOR_Z: () = {
                    extern crate zeroize;
                    impl zeroize::Zeroize for Z {
                        fn zeroize(&mut self) {
                            match self {
                                Z {
                                    a: ref mut __binding_0,
                                    b: ref mut __binding_1,
                                    c: ref mut __binding_2,
                                } => {
                                    { __binding_0.zeroize(); }
                                    { __binding_1.zeroize(); }
                                    { __binding_2.zeroize(); }
                                }
                            }
                        }
                    }
                };
                #[doc(hidden)]
                #[allow(non_upper_case_globals)]
                const _DERIVE_Drop_FOR_Z: () = {
                    impl Drop for Z {
                        fn drop(&mut self) {
                            self.zeroize();
                        }
                    }
                };
            }
            no_build // tests the code compiles are in the `zeroize` crate
        }
    }
}
