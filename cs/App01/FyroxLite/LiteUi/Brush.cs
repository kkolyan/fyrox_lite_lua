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

// fyrox_lite::lite_ui::Brush
[StructLayout(LayoutKind.Sequential)]
public struct Brush
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
    internal Brush Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Brush? ToFacade(in Brush_optional value)
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
    public static Brush_optional FromFacade(in Brush? value)
    {
        if (value == null)
        {
            return new Brush_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Brush_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Brush_slice
{
    private unsafe Brush* begin;
    private int length;
    internal List<Brush>? Fetched;

    internal unsafe Brush_slice(Brush* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref Brush_slice self)
    {
        var fetched = new List<Brush>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
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
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal Brush Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe Brush ToFacade(in Brush_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static Brush_result FromFacade(in Brush self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Brush_result {Ok = 1, Value = __item_from_facade};
    }
}