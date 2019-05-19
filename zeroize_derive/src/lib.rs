//! Custom derive support for `zeroize`

#![crate_type = "proc-macro"]
#![deny(warnings, unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]

extern crate proc_macro;

use quote::quote;
use synstructure::{decl_derive, BindStyle};

/// Custom derive for `Zeroize`
fn zeroize_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    s.bind_with(|_| BindStyle::RefMut);

    let body = s.each(|bi| quote! { #bi.zeroize(); });

    s.bound_impl(
        quote!(zeroize::Zeroize),
        quote! {
            fn zeroize(&mut self) {
                match self {
                    #body
                }
            }
        },
    )
}
decl_derive!([Zeroize] => zeroize_derive);

/// Custom derive for `ZeroizeOnDrop`
fn zeroize_on_drop_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let drop_impl = s.gen_impl(quote! {
        gen impl Drop for @Self {
            fn drop(&mut self) {
                self.zeroize();
            }
        }
    });

    let zeroize_on_drop_impl = s.bound_impl(quote!(zeroize::ZeroizeOnDrop), quote!());

    quote! {
        #[doc(hidden)]
        #drop_impl
        #zeroize_on_drop_impl
    }
}
decl_derive!([ZeroizeOnDrop] => zeroize_on_drop_derive);

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
            }
            no_build // tests the code compiles are in the `zeroize` crate
        }
    }

    #[test]
    fn zeroize_on_drop() {
        test_derive! {
            zeroize_on_drop_derive {
                struct Z {
                    a: String,
                    b: Vec<u8>,
                    c: [u8; 3],
                }
            }
            expands to {
                #[doc(hidden)]
                #[allow(non_upper_case_globals)]
                const _DERIVE_Drop_FOR_Z: () = {
                    impl Drop for Z {
                        fn drop(&mut self) {
                            self.zeroize();
                        }
                    }
                };
                #[allow(non_upper_case_globals)]
                #[doc(hidden)]
                const _DERIVE_zeroize_ZeroizeOnDrop_FOR_Z : () = {
                    extern crate zeroize;
                    impl zeroize :: ZeroizeOnDrop for Z {}
                };
            }
            no_build // tests the code compiles are in the `zeroize` crate
        }
    }
}
