use std::fs;

use fyrox_lite_model::Domain;

use crate::{load_path::load_path, resolve_classes::resolve_classes};

pub fn generate_domain(dir: &str) -> Domain {
    println!("generating domain from dir {}", dir);
    let mut domain = Domain::default();
    let mut aliases = Default::default();
    for entry in fs::read_dir(dir).unwrap().flatten() {
        println!("checking entry {:?}", entry);
        if entry.file_type().unwrap().is_dir() {
            println!("skipping, because dir");
            continue;
        }
        if !entry.file_name().to_string_lossy().ends_with(".rs") {
            println!("skipping, because non-rust");
            continue;
        }
        println!("loading...");
        load_path(&entry.path(), &mut domain, &mut aliases);
    }

    resolve_classes(&mut domain, &mut aliases);
    let s = serde_json::to_string_pretty(&domain).unwrap();
    let deserialized : Domain = serde_json::from_str(s.as_str()).unwrap();
    assert_eq!(domain, deserialized);
    fs::write(format!("{}/domain.json", dir), s).unwrap();
	domain
}