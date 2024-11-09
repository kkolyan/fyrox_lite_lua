use std::collections::HashMap;
use to_vec::ToVec;
use lite_model::{ClassName, Domain};

pub fn classes_by_package(domain: &Domain) -> Vec<(String, Vec<ClassName>)> {
    let mut by_package: HashMap<String, Vec<ClassName>> = Default::default();
    for class in domain.classes.iter() {
        let rust_name_without_crate = class.rust_name().0.split_once("::").unwrap().1;
        let package = rust_name_without_crate.split_once("::").unwrap().0;
        by_package
            .entry(package.to_string())
            .or_default()
            .push(class.class_name().clone());
    }
    let mut result = by_package.into_iter().to_vec();
    result.sort_by_key(|(package, _)| package.to_string());
    result
}