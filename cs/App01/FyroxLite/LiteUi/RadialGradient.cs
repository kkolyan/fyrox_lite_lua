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

// fyrox_lite::lite_ui::RadialGradient
[StructLayout(LayoutKind.Sequential)]
public partial struct RadialGradient
{
    public Vector2 Center {
        get => NativeVector2.ToFacade(_center);
        set => _center = NativeVector2.FromFacade(value);
    }
    public List<GradientPoint> Stops {
        get => GradientPoint_slice.ToFacade(_stops);
        set => _stops = GradientPoint_slice.FromFacade(value);
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private NativeVector2 _center;
    private GradientPoint_slice _stops;
}

[StructLayout(LayoutKind.Sequential)]
internal struct RadialGradient_optional
{
    private RadialGradient value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RadialGradient? ToFacade(in RadialGradient_optional value)
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
    public static RadialGradient_optional FromFacade(in RadialGradient? value)
    {
        if (value == null)
        {
            return new RadialGradient_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new RadialGradient_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct RadialGradient_slice
{
    internal unsafe RadialGradient* begin;
    internal int length;

    internal unsafe RadialGradient_slice(RadialGradient* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<RadialGradient> ToFacade(in RadialGradient_slice self)
    {
        var fetched = new List<RadialGradient>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static RadialGradient_slice FromFacade(in List<RadialGradient> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct RadialGradient_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private RadialGradient value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe RadialGradient ToFacade(in RadialGradient_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static RadialGradient_result FromFacade(in RadialGradient self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new RadialGradient_result {ok = 1, value = __item_from_facade};
    }
}