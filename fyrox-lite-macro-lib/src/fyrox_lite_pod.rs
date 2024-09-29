use std::ops::Deref;

use fyrox_lite_model::{DataType, Domain};
use fyrox_lite_parser::{extract_ty::extract_ty, visit_impl::extract_engine_class, visit_struct::extract_pod_struct};
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

pub fn fyrox_lite_pod(attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse2::<syn::Item>(item) {
        Ok(it) => match it {
            syn::Item::Enum(_) => {
                quote! {
                    #[derive(Debug, Clone)]
                    #it
                }
            }
            syn::Item::Struct(struct_) => {
                let mut errors = Vec::new();

                extract_pod_struct(
                    attr,
                    &struct_,
                    &mut errors,
                );

                let ident = &struct_.ident;

                let errors = errors
                    .into_iter()
                    .map(|it| it.into_compile_error())
                    .collect::<TokenStream>();

                let field_assertions =
                    generate_static_assertions(struct_.fields.iter().map(|it| &it.ty));
                quote! {
                    #errors
                    #[derive(Debug, Clone)]
                    #struct_

                    impl crate::LiteDataType for #ident {}
                    #field_assertions

                }
            }
            it => {
                let error = syn::Error::new(
                    attr.span(),
                    "fyrox_lite_pod allowed only for struct or enum declarations",
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