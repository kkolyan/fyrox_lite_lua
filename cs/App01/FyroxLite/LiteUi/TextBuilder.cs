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
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
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
internal struct TextBuilder_optional {
    internal TextBuilder Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static TextBuilder? ToFacade(in TextBuilder_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static TextBuilder_optional FromFacade(in TextBuilder? value) => new TextBuilder_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct TextBuilder_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal TextBuilder value;
    [FieldOffset(sizeof(int))]
    internal string err;
}