using FyroxLite.LiteInput;
using FyroxLite.LiteLog;
using FyroxLite.LiteMath;
using FyroxLite.LiteNode;
using FyroxLite.LitePhysics;
using FyroxLite.LitePlugin;
using FyroxLite.LitePrefab;
using FyroxLite.LiteScene;
using FyroxLite.LiteUi;
using FyroxLite.LiteWindow;
using FyroxLite.LiteBase;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LiteUi;

// fyrox_lite::lite_ui::Color
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Color
{
    //public const Color WHITE = color_to_lite (fyrox :: core :: color :: Color :: WHITE);
    //public const Color BLACK = color_to_lite (fyrox :: core :: color :: Color :: BLACK);
    //public const Color RED = color_to_lite (fyrox :: core :: color :: Color :: RED);
    //public const Color GREEN = color_to_lite (fyrox :: core :: color :: Color :: GREEN);
    //public const Color BLUE = color_to_lite (fyrox :: core :: color :: Color :: BLUE);
    //public const Color TRANSPARENT = color_to_lite (fyrox :: core :: color :: Color :: TRANSPARENT);
    //public const Color MAROON = color_to_lite (fyrox :: core :: color :: Color :: MAROON);
    //public const Color DARK_RED = color_to_lite (fyrox :: core :: color :: Color :: DARK_RED);
    //public const Color BROWN = color_to_lite (fyrox :: core :: color :: Color :: BROWN);
    //public const Color FIREBRICK = color_to_lite (fyrox :: core :: color :: Color :: FIREBRICK);
    //public const Color CRIMSON = color_to_lite (fyrox :: core :: color :: Color :: CRIMSON);
    //public const Color TOMATO = color_to_lite (fyrox :: core :: color :: Color :: TOMATO);
    //public const Color CORAL = color_to_lite (fyrox :: core :: color :: Color :: CORAL);
    //public const Color INDIAN_RED = color_to_lite (fyrox :: core :: color :: Color :: INDIAN_RED);
    //public const Color LIGHT_CORAL = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_CORAL);
    //public const Color DARK_SALMON = color_to_lite (fyrox :: core :: color :: Color :: DARK_SALMON);
    //public const Color SALMON = color_to_lite (fyrox :: core :: color :: Color :: SALMON);
    //public const Color LIGHT_SALMON = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_SALMON);
    //public const Color ORANGE_RED = color_to_lite (fyrox :: core :: color :: Color :: ORANGE_RED);
    //public const Color DARK_ORANGE = color_to_lite (fyrox :: core :: color :: Color :: DARK_ORANGE);
    //public const Color ORANGE = color_to_lite (fyrox :: core :: color :: Color :: ORANGE);
    //public const Color GOLD = color_to_lite (fyrox :: core :: color :: Color :: GOLD);
    //public const Color DARK_GOLDEN_ROD = color_to_lite (fyrox :: core :: color :: Color :: DARK_GOLDEN_ROD);
    //public const Color GOLDEN_ROD = color_to_lite (fyrox :: core :: color :: Color :: GOLDEN_ROD);
    //public const Color PALE_GOLDEN_ROD = color_to_lite (fyrox :: core :: color :: Color :: PALE_GOLDEN_ROD);
    //public const Color DARK_KHAKI = color_to_lite (fyrox :: core :: color :: Color :: DARK_KHAKI);
    //public const Color KHAKI = color_to_lite (fyrox :: core :: color :: Color :: KHAKI);
    //public const Color OLIVE = color_to_lite (fyrox :: core :: color :: Color :: OLIVE);
    //public const Color YELLOW = color_to_lite (fyrox :: core :: color :: Color :: YELLOW);
    //public const Color YELLOW_GREEN = color_to_lite (fyrox :: core :: color :: Color :: YELLOW_GREEN);
    //public const Color DARK_OLIVE_GREEN = color_to_lite (fyrox :: core :: color :: Color :: DARK_OLIVE_GREEN);
    //public const Color OLIVE_DRAB = color_to_lite (fyrox :: core :: color :: Color :: OLIVE_DRAB);
    //public const Color LAWN_GREEN = color_to_lite (fyrox :: core :: color :: Color :: LAWN_GREEN);
    //public const Color CHARTREUSE = color_to_lite (fyrox :: core :: color :: Color :: CHARTREUSE);
    //public const Color GREEN_YELLOW = color_to_lite (fyrox :: core :: color :: Color :: GREEN_YELLOW);
    //public const Color DARK_GREEN = color_to_lite (fyrox :: core :: color :: Color :: DARK_GREEN);
    //public const Color FOREST_GREEN = color_to_lite (fyrox :: core :: color :: Color :: FOREST_GREEN);
    //public const Color LIME = color_to_lite (fyrox :: core :: color :: Color :: LIME);
    //public const Color LIME_GREEN = color_to_lite (fyrox :: core :: color :: Color :: LIME_GREEN);
    //public const Color LIGHT_GREEN = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_GREEN);
    //public const Color PALE_GREEN = color_to_lite (fyrox :: core :: color :: Color :: PALE_GREEN);
    //public const Color DARK_SEA_GREEN = color_to_lite (fyrox :: core :: color :: Color :: DARK_SEA_GREEN);
    //public const Color MEDIUM_SPRING_GREEN = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_SPRING_GREEN);
    //public const Color SPRING_GREEN = color_to_lite (fyrox :: core :: color :: Color :: SPRING_GREEN);
    //public const Color SEA_GREEN = color_to_lite (fyrox :: core :: color :: Color :: SEA_GREEN);
    //public const Color MEDIUM_AQUA_MARINE = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_AQUA_MARINE);
    //public const Color MEDIUM_SEA_GREEN = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_SEA_GREEN);
    //public const Color LIGHT_SEA_GREEN = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_SEA_GREEN);
    //public const Color DARK_SLATE_GRAY = color_to_lite (fyrox :: core :: color :: Color :: DARK_SLATE_GRAY);
    //public const Color TEAL = color_to_lite (fyrox :: core :: color :: Color :: TEAL);
    //public const Color DARK_CYAN = color_to_lite (fyrox :: core :: color :: Color :: DARK_CYAN);
    //public const Color AQUA = color_to_lite (fyrox :: core :: color :: Color :: AQUA);
    //public const Color CYAN = color_to_lite (fyrox :: core :: color :: Color :: CYAN);
    //public const Color LIGHT_CYAN = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_CYAN);
    //public const Color DARK_TURQUOISE = color_to_lite (fyrox :: core :: color :: Color :: DARK_TURQUOISE);
    //public const Color TURQUOISE = color_to_lite (fyrox :: core :: color :: Color :: TURQUOISE);
    //public const Color MEDIUM_TURQUOISE = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_TURQUOISE);
    //public const Color PALE_TURQUOISE = color_to_lite (fyrox :: core :: color :: Color :: PALE_TURQUOISE);
    //public const Color AQUA_MARINE = color_to_lite (fyrox :: core :: color :: Color :: AQUA_MARINE);
    //public const Color POWDER_BLUE = color_to_lite (fyrox :: core :: color :: Color :: POWDER_BLUE);
    //public const Color CADET_BLUE = color_to_lite (fyrox :: core :: color :: Color :: CADET_BLUE);
    //public const Color STEEL_BLUE = color_to_lite (fyrox :: core :: color :: Color :: STEEL_BLUE);
    //public const Color CORN_FLOWER_BLUE = color_to_lite (fyrox :: core :: color :: Color :: CORN_FLOWER_BLUE);
    //public const Color DEEP_SKY_BLUE = color_to_lite (fyrox :: core :: color :: Color :: DEEP_SKY_BLUE);
    //public const Color DODGER_BLUE = color_to_lite (fyrox :: core :: color :: Color :: DODGER_BLUE);
    //public const Color LIGHT_BLUE = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_BLUE);
    //public const Color SKY_BLUE = color_to_lite (fyrox :: core :: color :: Color :: SKY_BLUE);
    //public const Color LIGHT_SKY_BLUE = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_SKY_BLUE);
    //public const Color MIDNIGHT_BLUE = color_to_lite (fyrox :: core :: color :: Color :: MIDNIGHT_BLUE);
    //public const Color NAVY = color_to_lite (fyrox :: core :: color :: Color :: NAVY);
    //public const Color DARK_BLUE = color_to_lite (fyrox :: core :: color :: Color :: DARK_BLUE);
    //public const Color MEDIUM_BLUE = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_BLUE);
    //public const Color ROYAL_BLUE = color_to_lite (fyrox :: core :: color :: Color :: ROYAL_BLUE);
    //public const Color BLUE_VIOLET = color_to_lite (fyrox :: core :: color :: Color :: BLUE_VIOLET);
    //public const Color INDIGO = color_to_lite (fyrox :: core :: color :: Color :: INDIGO);
    //public const Color DARK_SLATE_BLUE = color_to_lite (fyrox :: core :: color :: Color :: DARK_SLATE_BLUE);
    //public const Color SLATE_BLUE = color_to_lite (fyrox :: core :: color :: Color :: SLATE_BLUE);
    //public const Color MEDIUM_SLATE_BLUE = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_SLATE_BLUE);
    //public const Color MEDIUM_PURPLE = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_PURPLE);
    //public const Color DARK_MAGENTA = color_to_lite (fyrox :: core :: color :: Color :: DARK_MAGENTA);
    //public const Color DARK_VIOLET = color_to_lite (fyrox :: core :: color :: Color :: DARK_VIOLET);
    //public const Color DARK_ORCHID = color_to_lite (fyrox :: core :: color :: Color :: DARK_ORCHID);
    //public const Color MEDIUM_ORCHID = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_ORCHID);
    //public const Color PURPLE = color_to_lite (fyrox :: core :: color :: Color :: PURPLE);
    //public const Color THISTLE = color_to_lite (fyrox :: core :: color :: Color :: THISTLE);
    //public const Color PLUM = color_to_lite (fyrox :: core :: color :: Color :: PLUM);
    //public const Color VIOLET = color_to_lite (fyrox :: core :: color :: Color :: VIOLET);
    //public const Color MAGENTA = color_to_lite (fyrox :: core :: color :: Color :: MAGENTA);
    //public const Color ORCHID = color_to_lite (fyrox :: core :: color :: Color :: ORCHID);
    //public const Color MEDIUM_VIOLET_RED = color_to_lite (fyrox :: core :: color :: Color :: MEDIUM_VIOLET_RED);
    //public const Color PALE_VIOLET_RED = color_to_lite (fyrox :: core :: color :: Color :: PALE_VIOLET_RED);
    //public const Color DEEP_PINK = color_to_lite (fyrox :: core :: color :: Color :: DEEP_PINK);
    //public const Color HOT_PINK = color_to_lite (fyrox :: core :: color :: Color :: HOT_PINK);
    //public const Color LIGHT_PINK = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_PINK);
    //public const Color PINK = color_to_lite (fyrox :: core :: color :: Color :: PINK);
    //public const Color ANTIQUE_WHITE = color_to_lite (fyrox :: core :: color :: Color :: ANTIQUE_WHITE);
    //public const Color BEIGE = color_to_lite (fyrox :: core :: color :: Color :: BEIGE);
    //public const Color BISQUE = color_to_lite (fyrox :: core :: color :: Color :: BISQUE);
    //public const Color BLANCHED_ALMOND = color_to_lite (fyrox :: core :: color :: Color :: BLANCHED_ALMOND);
    //public const Color WHEAT = color_to_lite (fyrox :: core :: color :: Color :: WHEAT);
    //public const Color CORN_SILK = color_to_lite (fyrox :: core :: color :: Color :: CORN_SILK);
    //public const Color LEMON_CHIFFON = color_to_lite (fyrox :: core :: color :: Color :: LEMON_CHIFFON);
    //public const Color LIGHT_GOLDEN_ROD_YELLOW = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_GOLDEN_ROD_YELLOW);
    //public const Color LIGHT_YELLOW = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_YELLOW);
    //public const Color SADDLE_BROWN = color_to_lite (fyrox :: core :: color :: Color :: SADDLE_BROWN);
    //public const Color SIENNA = color_to_lite (fyrox :: core :: color :: Color :: SIENNA);
    //public const Color CHOCOLATE = color_to_lite (fyrox :: core :: color :: Color :: CHOCOLATE);
    //public const Color PERU = color_to_lite (fyrox :: core :: color :: Color :: PERU);
    //public const Color SANDY_BROWN = color_to_lite (fyrox :: core :: color :: Color :: SANDY_BROWN);
    //public const Color BURLY_WOOD = color_to_lite (fyrox :: core :: color :: Color :: BURLY_WOOD);
    //public const Color TAN = color_to_lite (fyrox :: core :: color :: Color :: TAN);
    //public const Color ROSY_BROWN = color_to_lite (fyrox :: core :: color :: Color :: ROSY_BROWN);
    //public const Color MOCCASIN = color_to_lite (fyrox :: core :: color :: Color :: MOCCASIN);
    //public const Color NAVAJO_WHITE = color_to_lite (fyrox :: core :: color :: Color :: NAVAJO_WHITE);
    //public const Color PEACH_PUFF = color_to_lite (fyrox :: core :: color :: Color :: PEACH_PUFF);
    //public const Color MISTY_ROSE = color_to_lite (fyrox :: core :: color :: Color :: MISTY_ROSE);
    //public const Color LAVENDER_BLUSH = color_to_lite (fyrox :: core :: color :: Color :: LAVENDER_BLUSH);
    //public const Color LINEN = color_to_lite (fyrox :: core :: color :: Color :: LINEN);
    //public const Color OLD_LACE = color_to_lite (fyrox :: core :: color :: Color :: OLD_LACE);
    //public const Color PAPAYA_WHIP = color_to_lite (fyrox :: core :: color :: Color :: PAPAYA_WHIP);
    //public const Color SEA_SHELL = color_to_lite (fyrox :: core :: color :: Color :: SEA_SHELL);
    //public const Color MINT_CREAM = color_to_lite (fyrox :: core :: color :: Color :: MINT_CREAM);
    //public const Color SLATE_GRAY = color_to_lite (fyrox :: core :: color :: Color :: SLATE_GRAY);
    //public const Color LIGHT_SLATE_GRAY = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_SLATE_GRAY);
    //public const Color LIGHT_STEEL_BLUE = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_STEEL_BLUE);
    //public const Color LAVENDER = color_to_lite (fyrox :: core :: color :: Color :: LAVENDER);
    //public const Color FLORAL_WHITE = color_to_lite (fyrox :: core :: color :: Color :: FLORAL_WHITE);
    //public const Color ALICE_BLUE = color_to_lite (fyrox :: core :: color :: Color :: ALICE_BLUE);
    //public const Color GHOST_WHITE = color_to_lite (fyrox :: core :: color :: Color :: GHOST_WHITE);
    //public const Color HONEYDEW = color_to_lite (fyrox :: core :: color :: Color :: HONEYDEW);
    //public const Color IVORY = color_to_lite (fyrox :: core :: color :: Color :: IVORY);
    //public const Color AZURE = color_to_lite (fyrox :: core :: color :: Color :: AZURE);
    //public const Color SNOW = color_to_lite (fyrox :: core :: color :: Color :: SNOW);
    //public const Color DIM_GRAY = color_to_lite (fyrox :: core :: color :: Color :: DIM_GRAY);
    //public const Color GRAY = color_to_lite (fyrox :: core :: color :: Color :: GRAY);
    //public const Color DARK_GRAY = color_to_lite (fyrox :: core :: color :: Color :: DARK_GRAY);
    //public const Color SILVER = color_to_lite (fyrox :: core :: color :: Color :: SILVER);
    //public const Color LIGHT_GRAY = color_to_lite (fyrox :: core :: color :: Color :: LIGHT_GRAY);
    //public const Color GAINSBORO = color_to_lite (fyrox :: core :: color :: Color :: GAINSBORO);
    //public const Color WHITE_SMOKE = color_to_lite (fyrox :: core :: color :: Color :: WHITE_SMOKE);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Color_optional
{
    private Color value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Color? ToFacade(in Color_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Color_optional FromFacade(in Color? value)
    {
        if (value == null)
        {
            return new Color_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Color_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}