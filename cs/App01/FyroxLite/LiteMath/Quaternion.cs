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
namespace FyroxLite.LiteMath;

// fyrox_lite::lite_math::PodQuaternion
[StructLayout(LayoutKind.Sequential)]
public struct Quaternion
{
    public float I {
        get => _i;
        set => _i = value;
    }
    public float J {
        get => _j;
        set => _j = value;
    }
    public float K {
        get => _k;
        set => _k = value;
    }
    public float W {
        get => _w;
        set => _w = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private float _i;
    private float _j;
    private float _k;
    private float _w;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Quaternion_optional
{
    private Quaternion value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Quaternion? ToFacade(in Quaternion_optional value)
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
    public static Quaternion_optional FromFacade(in Quaternion? value)
    {
        if (value == null)
        {
            return new Quaternion_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Quaternion_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Quaternion_slice
{
    private unsafe Quaternion* begin;
    private int length;
    internal List<Quaternion>? Fetched;

    internal unsafe Quaternion_slice(Quaternion* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref Quaternion_slice self)
    {
        var fetched = new List<Quaternion>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<Quaternion> ToFacade(in Quaternion_slice self)
    {
        var fetched = new List<Quaternion>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static Quaternion_slice FromFacade(in List<Quaternion> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct Quaternion_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private Quaternion value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe Quaternion ToFacade(in Quaternion_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static Quaternion_result FromFacade(in Quaternion self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Quaternion_result {ok = 1, value = __item_from_facade};
    }
}