use std::fmt::{Display, Write};



#[extend::ext]
pub impl &str {
	fn var(&self, name: &str, value: impl Display) -> String {
		let placeholder = &format!("${{{}}}",name);
		assert!(self.contains(placeholder));
		self.replace(placeholder, &format!("{}", value))
	}
}

#[extend::ext]
pub impl String {
	fn var(&self, name: &str, value: impl Display) -> String {
		self.as_str().var(name, value)
	}
}

pub fn render<const N: usize>(target: &mut dyn Write, template: &str, vars: [(&str, &dyn Display); N]) {
	let mut s = template.to_string();
	
	for (name, value) in vars {
		let placeholder = &format!("${{{}}}",name);
		assert!(template.contains(placeholder), "missing placeholder for {}", name);
		s = s.replace(placeholder, format!("{}", value).as_str());
	}
	target.write_str(s.as_str()).unwrap();
}