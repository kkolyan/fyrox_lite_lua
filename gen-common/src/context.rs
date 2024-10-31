use std::collections::HashMap;

use lite_model::{Class, ClassName, DataType, Domain, RustQualifiedName};

pub struct GenerationContext<'a> {
	pub internal_to_external: HashMap<RustQualifiedName, ClassName>,
	pub domain: &'a Domain,
}

impl GenerationContext<'_> {
	pub fn is_struct(&self, ty: &DataType) -> bool {
		if let DataType::Object(class_name) = ty {
			if let Some(Class::Struct(_)) = self.domain.get_class(class_name) {
				return true;
			}
		}
		false
	}
}