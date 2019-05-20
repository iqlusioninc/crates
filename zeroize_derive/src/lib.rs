//! Custom derive support for `zeroize`

#![crate_type = "proc-macro"]
#![deny(warnings, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use synstructure::{decl_derive, BindStyle};

/// Name of zeroize-related attributes
const ZEROIZE_ATTR: &str = "zeroize";

/// Custom derive for `Zeroize`
fn zeroize_derive(mut s: synstructure::Structure) -> TokenStream {
    s.bind_with(|_| BindStyle::RefMut);

    let attributes = ZeroizeDeriveAttrs::parse(&s);

    let zeroizers = s.each(|bi| quote! { #bi.zeroize(); });

    let zeroize_impl = s.bound_impl(
        quote!(zeroize::Zeroize),
        quote! {
            fn zeroize(&mut self) {
                match self {
                    #zeroizers
                }
            }
        },
    );

    if attributes.no_drop {
        return zeroize_impl;
    }

    let drop_impl = s.gen_impl(quote! {
        gen impl Drop for @Self {
            fn drop(&mut self) {
                self.zeroize();
            }
        }
    });

    quote! {
        #zeroize_impl

        #[doc(hidden)]
        #drop_impl
    }
}
decl_derive!([Zeroize, attributes(zeroize)] => zeroize_derive);

/// Custom derive attributes for `Zeroize`
struct ZeroizeDeriveAttrs {
    /// Disable the on-by-default `Drop` derive
    no_drop: bool,
}

impl Default for ZeroizeDeriveAttrs {
    fn default() -> Self {
        Self { no_drop: false }
    }
}

impl ZeroizeDeriveAttrs {
    /// Parse attributes from the incoming AST
    fn parse(s: &synstructure::Structure) -> Self {
        let mut result = Self::default();

        for v in s.variants().iter() {
            for attr in v.ast().attrs.iter() {
                if attr.path.is_ident(ZEROIZE_ATTR) {
                    // TODO(tarcieri): hax, but probably good enough for now
                    match attr.tts.to_string().as_ref() {
                        "( drop )" => (), // enabled by default
                        "( no_drop )" => result.no_drop = true,
                        other => panic!("unknown zeroize attribute: {}", other),
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use synstructure::test_derive;

    #[test]
    fn zeroize() {
        test_derive! {
            zeroize_derive {
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
