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

// fyrox_lite::lite_ui::LiteUiNode
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct UiNode
{
}

[StructLayout(LayoutKind.Sequential)]
internal struct UiNode_optional
{
    private UiNode value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static UiNode? ToFacade(in UiNode_optional value)
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
    public static UiNode_optional FromFacade(in UiNode? value)
    {
        if (value == null)
        {
            return new UiNode_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new UiNode_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}