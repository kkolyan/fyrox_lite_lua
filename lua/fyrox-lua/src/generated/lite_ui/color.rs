
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
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};

impl FyroxUserData for fyrox_lite::lite_ui::Color {
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
            let value = fyrox_lite::lite_ui::Color::WHITE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("BLACK", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::BLACK;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("RED", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::RED;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("TRANSPARENT", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::TRANSPARENT;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MAROON", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MAROON;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_RED", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_RED;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("BROWN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::BROWN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("FIREBRICK", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::FIREBRICK;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("CRIMSON", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::CRIMSON;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("TOMATO", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::TOMATO;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("CORAL", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::CORAL;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("INDIAN_RED", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::INDIAN_RED;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_CORAL", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_CORAL;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_SALMON", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_SALMON;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SALMON", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SALMON;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_SALMON", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_SALMON;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("ORANGE_RED", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::ORANGE_RED;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_ORANGE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_ORANGE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("ORANGE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::ORANGE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("GOLD", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::GOLD;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_GOLDEN_ROD", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_GOLDEN_ROD;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("GOLDEN_ROD", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::GOLDEN_ROD;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PALE_GOLDEN_ROD", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PALE_GOLDEN_ROD;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_KHAKI", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_KHAKI;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("KHAKI", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::KHAKI;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("OLIVE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::OLIVE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("YELLOW", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::YELLOW;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("YELLOW_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::YELLOW_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_OLIVE_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_OLIVE_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("OLIVE_DRAB", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::OLIVE_DRAB;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LAWN_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LAWN_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("CHARTREUSE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::CHARTREUSE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("GREEN_YELLOW", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::GREEN_YELLOW;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("FOREST_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::FOREST_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIME", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIME;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIME_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIME_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PALE_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PALE_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_SEA_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_SEA_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_SPRING_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_SPRING_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SPRING_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SPRING_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SEA_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SEA_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_AQUA_MARINE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_AQUA_MARINE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_SEA_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_SEA_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_SEA_GREEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_SEA_GREEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_SLATE_GRAY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_SLATE_GRAY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("TEAL", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::TEAL;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_CYAN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_CYAN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("AQUA", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::AQUA;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("CYAN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::CYAN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_CYAN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_CYAN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_TURQUOISE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_TURQUOISE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("TURQUOISE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::TURQUOISE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_TURQUOISE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_TURQUOISE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PALE_TURQUOISE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PALE_TURQUOISE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("AQUA_MARINE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::AQUA_MARINE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("POWDER_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::POWDER_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("CADET_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::CADET_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("STEEL_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::STEEL_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("CORN_FLOWER_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::CORN_FLOWER_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DEEP_SKY_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DEEP_SKY_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DODGER_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DODGER_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SKY_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SKY_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_SKY_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_SKY_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MIDNIGHT_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MIDNIGHT_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("NAVY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::NAVY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("ROYAL_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::ROYAL_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("BLUE_VIOLET", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::BLUE_VIOLET;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("INDIGO", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::INDIGO;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_SLATE_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_SLATE_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SLATE_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SLATE_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_SLATE_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_SLATE_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_PURPLE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_PURPLE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_MAGENTA", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_MAGENTA;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_VIOLET", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_VIOLET;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_ORCHID", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_ORCHID;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_ORCHID", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_ORCHID;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PURPLE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PURPLE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("THISTLE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::THISTLE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PLUM", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PLUM;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("VIOLET", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::VIOLET;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MAGENTA", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MAGENTA;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("ORCHID", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::ORCHID;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MEDIUM_VIOLET_RED", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MEDIUM_VIOLET_RED;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PALE_VIOLET_RED", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PALE_VIOLET_RED;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DEEP_PINK", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DEEP_PINK;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("HOT_PINK", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::HOT_PINK;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_PINK", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_PINK;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PINK", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PINK;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("ANTIQUE_WHITE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::ANTIQUE_WHITE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("BEIGE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::BEIGE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("BISQUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::BISQUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("BLANCHED_ALMOND", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::BLANCHED_ALMOND;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("WHEAT", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::WHEAT;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("CORN_SILK", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::CORN_SILK;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LEMON_CHIFFON", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LEMON_CHIFFON;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_GOLDEN_ROD_YELLOW", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_GOLDEN_ROD_YELLOW;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_YELLOW", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_YELLOW;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SADDLE_BROWN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SADDLE_BROWN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SIENNA", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SIENNA;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("CHOCOLATE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::CHOCOLATE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PERU", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PERU;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SANDY_BROWN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SANDY_BROWN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("BURLY_WOOD", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::BURLY_WOOD;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("TAN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::TAN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("ROSY_BROWN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::ROSY_BROWN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MOCCASIN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MOCCASIN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("NAVAJO_WHITE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::NAVAJO_WHITE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PEACH_PUFF", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PEACH_PUFF;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MISTY_ROSE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MISTY_ROSE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LAVENDER_BLUSH", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LAVENDER_BLUSH;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LINEN", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LINEN;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("OLD_LACE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::OLD_LACE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("PAPAYA_WHIP", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::PAPAYA_WHIP;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SEA_SHELL", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SEA_SHELL;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("MINT_CREAM", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::MINT_CREAM;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SLATE_GRAY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SLATE_GRAY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_SLATE_GRAY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_SLATE_GRAY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_STEEL_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_STEEL_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LAVENDER", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LAVENDER;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("FLORAL_WHITE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::FLORAL_WHITE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("ALICE_BLUE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::ALICE_BLUE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("GHOST_WHITE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::GHOST_WHITE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("HONEYDEW", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::HONEYDEW;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("IVORY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::IVORY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("AZURE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::AZURE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SNOW", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SNOW;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DIM_GRAY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DIM_GRAY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("GRAY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::GRAY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("DARK_GRAY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::DARK_GRAY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("SILVER", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::SILVER;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("LIGHT_GRAY", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::LIGHT_GRAY;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("GAINSBORO", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::GAINSBORO;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });

        fields.add_field_method_get("WHITE_SMOKE", |lua, this| {
            let value = fyrox_lite::lite_ui::Color::WHITE_SMOKE;
            Ok(Traitor::new(fyrox_lite::lite_ui::Color::from(value)))
        });
    }
}
