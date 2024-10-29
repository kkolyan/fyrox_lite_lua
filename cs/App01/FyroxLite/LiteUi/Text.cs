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

// fyrox_lite::lite_ui::LiteText
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Text
{

    public void SetTextAsync(string text)
    {
        unsafe {
            var _text = text;
            fyrox_lite_lite_ui_LiteText_SetTextAsync(this, _text);
        }
    }

    public static Text New(TextBuilder state)
    {
        unsafe {
            var _state = state;
            var __ret = fyrox_lite_lite_ui_LiteText_New(_state);
            return __ret;
        }
    }
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_ui_LiteText_SetTextAsync(Text self, string text);
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Text fyrox_lite_lite_ui_LiteText_New(TextBuilder state);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Text_optional
{
    internal Text Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Text? ToFacade(in Text_optional value)
    {
        if (value.HasValue)
        {
            var __item = value.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Text_optional FromFacade(in Text? value)
    {
        if (value == null)
        {
            return new Text_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Text_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}