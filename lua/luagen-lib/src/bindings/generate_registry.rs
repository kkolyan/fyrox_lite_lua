use lite_model::Class;

use crate::{code_model::{Module, ModContent}, context::GenerationContext, templating::render};


pub fn generate_registry(ctx: &GenerationContext) -> Module {
    let mut s = String::new();
    render(
        &mut s,
        r#"
        pub fn register_classes(lua: &mlua::Lua) {
            use crate::fyrox_lite_class::FyroxUserData;
    "#,
        [],
    );
    for class in ctx.domain.classes.iter() {
        let provides_class = match class {
            Class::Engine(_) => true,
            Class::Struct(_) => false,
            Class::Enum(_) => true,
        };
        if provides_class {
            s += format!(
                "
            {}::register_class(lua);
            ",
                class.rust_name()
            )
            .as_str();
        }
    }
    s += r#"
        }
    "#;
    Module {
        name: "registry".into(),
        content: ModContent::Code(s),
    }
}
