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
using System.Numerics;
using FyroxLite.LiteBase;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LiteUi;

// fyrox_lite::lite_ui::Brush
[StructLayout(LayoutKind.Sequential)]
public partial struct Brush
{
    public Color? SolidColor {
        get => Color_optional.ToFacade(_solid_color);
        set => _solid_color = Color_optional.FromFacade(value);
    }
    public LinearGradient? LinearGradient {
        get => LinearGradient_optional.ToFacade(_linear_gradient);
        set => _linear_gradient = LinearGradient_optional.FromFacade(value);
    }
    public RadialGradient? RadialGradient {
        get => RadialGradient_optional.ToFacade(_radial_gradient);
        set => _radial_gradient = RadialGradient_optional.FromFacade(value);
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private Color_optional _solid_color;
    private LinearGradient_optional _linear_gradient;
    private RadialGradient_optional _radial_gradient;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Brush_optional
{
    private Brush value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Brush? ToFacade(in Brush_optional value)
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
    public static Brush_optional FromFacade(in Brush? value)
    {
        if (value == null)
        {
            return new Brush_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new Brush_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Brush_slice
{
    internal unsafe Brush* begin;
    internal int length;

    internal unsafe Brush_slice(Brush* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Brush> ToFacade(in Brush_slice self)
    {
        var fetched = new List<Brush>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static Brush_slice FromFacade(in List<Brush> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct Brush_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private Brush value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe Brush ToFacade(in Brush_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static Brush_result FromFacade(in Brush self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Brush_result {ok = 1, value = __item_from_facade};
    }
}