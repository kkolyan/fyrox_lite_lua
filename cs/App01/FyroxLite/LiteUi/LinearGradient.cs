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

// fyrox_lite::lite_ui::LinearGradient
[StructLayout(LayoutKind.Sequential)]
public partial struct LinearGradient
{
    public Vector2 From {
        get => NativeVector2.ToFacade(_from);
        set => _from = NativeVector2.FromFacade(value);
    }
    public Vector2 To {
        get => NativeVector2.ToFacade(_to);
        set => _to = NativeVector2.FromFacade(value);
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
    private NativeVector2 _from;
    private NativeVector2 _to;
    private GradientPoint_slice _stops;
}

[StructLayout(LayoutKind.Sequential)]
internal struct LinearGradient_optional
{
    private LinearGradient value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static LinearGradient? ToFacade(in LinearGradient_optional value)
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
    public static LinearGradient_optional FromFacade(in LinearGradient? value)
    {
        if (value == null)
        {
            return new LinearGradient_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new LinearGradient_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct LinearGradient_slice
{
    internal unsafe LinearGradient* begin;
    internal int length;

    internal unsafe LinearGradient_slice(LinearGradient* begin, int length)
    {
        this.begin = begin;
        this.length = length;
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
    private int ok;

    [FieldOffset(sizeof(int))]
    private LinearGradient value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe LinearGradient ToFacade(in LinearGradient_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static LinearGradient_result FromFacade(in LinearGradient self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new LinearGradient_result {ok = 1, value = __item_from_facade};
    }
}