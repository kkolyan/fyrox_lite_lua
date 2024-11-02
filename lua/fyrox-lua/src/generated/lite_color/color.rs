
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::let_and_return)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_match)]
#![allow(clippy::let_unit_value)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
};

impl FyroxUserData for fyrox_lite_color::lite_color::Color {
    const CLASS_NAME: &'static str = "Color";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }
    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {}
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("WHITE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::WHITE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("BLACK", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::BLACK;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("RED", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::RED;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("TRANSPARENT", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::TRANSPARENT;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MAROON", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MAROON;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_RED", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_RED;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("BROWN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::BROWN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("FIREBRICK", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::FIREBRICK;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("CRIMSON", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::CRIMSON;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("TOMATO", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::TOMATO;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("CORAL", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::CORAL;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("INDIAN_RED", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::INDIAN_RED;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_CORAL", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_CORAL;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_SALMON", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_SALMON;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SALMON", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SALMON;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_SALMON", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_SALMON;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("ORANGE_RED", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::ORANGE_RED;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_ORANGE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_ORANGE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("ORANGE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::ORANGE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("GOLD", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::GOLD;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_GOLDEN_ROD", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_GOLDEN_ROD;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("GOLDEN_ROD", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::GOLDEN_ROD;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PALE_GOLDEN_ROD", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PALE_GOLDEN_ROD;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_KHAKI", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_KHAKI;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("KHAKI", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::KHAKI;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("OLIVE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::OLIVE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("YELLOW", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::YELLOW;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("YELLOW_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::YELLOW_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_OLIVE_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_OLIVE_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("OLIVE_DRAB", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::OLIVE_DRAB;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LAWN_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LAWN_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("CHARTREUSE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::CHARTREUSE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("GREEN_YELLOW", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::GREEN_YELLOW;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("FOREST_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::FOREST_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIME", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIME;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIME_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIME_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PALE_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PALE_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_SEA_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_SEA_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_SPRING_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_SPRING_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SPRING_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SPRING_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SEA_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SEA_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_AQUA_MARINE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_AQUA_MARINE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_SEA_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_SEA_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_SEA_GREEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_SEA_GREEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_SLATE_GRAY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_SLATE_GRAY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("TEAL", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::TEAL;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_CYAN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_CYAN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("AQUA", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::AQUA;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("CYAN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::CYAN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_CYAN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_CYAN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_TURQUOISE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_TURQUOISE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("TURQUOISE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::TURQUOISE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_TURQUOISE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_TURQUOISE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PALE_TURQUOISE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PALE_TURQUOISE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("AQUA_MARINE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::AQUA_MARINE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("POWDER_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::POWDER_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("CADET_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::CADET_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("STEEL_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::STEEL_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("CORN_FLOWER_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::CORN_FLOWER_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DEEP_SKY_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DEEP_SKY_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DODGER_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DODGER_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SKY_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SKY_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_SKY_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_SKY_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MIDNIGHT_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MIDNIGHT_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("NAVY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::NAVY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("ROYAL_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::ROYAL_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("BLUE_VIOLET", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::BLUE_VIOLET;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("INDIGO", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::INDIGO;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_SLATE_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_SLATE_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SLATE_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SLATE_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_SLATE_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_SLATE_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_PURPLE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_PURPLE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_MAGENTA", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_MAGENTA;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_VIOLET", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_VIOLET;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_ORCHID", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_ORCHID;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_ORCHID", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_ORCHID;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PURPLE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PURPLE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("THISTLE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::THISTLE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PLUM", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PLUM;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("VIOLET", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::VIOLET;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MAGENTA", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MAGENTA;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("ORCHID", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::ORCHID;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MEDIUM_VIOLET_RED", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MEDIUM_VIOLET_RED;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PALE_VIOLET_RED", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PALE_VIOLET_RED;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DEEP_PINK", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DEEP_PINK;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("HOT_PINK", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::HOT_PINK;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_PINK", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_PINK;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PINK", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PINK;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("ANTIQUE_WHITE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::ANTIQUE_WHITE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("BEIGE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::BEIGE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("BISQUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::BISQUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("BLANCHED_ALMOND", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::BLANCHED_ALMOND;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("WHEAT", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::WHEAT;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("CORN_SILK", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::CORN_SILK;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LEMON_CHIFFON", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LEMON_CHIFFON;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_GOLDEN_ROD_YELLOW", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_GOLDEN_ROD_YELLOW;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_YELLOW", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_YELLOW;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SADDLE_BROWN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SADDLE_BROWN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SIENNA", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SIENNA;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("CHOCOLATE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::CHOCOLATE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PERU", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PERU;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SANDY_BROWN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SANDY_BROWN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("BURLY_WOOD", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::BURLY_WOOD;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("TAN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::TAN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("ROSY_BROWN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::ROSY_BROWN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MOCCASIN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MOCCASIN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("NAVAJO_WHITE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::NAVAJO_WHITE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PEACH_PUFF", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PEACH_PUFF;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MISTY_ROSE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MISTY_ROSE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LAVENDER_BLUSH", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LAVENDER_BLUSH;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LINEN", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LINEN;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("OLD_LACE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::OLD_LACE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("PAPAYA_WHIP", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::PAPAYA_WHIP;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SEA_SHELL", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SEA_SHELL;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("MINT_CREAM", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::MINT_CREAM;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SLATE_GRAY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SLATE_GRAY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_SLATE_GRAY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_SLATE_GRAY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_STEEL_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_STEEL_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LAVENDER", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LAVENDER;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("FLORAL_WHITE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::FLORAL_WHITE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("ALICE_BLUE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::ALICE_BLUE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("GHOST_WHITE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::GHOST_WHITE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("HONEYDEW", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::HONEYDEW;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("IVORY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::IVORY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("AZURE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::AZURE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SNOW", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SNOW;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DIM_GRAY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DIM_GRAY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("GRAY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::GRAY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("DARK_GRAY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::DARK_GRAY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("SILVER", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::SILVER;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("LIGHT_GRAY", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::LIGHT_GRAY;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("GAINSBORO", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::GAINSBORO;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
        fields.add_field_method_get("WHITE_SMOKE", |lua, this| {
            let value = fyrox_lite_color::lite_color::Color::WHITE_SMOKE;
            Ok(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                value,
            )))
        });
    }
}
