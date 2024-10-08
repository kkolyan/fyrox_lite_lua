use std::{fs, process::{self, Stdio}};

#[derive(Default)]
pub struct HierarchicalCodeBase {
    pub mods: Vec<Module>,
}
impl HierarchicalCodeBase {
    pub fn write_rust(&self, target_dir: &str) {
        write_rust_mods(target_dir, &self.mods);
    }
    pub fn write_lua(&self, target_dir: &str) {
        write_lua_mods(target_dir, &self.mods);
    }
}

pub struct Module {
    pub name: String,
    pub content: ModContent,
}

pub enum ModContent {
    Children(Vec<Module>),
    Code(String)
}

impl Module {
    fn write_rust(&self, parent_dir: &str)  {
        match &self.content {
            ModContent::Children(children) => {

                let dir = format!("{}/{}", parent_dir, self.name);
                
                write_rust_mods(&dir, children);
            },
            ModContent::Code(code) => {
                let file = format!("{}/{}.rs", parent_dir, self.name);
                fs::write(&file, code).unwrap();
                process::Command::new("rustfmt")
                .arg(file)
                .stderr(Stdio::null())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            },
        }
    }
    fn write_lua(&self, parent_dir: &str)  {
        match &self.content {
            ModContent::Children(children) => {

                let dir = format!("{}/{}", parent_dir, self.name);
                
                write_lua_mods(&dir, children);
            },
            ModContent::Code(code) => {
                let file = format!("{}/{}.lua", parent_dir, self.name);
                fs::write(&file, code).unwrap();
                process::Command::new("rustfmt")
                .arg(file)
                .stderr(Stdio::null())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
            },
        }
    }
}

fn write_rust_mods(dir: &str, children: &[Module])  {
    let _ = fs::create_dir_all(dir);

    let lib_rs = children
        .iter()
        .map(|it| format!("pub mod {};\n", it.name))
        .collect::<Vec<_>>()
        .join("");

    fs::write(format!("{}/mod.rs", &dir), lib_rs).unwrap();

    for m in children.iter() {
        m.write_rust(dir);
    }
}

fn write_lua_mods(dir: &str, children: &[Module])  {
    let _ = fs::create_dir_all(dir);

    for m in children.iter() {
        m.write_lua(dir);
    }
}