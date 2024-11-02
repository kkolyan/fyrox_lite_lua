use lite_macro::lite_api;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[lite_api]
impl Color {
    pub const WHITE: Color = color_to_lite(fyrox_core::color::Color::WHITE);
    pub const BLACK: Color = color_to_lite(fyrox_core::color::Color::BLACK);
    pub const RED: Color = color_to_lite(fyrox_core::color::Color::RED);
    pub const GREEN: Color = color_to_lite(fyrox_core::color::Color::GREEN);
    pub const BLUE: Color = color_to_lite(fyrox_core::color::Color::BLUE);
    pub const TRANSPARENT: Color = color_to_lite(fyrox_core::color::Color::TRANSPARENT);
    pub const MAROON: Color = color_to_lite(fyrox_core::color::Color::MAROON);
    pub const DARK_RED: Color = color_to_lite(fyrox_core::color::Color::DARK_RED);
    pub const BROWN: Color = color_to_lite(fyrox_core::color::Color::BROWN);
    pub const FIREBRICK: Color = color_to_lite(fyrox_core::color::Color::FIREBRICK);
    pub const CRIMSON: Color = color_to_lite(fyrox_core::color::Color::CRIMSON);
    pub const TOMATO: Color = color_to_lite(fyrox_core::color::Color::TOMATO);
    pub const CORAL: Color = color_to_lite(fyrox_core::color::Color::CORAL);
    pub const INDIAN_RED: Color = color_to_lite(fyrox_core::color::Color::INDIAN_RED);
    pub const LIGHT_CORAL: Color = color_to_lite(fyrox_core::color::Color::LIGHT_CORAL);
    pub const DARK_SALMON: Color = color_to_lite(fyrox_core::color::Color::DARK_SALMON);
    pub const SALMON: Color = color_to_lite(fyrox_core::color::Color::SALMON);
    pub const LIGHT_SALMON: Color = color_to_lite(fyrox_core::color::Color::LIGHT_SALMON);
    pub const ORANGE_RED: Color = color_to_lite(fyrox_core::color::Color::ORANGE_RED);
    pub const DARK_ORANGE: Color = color_to_lite(fyrox_core::color::Color::DARK_ORANGE);
    pub const ORANGE: Color = color_to_lite(fyrox_core::color::Color::ORANGE);
    pub const GOLD: Color = color_to_lite(fyrox_core::color::Color::GOLD);
    pub const DARK_GOLDEN_ROD: Color = color_to_lite(fyrox_core::color::Color::DARK_GOLDEN_ROD);
    pub const GOLDEN_ROD: Color = color_to_lite(fyrox_core::color::Color::GOLDEN_ROD);
    pub const PALE_GOLDEN_ROD: Color = color_to_lite(fyrox_core::color::Color::PALE_GOLDEN_ROD);
    pub const DARK_KHAKI: Color = color_to_lite(fyrox_core::color::Color::DARK_KHAKI);
    pub const KHAKI: Color = color_to_lite(fyrox_core::color::Color::KHAKI);
    pub const OLIVE: Color = color_to_lite(fyrox_core::color::Color::OLIVE);
    pub const YELLOW: Color = color_to_lite(fyrox_core::color::Color::YELLOW);
    pub const YELLOW_GREEN: Color = color_to_lite(fyrox_core::color::Color::YELLOW_GREEN);
    pub const DARK_OLIVE_GREEN: Color = color_to_lite(fyrox_core::color::Color::DARK_OLIVE_GREEN);
    pub const OLIVE_DRAB: Color = color_to_lite(fyrox_core::color::Color::OLIVE_DRAB);
    pub const LAWN_GREEN: Color = color_to_lite(fyrox_core::color::Color::LAWN_GREEN);
    pub const CHARTREUSE: Color = color_to_lite(fyrox_core::color::Color::CHARTREUSE);
    pub const GREEN_YELLOW: Color = color_to_lite(fyrox_core::color::Color::GREEN_YELLOW);
    pub const DARK_GREEN: Color = color_to_lite(fyrox_core::color::Color::DARK_GREEN);
    pub const FOREST_GREEN: Color = color_to_lite(fyrox_core::color::Color::FOREST_GREEN);
    pub const LIME: Color = color_to_lite(fyrox_core::color::Color::LIME);
    pub const LIME_GREEN: Color = color_to_lite(fyrox_core::color::Color::LIME_GREEN);
    pub const LIGHT_GREEN: Color = color_to_lite(fyrox_core::color::Color::LIGHT_GREEN);
    pub const PALE_GREEN: Color = color_to_lite(fyrox_core::color::Color::PALE_GREEN);
    pub const DARK_SEA_GREEN: Color = color_to_lite(fyrox_core::color::Color::DARK_SEA_GREEN);
    pub const MEDIUM_SPRING_GREEN: Color =
        color_to_lite(fyrox_core::color::Color::MEDIUM_SPRING_GREEN);
    pub const SPRING_GREEN: Color = color_to_lite(fyrox_core::color::Color::SPRING_GREEN);
    pub const SEA_GREEN: Color = color_to_lite(fyrox_core::color::Color::SEA_GREEN);
    pub const MEDIUM_AQUA_MARINE: Color =
        color_to_lite(fyrox_core::color::Color::MEDIUM_AQUA_MARINE);
    pub const MEDIUM_SEA_GREEN: Color = color_to_lite(fyrox_core::color::Color::MEDIUM_SEA_GREEN);
    pub const LIGHT_SEA_GREEN: Color = color_to_lite(fyrox_core::color::Color::LIGHT_SEA_GREEN);
    pub const DARK_SLATE_GRAY: Color = color_to_lite(fyrox_core::color::Color::DARK_SLATE_GRAY);
    pub const TEAL: Color = color_to_lite(fyrox_core::color::Color::TEAL);
    pub const DARK_CYAN: Color = color_to_lite(fyrox_core::color::Color::DARK_CYAN);
    pub const AQUA: Color = color_to_lite(fyrox_core::color::Color::AQUA);
    pub const CYAN: Color = color_to_lite(fyrox_core::color::Color::CYAN);
    pub const LIGHT_CYAN: Color = color_to_lite(fyrox_core::color::Color::LIGHT_CYAN);
    pub const DARK_TURQUOISE: Color = color_to_lite(fyrox_core::color::Color::DARK_TURQUOISE);
    pub const TURQUOISE: Color = color_to_lite(fyrox_core::color::Color::TURQUOISE);
    pub const MEDIUM_TURQUOISE: Color = color_to_lite(fyrox_core::color::Color::MEDIUM_TURQUOISE);
    pub const PALE_TURQUOISE: Color = color_to_lite(fyrox_core::color::Color::PALE_TURQUOISE);
    pub const AQUA_MARINE: Color = color_to_lite(fyrox_core::color::Color::AQUA_MARINE);
    pub const POWDER_BLUE: Color = color_to_lite(fyrox_core::color::Color::POWDER_BLUE);
    pub const CADET_BLUE: Color = color_to_lite(fyrox_core::color::Color::CADET_BLUE);
    pub const STEEL_BLUE: Color = color_to_lite(fyrox_core::color::Color::STEEL_BLUE);
    pub const CORN_FLOWER_BLUE: Color = color_to_lite(fyrox_core::color::Color::CORN_FLOWER_BLUE);
    pub const DEEP_SKY_BLUE: Color = color_to_lite(fyrox_core::color::Color::DEEP_SKY_BLUE);
    pub const DODGER_BLUE: Color = color_to_lite(fyrox_core::color::Color::DODGER_BLUE);
    pub const LIGHT_BLUE: Color = color_to_lite(fyrox_core::color::Color::LIGHT_BLUE);
    pub const SKY_BLUE: Color = color_to_lite(fyrox_core::color::Color::SKY_BLUE);
    pub const LIGHT_SKY_BLUE: Color = color_to_lite(fyrox_core::color::Color::LIGHT_SKY_BLUE);
    pub const MIDNIGHT_BLUE: Color = color_to_lite(fyrox_core::color::Color::MIDNIGHT_BLUE);
    pub const NAVY: Color = color_to_lite(fyrox_core::color::Color::NAVY);
    pub const DARK_BLUE: Color = color_to_lite(fyrox_core::color::Color::DARK_BLUE);
    pub const MEDIUM_BLUE: Color = color_to_lite(fyrox_core::color::Color::MEDIUM_BLUE);
    pub const ROYAL_BLUE: Color = color_to_lite(fyrox_core::color::Color::ROYAL_BLUE);
    pub const BLUE_VIOLET: Color = color_to_lite(fyrox_core::color::Color::BLUE_VIOLET);
    pub const INDIGO: Color = color_to_lite(fyrox_core::color::Color::INDIGO);
    pub const DARK_SLATE_BLUE: Color = color_to_lite(fyrox_core::color::Color::DARK_SLATE_BLUE);
    pub const SLATE_BLUE: Color = color_to_lite(fyrox_core::color::Color::SLATE_BLUE);
    pub const MEDIUM_SLATE_BLUE: Color =
        color_to_lite(fyrox_core::color::Color::MEDIUM_SLATE_BLUE);
    pub const MEDIUM_PURPLE: Color = color_to_lite(fyrox_core::color::Color::MEDIUM_PURPLE);
    pub const DARK_MAGENTA: Color = color_to_lite(fyrox_core::color::Color::DARK_MAGENTA);
    pub const DARK_VIOLET: Color = color_to_lite(fyrox_core::color::Color::DARK_VIOLET);
    pub const DARK_ORCHID: Color = color_to_lite(fyrox_core::color::Color::DARK_ORCHID);
    pub const MEDIUM_ORCHID: Color = color_to_lite(fyrox_core::color::Color::MEDIUM_ORCHID);
    pub const PURPLE: Color = color_to_lite(fyrox_core::color::Color::PURPLE);
    pub const THISTLE: Color = color_to_lite(fyrox_core::color::Color::THISTLE);
    pub const PLUM: Color = color_to_lite(fyrox_core::color::Color::PLUM);
    pub const VIOLET: Color = color_to_lite(fyrox_core::color::Color::VIOLET);
    pub const MAGENTA: Color = color_to_lite(fyrox_core::color::Color::MAGENTA);
    pub const ORCHID: Color = color_to_lite(fyrox_core::color::Color::ORCHID);
    pub const MEDIUM_VIOLET_RED: Color =
        color_to_lite(fyrox_core::color::Color::MEDIUM_VIOLET_RED);
    pub const PALE_VIOLET_RED: Color = color_to_lite(fyrox_core::color::Color::PALE_VIOLET_RED);
    pub const DEEP_PINK: Color = color_to_lite(fyrox_core::color::Color::DEEP_PINK);
    pub const HOT_PINK: Color = color_to_lite(fyrox_core::color::Color::HOT_PINK);
    pub const LIGHT_PINK: Color = color_to_lite(fyrox_core::color::Color::LIGHT_PINK);
    pub const PINK: Color = color_to_lite(fyrox_core::color::Color::PINK);
    pub const ANTIQUE_WHITE: Color = color_to_lite(fyrox_core::color::Color::ANTIQUE_WHITE);
    pub const BEIGE: Color = color_to_lite(fyrox_core::color::Color::BEIGE);
    pub const BISQUE: Color = color_to_lite(fyrox_core::color::Color::BISQUE);
    pub const BLANCHED_ALMOND: Color = color_to_lite(fyrox_core::color::Color::BLANCHED_ALMOND);
    pub const WHEAT: Color = color_to_lite(fyrox_core::color::Color::WHEAT);
    pub const CORN_SILK: Color = color_to_lite(fyrox_core::color::Color::CORN_SILK);
    pub const LEMON_CHIFFON: Color = color_to_lite(fyrox_core::color::Color::LEMON_CHIFFON);
    pub const LIGHT_GOLDEN_ROD_YELLOW: Color =
        color_to_lite(fyrox_core::color::Color::LIGHT_GOLDEN_ROD_YELLOW);
    pub const LIGHT_YELLOW: Color = color_to_lite(fyrox_core::color::Color::LIGHT_YELLOW);
    pub const SADDLE_BROWN: Color = color_to_lite(fyrox_core::color::Color::SADDLE_BROWN);
    pub const SIENNA: Color = color_to_lite(fyrox_core::color::Color::SIENNA);
    pub const CHOCOLATE: Color = color_to_lite(fyrox_core::color::Color::CHOCOLATE);
    pub const PERU: Color = color_to_lite(fyrox_core::color::Color::PERU);
    pub const SANDY_BROWN: Color = color_to_lite(fyrox_core::color::Color::SANDY_BROWN);
    pub const BURLY_WOOD: Color = color_to_lite(fyrox_core::color::Color::BURLY_WOOD);
    pub const TAN: Color = color_to_lite(fyrox_core::color::Color::TAN);
    pub const ROSY_BROWN: Color = color_to_lite(fyrox_core::color::Color::ROSY_BROWN);
    pub const MOCCASIN: Color = color_to_lite(fyrox_core::color::Color::MOCCASIN);
    pub const NAVAJO_WHITE: Color = color_to_lite(fyrox_core::color::Color::NAVAJO_WHITE);
    pub const PEACH_PUFF: Color = color_to_lite(fyrox_core::color::Color::PEACH_PUFF);
    pub const MISTY_ROSE: Color = color_to_lite(fyrox_core::color::Color::MISTY_ROSE);
    pub const LAVENDER_BLUSH: Color = color_to_lite(fyrox_core::color::Color::LAVENDER_BLUSH);
    pub const LINEN: Color = color_to_lite(fyrox_core::color::Color::LINEN);
    pub const OLD_LACE: Color = color_to_lite(fyrox_core::color::Color::OLD_LACE);
    pub const PAPAYA_WHIP: Color = color_to_lite(fyrox_core::color::Color::PAPAYA_WHIP);
    pub const SEA_SHELL: Color = color_to_lite(fyrox_core::color::Color::SEA_SHELL);
    pub const MINT_CREAM: Color = color_to_lite(fyrox_core::color::Color::MINT_CREAM);
    pub const SLATE_GRAY: Color = color_to_lite(fyrox_core::color::Color::SLATE_GRAY);
    pub const LIGHT_SLATE_GRAY: Color = color_to_lite(fyrox_core::color::Color::LIGHT_SLATE_GRAY);
    pub const LIGHT_STEEL_BLUE: Color = color_to_lite(fyrox_core::color::Color::LIGHT_STEEL_BLUE);
    pub const LAVENDER: Color = color_to_lite(fyrox_core::color::Color::LAVENDER);
    pub const FLORAL_WHITE: Color = color_to_lite(fyrox_core::color::Color::FLORAL_WHITE);
    pub const ALICE_BLUE: Color = color_to_lite(fyrox_core::color::Color::ALICE_BLUE);
    pub const GHOST_WHITE: Color = color_to_lite(fyrox_core::color::Color::GHOST_WHITE);
    pub const HONEYDEW: Color = color_to_lite(fyrox_core::color::Color::HONEYDEW);
    pub const IVORY: Color = color_to_lite(fyrox_core::color::Color::IVORY);
    pub const AZURE: Color = color_to_lite(fyrox_core::color::Color::AZURE);
    pub const SNOW: Color = color_to_lite(fyrox_core::color::Color::SNOW);
    pub const DIM_GRAY: Color = color_to_lite(fyrox_core::color::Color::DIM_GRAY);
    pub const GRAY: Color = color_to_lite(fyrox_core::color::Color::GRAY);
    pub const DARK_GRAY: Color = color_to_lite(fyrox_core::color::Color::DARK_GRAY);
    pub const SILVER: Color = color_to_lite(fyrox_core::color::Color::SILVER);
    pub const LIGHT_GRAY: Color = color_to_lite(fyrox_core::color::Color::LIGHT_GRAY);
    pub const GAINSBORO: Color = color_to_lite(fyrox_core::color::Color::GAINSBORO);
    pub const WHITE_SMOKE: Color = color_to_lite(fyrox_core::color::Color::WHITE_SMOKE);
}

const fn color_to_lite(value: fyrox_core::color::Color) -> Color {
    Color {
        r: value.r,
        g: value.g,
        b: value.b,
        a: value.a,
    }
}

impl From<Color> for fyrox_lite::lite_ui::Color {
    fn from(Color { r, g, b, a }: Color) -> Self {
        Self { r, g, b, a }
    }
}

impl From<fyrox_lite::lite_ui::Color> for Color {
    fn from(fyrox_lite::lite_ui::Color { r, g, b, a }: fyrox_lite::lite_ui::Color) -> Self {
        Self { r, g, b, a }
    }
}
