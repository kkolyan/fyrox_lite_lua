use std::{fs, io, process::{self, Stdio}};

#[derive(Default)]
pub struct SimpleRustCodeBase {
    pub mods: Vec<Mod>,
}
impl SimpleRustCodeBase {
    pub fn write(&self, target_dir: &str) {
        write_mods(target_dir, &self.mods);
    }
}

pub struct Mod {
    pub name: String,
    pub content: ModContent,
}

pub enum ModContent {
    Children(Vec<Mod>),
    Code(String)
}

impl Mod {
    fn write(&self, parent_dir: &str)  {
        match &self.content {
            ModContent::Children(children) => {

                let dir = format!("{}/{}", parent_dir, self.name);
                
                write_mods(&dir, children);
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
}

fn write_mods(dir: &str, children: &[Mod])  {
    let _ = fs::create_dir_all(&dir);

    let lib_rs = children
        .iter()
        .map(|it| format!("pub mod {};\n", it.name))
        .collect::<Vec<_>>()
        .join("");

    fs::write(format!("{}/mod.rs", &dir), lib_rs).unwrap();

    for m in children.iter() {
        m.write(dir);
    }
}