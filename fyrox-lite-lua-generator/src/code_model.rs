use std::{fs, io};


#[derive(Default)]
pub struct SimpleRustCodeBase {
	pub mods: Vec<Mod>,
}
impl SimpleRustCodeBase {
	pub fn write(&self, target_dir: &str) -> io::Result<()> {

		let _ = fs::create_dir_all(target_dir);
	
		let lib_rs = self
			.mods
			.iter()
			.map(|it| format!("pub mod {};\n", it.name))
			.collect::<Vec<_>>()
			.join("");
	
			
		fs::write(format!("{target_dir}/mod.rs"), lib_rs)?;
	
		for m in self.mods.iter() {
			fs::write(format!("{target_dir}/{}.rs", m.name), &m.content)?;
		}
		Ok(())
	}
}

pub struct Mod {
	pub name: String,
	pub content: String,
}