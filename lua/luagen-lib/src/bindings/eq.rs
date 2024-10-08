use lite_model::Feature;

pub(crate) fn generate_eq(s: &mut String, features: &[Feature]) {
    for feature in features {
        match feature {
            lite_model::Feature::Eq => {
                *s += r#"
                methods.add_meta_method(mlua::MetaMethod::Eq.name(), |lua, this, args: TypedUserData<Traitor<Self>>| {
                    Ok(<Self as PartialEq>::eq(this.inner(), args.borrow()?.inner()))
                });
                "#;
            },
        }
    }
}