use fyrox_lite_model::DataType;
use quote::ToTokens;


pub fn extract_ty(ty: &syn::Type) -> syn::Result<fyrox_lite_model::DataType> {
    match ty {
        syn::Type::Path(it) => {
			if it.qself.is_some() {
				return Err(syn::Error::new_spanned(
                    it,
                    "Fyrox Lite: Self is not allowed",
                ));
			}
			if it.path.segments.len() > 1 {
				return Err(syn::Error::new_spanned(
					&it.path.segments,
					"Fyrox Lite: qualified names not allowed",
				));
			}
			let segment = &it.path.segments[0];
			if segment.ident == "Vec" {
				return Ok(DataType::Vec(Box::new(extract_single_type_param(segment)?)));
			}
			if segment.ident == "Option" {
				return Ok(DataType::Option(Box::new(extract_single_type_param(segment)?)));
			}
			let syn::PathArguments::None = &segment.arguments else {
				return Err(syn::Error::new_spanned(
					&it.path.segments,
					"Fyrox Lite: generics allowed for `Vec`, `&mut dyn DynamicArray` and `Option` only",
				));
			};
			Ok(match segment.ident.to_string().as_str() {
				"bool" => DataType::Bool,
				"i32" => DataType::I32,
				"i64" => DataType::I64,
				"f32" => DataType::F32,
				"f64" => DataType::F64,
				"String" => DataType::String,
				_=> DataType::UnresolvedClass(segment.ident.to_string()),
			})
		},
        syn::Type::Reference(ref_) => {
            if ref_.lifetime.is_some() {
                return Err(syn::Error::new_spanned(
                    &ref_.lifetime,
                    "Fyrox Lite: lifetimes not allowed",
                ));
            }
            match ref_.elem.as_ref() {
                syn::Type::TraitObject(it) => {
                    if it.bounds.len() > 1 {
                        return Err(syn::Error::new_spanned(
                            &it.bounds,
                            "Fyrox Lite: trait objects not allowed",
                        ));
                    }
                    match &it.bounds[0] {
                        syn::TypeParamBound::Trait(it) => {
                            let syn::TraitBoundModifier::None = &it.modifier else {
                                return Err(syn::Error::new_spanned(
                                    it.modifier,
                                    "Fyrox Lite: trait bound modifiers not allowed",
                                ));
                            };
                            if it.lifetimes.is_some() {
                                return Err(syn::Error::new_spanned(
                                    &it.lifetimes,
                                    "Fyrox Lite: lifetimes not allowed",
                                ));
                            }
                            if it.path.segments.len() > 1 {
                                return Err(syn::Error::new_spanned(
                                    &it.path.segments,
                                    "Fyrox Lite: qualified names not allowed (2)",
                                ));
                            }
                            let segment = &it.path.segments[0];
                            if segment.ident == "DynamicArray" {
                                if ref_.mutability.is_none() {
                                    return Err(syn::Error::new_spanned(
                                        ref_.mutability,
                                        "Fyrox Lite: DynamicArray referene should be mutable",
                                    ));
                                }
								return Ok(DataType::DynamicArray(Box::new(extract_single_type_param(segment)?)));
                            }
                            Err(syn::Error::new_spanned(it, "unknown language provided type"))
                        }
                        _ => {
                            Err(syn::Error::new_spanned(
                                &it.bounds,
                                "Fyrox Lite: non-trait bounds not allowed",
                            ))
                        }
                    }
                }
                _ => Err(syn::Error::new_spanned(
                    ty,
                    format!("Fyrox Lite: references not allowed (except for some special types). {}", ty.to_token_stream()),
                )),
            }
        }
        _ => Err(syn::Error::new_spanned(
			ty,
			"Fyrox Lite: not allowed type",
		)),
    }
}

fn extract_single_type_param(segment: &syn::PathSegment) -> Result<DataType, syn::Error> {
	
	if let syn::PathArguments::AngleBracketed(it) = &segment.arguments {
		if it.args.len() == 1 {
			if let syn::GenericArgument::Type(ty) = &it.args[0] {
				return extract_ty(ty);
			}
		}
	}
	Err(syn::Error::new_spanned(
		segment,
		"Fyrox Lite: expected to have a single type parameter",
	))
}