use std::ops::Deref;

use fyrox_lite_model::{DataType, Domain};
use fyrox_lite_parser::{
    extract_ty::extract_ty, extract_engine_class::extract_engine_class_and_inject_assertions, extract_pod_struct::extract_pod_struct,
};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse::{self, Parse},
    parse2, parse_quote,
    spanned::Spanned,
    Ident,
};
use uuid::Uuid;

use crate::generate_static_assertions;

pub fn fyrox_lite_engine_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse2::<syn::Item>(item) {
        Ok(it) => match it {
            syn::Item::Impl(mut item) => {
                let mut errors = Vec::new();

                let ident = extract_engine_class_and_inject_assertions(attr, &mut item, &mut errors)
                    .map(|(rust_class_name, _class)| rust_class_name);

                let errors = errors
                    .into_iter()
                    .map(|it| it.into_compile_error())
                    .collect::<TokenStream>();

                let impl_lite_data_type =
                    ident.map(|ident| quote! {impl crate::LiteDataType for #ident {}});

                quote! {
                    #errors
                    #item

                    #impl_lite_data_type
                }
            }
            it => {
                let error = syn::Error::new(
                    attr.span(),
                    "fyrox_lite_engine_class allowed only for impl declarations",
                )
                .into_compile_error();
                quote! {
                    #error
                    #it
                }
            }
        },
        Err(err) => err.to_compile_error(),
    }
}
