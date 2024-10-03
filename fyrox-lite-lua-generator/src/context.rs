use std::collections::HashMap;

use fyrox_lite_model::{ClassName, Domain, RustQualifiedName};

pub struct GenerationContext {
	pub internal_to_external: HashMap<RustQualifiedName, ClassName>,
	pub domain: Domain,
}