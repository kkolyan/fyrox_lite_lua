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

// fyrox_lite::lite_ui::LinearGradient
[StructLayout(LayoutKind.Sequential)]
public struct LinearGradient
{
    public Vector2 From {
        get => _from;
        set => _from = value;
    }
    public Vector2 To {
        get => _to;
        set => _to = value;
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
    private Vector2 _from;
    private Vector2 _to;
    private GradientPoint_slice _stops;
}

[StructLayout(LayoutKind.Sequential)]
internal struct LinearGradient_optional
{
    internal LinearGradient Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static LinearGradient? ToFacade(in LinearGradient_optional value)
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
    public static LinearGradient_optional FromFacade(in LinearGradient? value)
    {
        if (value == null)
        {
            return new LinearGradient_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new LinearGradient_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct LinearGradient_slice
{
    private unsafe LinearGradient* begin;
    private int length;
    internal List<LinearGradient>? Fetched;

    internal unsafe LinearGradient_slice(LinearGradient* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref LinearGradient_slice self)
    {
        var fetched = new List<LinearGradient>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<LinearGradient> ToFacade(in LinearGradient_slice self)
    {
        var fetched = new List<LinearGradient>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static LinearGradient_slice FromFacade(in List<LinearGradient> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct LinearGradient_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal LinearGradient Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe LinearGradient ToFacade(in LinearGradient_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static LinearGradient_result FromFacade(in LinearGradient self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new LinearGradient_result {Ok = 1, Value = __item_from_facade};
    }
}