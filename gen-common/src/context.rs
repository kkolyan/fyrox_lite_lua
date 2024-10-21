use std::collections::HashMap;

use lite_model::{ClassName, Domain, RustQualifiedName};

pub struct GenerationContext<'a> {
	pub internal_to_external: HashMap<RustQualifiedName, ClassName>,
	pub domain: &'a Domain,
}