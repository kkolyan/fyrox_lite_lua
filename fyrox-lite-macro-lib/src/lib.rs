use proc_macro2::TokenStream;
// use quote::quote;


pub fn fyrox_lite_engine_class(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // quote! {
    //     #item
    // }
    item
}

pub fn fyrox_lite_user_class(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // quote! {
    //     #item
    // }
    item
}

pub fn fyrox_lite_pod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // quote! {
    //     #[derive(Debug, Copy, Clone)]
    //     #item
    // }
    item
}