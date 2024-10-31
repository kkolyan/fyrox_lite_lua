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

// fyrox_lite::lite_ui::TextBuilder
[StructLayout(LayoutKind.Sequential)]
public struct TextBuilder
{
    public Brush? Foreground {
        get => Brush_optional.ToFacade(_foreground);
        set => _foreground = Brush_optional.FromFacade(value);
    }
    public float? FontSize {
        get => float_optional.ToFacade(_font_size);
        set => _font_size = float_optional.FromFacade(value);
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private Brush_optional _foreground;
    private float_optional _font_size;
}

[StructLayout(LayoutKind.Sequential)]
internal struct TextBuilder_optional
{
    internal TextBuilder Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static TextBuilder? ToFacade(in TextBuilder_optional value)
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
    public static TextBuilder_optional FromFacade(in TextBuilder? value)
    {
        if (value == null)
        {
            return new TextBuilder_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new TextBuilder_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct TextBuilder_slice
{
    private unsafe TextBuilder* begin;
    private int length;
    internal List<TextBuilder>? Fetched;

    internal unsafe TextBuilder_slice(TextBuilder* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref TextBuilder_slice self)
    {
        var fetched = new List<TextBuilder>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<TextBuilder> ToFacade(in TextBuilder_slice self)
    {
        var fetched = new List<TextBuilder>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static TextBuilder_slice FromFacade(in List<TextBuilder> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct TextBuilder_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal TextBuilder Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe TextBuilder ToFacade(in TextBuilder_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static TextBuilder_result FromFacade(in TextBuilder self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new TextBuilder_result {Ok = 1, Value = __item_from_facade};
    }
}