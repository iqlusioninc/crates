//! Custom derive support for `zeroize`

#![crate_type = "proc-macro"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

macro_rules! q {
    ($($t:tt)*) => (quote_spanned!(proc_macro2::Span::call_site() => $($t)*))
}

#[proc_macro_derive(Zeroize)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let zeroizers = match ast.data {
        syn::Data::Struct(ref s) => derive_struct_zeroizers(&s.fields),
        syn::Data::Enum(_) => panic!("support for deriving Zeroize on enums not yet unimplemented"),
        syn::Data::Union(_) => panic!("can't derive Zeroize on union types"),
    };

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let zeroize_impl = q! {
        impl #impl_generics Zeroize for #name #ty_generics #where_clause {
            fn zeroize(&mut self) {
                #zeroizers
            }
        }
    };

    zeroize_impl.into()
}

fn derive_struct_zeroizers(fields: &syn::Fields) -> proc_macro2::TokenStream {
    let self_ident = syn::Ident::new("self", proc_macro2::Span::call_site());

    match *fields {
        syn::Fields::Named(ref fields) => {
            derive_field_zeroizers(&self_ident, Some(&fields.named), true)
        }
        syn::Fields::Unnamed(ref fields) => {
            derive_field_zeroizers(&self_ident, Some(&fields.unnamed), false)
        }
        syn::Fields::Unit => panic!("can't derive Zeroize on unit structs"),
    }
}

fn derive_field_zeroizers(
    target: &syn::Ident,
    fields: Option<&syn::punctuated::Punctuated<syn::Field, Token![,]>>,
    named: bool,
) -> proc_macro2::TokenStream {
    let empty = Default::default();
    let zeroizers = fields.unwrap_or(&empty).iter().enumerate().map(|(i, f)| {
        let is_phantom_data = match f.ty {
            syn::Type::Path(syn::TypePath {
                qself: None,
                ref path,
            }) => path
                .segments
                .last()
                .map(|x| x.value().ident == "PhantomData")
                .unwrap_or(false),
            _ => false,
        };

        if is_phantom_data {
            q!()
        } else if named {
            let ident = f.ident.clone().unwrap();
            q!(#target.#ident.zeroize())
        } else {
            q!(#target.#i.zeroize())
        }
    });

    q!(#(#zeroizers);*)
}
