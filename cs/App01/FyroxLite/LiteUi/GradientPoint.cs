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

// fyrox_lite::lite_ui::GradientPoint
[StructLayout(LayoutKind.Sequential)]
public struct GradientPoint
{
    public float Stop {
        get => _stop;
        set => _stop = value;
    }
    public Color Color {
        get => _color;
        set => _color = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private float _stop;
    private Color _color;
}

[StructLayout(LayoutKind.Sequential)]
internal struct GradientPoint_optional
{
    private GradientPoint value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static GradientPoint? ToFacade(in GradientPoint_optional value)
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
    public static GradientPoint_optional FromFacade(in GradientPoint? value)
    {
        if (value == null)
        {
            return new GradientPoint_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new GradientPoint_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct GradientPoint_slice
{
    private unsafe GradientPoint* begin;
    private int length;
    internal List<GradientPoint>? Fetched;

    internal unsafe GradientPoint_slice(GradientPoint* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref GradientPoint_slice self)
    {
        var fetched = new List<GradientPoint>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<GradientPoint> ToFacade(in GradientPoint_slice self)
    {
        var fetched = new List<GradientPoint>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static GradientPoint_slice FromFacade(in List<GradientPoint> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct GradientPoint_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private GradientPoint value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe GradientPoint ToFacade(in GradientPoint_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static GradientPoint_result FromFacade(in GradientPoint self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new GradientPoint_result {ok = 1, value = __item_from_facade};
    }
}