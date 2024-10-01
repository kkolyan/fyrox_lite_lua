pub mod fyrox_lite;
use std::ops::Deref;

use fyrox_lite_model::{DataType, Domain};
use fyrox_lite_parser::{extract_ty::extract_ty, extract_engine_class::extract_engine_class_and_inject_assertions, extract_pod_struct::extract_pod_struct};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse::{self, Parse},
    parse2, parse_quote,
    spanned::Spanned,
    Ident,
};
use uuid::Uuid;
// use quote::quote;


pub fn fyrox_lite_user_class(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let error = syn::Error::new_spanned(_attr, "Fyrox Lite: fyrox_lite_user_class is not implemented yet").into_compile_error();
    quote! {
        #error
        #item
    }
}


fn generate_static_assertions<'a>(items: impl Iterator<Item = &'a syn::Type>) -> TokenStream {
    items
        .map(|ty| {
            let id = Uuid::new_v4().to_string().replace("-", "_");
            let span = ty.span();
            let static_assertions_fn =
                Ident::new(format!("static_assertions_for_{}", id).as_str(), span);
            quote_spanned! {span =>
                #[allow(dead_code)]
                fn #static_assertions_fn() {
                    use crate::LiteDataType;
                    <#ty>::compiles_if_type_is_allowed();
                }
            }
        })
        .collect::<TokenStream>()
}
