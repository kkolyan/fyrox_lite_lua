use lite_model::Feature;
use proc_macro2::TokenStream;
use syn::{parse::Parse, parse2, Ident, Token};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct LiteApiAttr {
    pub class: Option<String>,
    pub features: Vec<Feature>,
}

impl Parse for LiteApiAttr {

    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut result: LiteApiAttr = Default::default();
        loop {
            if input.is_empty() {
                break;
            }
            let ident = input.parse::<Ident>()?;
            if ident == "class" {
                input.parse::<Token![=]>()?;
                result.class = Some(input.parse::<Ident>()?.to_string());
            } else if ident == "eq" {
                result.features.push(Feature::Eq);
            } else {
                return Err(syn::Error::new_spanned(
                    ident,
                    "unsupported option. supported: ".to_string() + LiteApiAttr::OPTIONS_HINT,
                ));
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(result)
    }
}

impl LiteApiAttr {
    pub const OPTIONS_HINT: &str = "[class=*][,eq]";

    pub fn from_attr_args(attr_args: TokenStream) -> syn::Result<LiteApiAttr> {
        parse2::<LiteApiAttr>(attr_args)
    }
}

#[cfg(test)]
mod tests {
    use lite_model::Feature;
    use quote::quote;

    use crate::lite_api_attr::LiteApiAttr;

    #[test]
    fn all_is_ok() {
        let a = LiteApiAttr::from_attr_args(quote! {class=Cat,eq}).unwrap();
        assert_eq!(
            a,
            LiteApiAttr {
                class: Some("Cat".to_owned()),
                features: vec![Feature::Eq]
            }
        );
    }

    #[test]
    fn only_class_is_ok() {
        let a = LiteApiAttr::from_attr_args(quote! {class=Cat}).unwrap();
        assert_eq!(
            a,
            LiteApiAttr {
                class: Some("Cat".to_owned()),
                features: vec![]
            }
        );
    }

    #[test]
    fn only_eq_is_ok() {
        let a = LiteApiAttr::from_attr_args(quote! {eq}).unwrap();
        assert_eq!(
            a,
            LiteApiAttr {
                class: None,
                features: vec![Feature::Eq]
            }
        );
    }

    #[test]
    fn empty_is_ok() {
        let a = LiteApiAttr::from_attr_args(quote! {}).unwrap();
        assert_eq!(
            a,
            LiteApiAttr {
                class: None,
                features: vec![]
            }
        );
    }

    #[test]
    fn order_doesnt_matter() {
        let a = LiteApiAttr::from_attr_args(quote! {eq,class=Cat}).unwrap();
        assert_eq!(
            a,
            LiteApiAttr {
                class: Some("Cat".to_owned()),
                features: vec![Feature::Eq]
            }
        );
    }

    #[test]
    fn trailing_comma_is_ok() {
        let a = LiteApiAttr::from_attr_args(quote! {eq,class=Cat,}).unwrap();
        assert_eq!(
            a,
            LiteApiAttr {
                class: Some("Cat".to_owned()),
                features: vec![Feature::Eq]
            }
        );
    }

    #[test]
    fn spaces_are_ok() {
        let a = LiteApiAttr::from_attr_args(quote! {eq, class=Cat}).unwrap();
        assert_eq!(
            a,
            LiteApiAttr {
                class: Some("Cat".to_owned()),
                features: vec![Feature::Eq]
            }
        );
    }

    #[test]
    fn unknown_option_is_bad() {
        let a = LiteApiAttr::from_attr_args(quote! {eq,clasz=Cat,}).err().unwrap();
        assert!(a.to_string().contains("unsupported option"));
    }
}
