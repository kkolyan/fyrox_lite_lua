use std::collections::HashMap;

use fyrox_lite_model::{Domain, RustQualifiedName};

pub struct GenerationContext {
	pub internal_to_external: HashMap<RustQualifiedName, RustQualifiedName>,
	pub domain: Domain,
}