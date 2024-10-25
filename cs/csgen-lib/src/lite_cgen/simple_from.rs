use std::{collections::HashSet, fmt::Display};

use gen_common::templating::render;
use lite_model::{ClassName, DataType};

use super::types;

pub fn generate_from<'a>(s: &mut String, from: &dyn Display, to: &dyn Display, client_replicated_types: &HashSet<ClassName>, fields: impl Iterator<Item = (&'a str, &'a DataType)>) {

    let mut field_names = vec![];

    render(
        s,
        r#"
            impl From<${from}> for ${to} {
                fn from(__value: ${from}) -> Self {
    "#,
        [("from", from), ("to", to)],
    );

    for (name, ty) in fields {
        field_names.push(name.to_string());

        render(s, "
                    let ${name} = __value.${name};
                    let ${name} = ${expr};
        ", [
            ("name", &name),
            ("expr", &types::generate_to_native(&ty, &name, client_replicated_types)),
        ]);
    }

    render(
        s,
        r#"
                    Self { ${field_names} }
                }
            }
    "#,
        [("field_names", &field_names.join(", "))],
    );
}