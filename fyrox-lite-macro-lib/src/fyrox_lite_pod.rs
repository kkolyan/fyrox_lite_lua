use std::ops::Deref;

use fyrox_lite_model::{DataType, Domain};
use fyrox_lite_parser::{extract_engine_class::extract_engine_class, extract_pod_enum::extract_pod_enum, extract_pod_struct::extract_pod_struct, extract_ty::extract_ty};
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
            syn::Item::Enum(item) => {
                let mut errors = Vec::new();

                let mut types = Vec::new();
                let ident = extract_pod_enum(attr, &item, &mut errors, &mut types)
                    .map(|(rust_class_name, _class)| rust_class_name);

                let field_assertions = generate_static_assertions(types.iter());

                let errors = errors
                    .into_iter()
                    .map(|it| it.into_compile_error())
                    .collect::<TokenStream>();

                let impl_lite_data_type =
                    ident.map(|ident| quote! {impl crate::LiteDataType for #ident {}});

                quote! {
                    #errors
                    #[derive(Debug, Clone)]
                    #item

                    #impl_lite_data_type
                    #field_assertions
                }
            }
            syn::Item::Struct(item) => {
                let mut errors = Vec::new();

                extract_pod_struct(
                    attr,
                    &item,
                    &mut errors,
                );

                let ident = &item.ident;

                let errors = errors
                    .into_iter()
                    .map(|it| it.into_compile_error())
                    .collect::<TokenStream>();

                let field_assertions =
                    generate_static_assertions(item.fields.iter().map(|it| &it.ty));
                quote! {
                    #errors
                    #[derive(Debug, Clone)]
                    #item

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