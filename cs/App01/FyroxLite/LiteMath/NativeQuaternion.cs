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
namespace FyroxLite.LiteMath;

// fyrox_lite::lite_math::PodQuaternion
[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeQuaternion
{
    private float _i;
    private float _j;
    private float _k;
    private float _w;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeQuaternion_optional
{
    private NativeQuaternion value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Quaternion? ToFacade(in NativeQuaternion_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeQuaternion.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeQuaternion_optional FromFacade(in Quaternion? value)
    {
        if (value == null)
        {
            return new NativeQuaternion_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = NativeQuaternion.FromFacade(__item);
        return new NativeQuaternion_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeQuaternion_slice
{
    internal unsafe NativeQuaternion* begin;
    internal int length;

    internal unsafe NativeQuaternion_slice(NativeQuaternion* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Quaternion> ToFacade(in NativeQuaternion_slice self)
    {
        var fetched = new List<Quaternion>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeQuaternion.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static NativeQuaternion_slice FromFacade(in List<Quaternion> self)
    {
        // NativeQuaternion.FromFacade(__item)
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeQuaternion_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeQuaternion value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe Quaternion ToFacade(in NativeQuaternion_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeQuaternion.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static NativeQuaternion_result FromFacade(in Quaternion self)
    {
        var __item = self;
        var __item_from_facade = NativeQuaternion.FromFacade(__item);
        return new NativeQuaternion_result {ok = 1, value = __item_from_facade};
    }
}