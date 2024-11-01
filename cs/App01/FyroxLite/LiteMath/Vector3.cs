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

// fyrox_lite::lite_math::PodVector3
[StructLayout(LayoutKind.Sequential)]
public struct Vector3
{
    public float X {
        get => _x;
        set => _x = value;
    }
    public float Y {
        get => _y;
        set => _y = value;
    }
    public float Z {
        get => _z;
        set => _z = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private float _x;
    private float _y;
    private float _z;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Vector3_optional
{
    private Vector3 value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector3? ToFacade(in Vector3_optional value)
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
    public static Vector3_optional FromFacade(in Vector3? value)
    {
        if (value == null)
        {
            return new Vector3_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Vector3_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Vector3_slice
{
    private unsafe Vector3* begin;
    private int length;
    internal List<Vector3>? Fetched;

    internal unsafe Vector3_slice(Vector3* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref Vector3_slice self)
    {
        var fetched = new List<Vector3>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<Vector3> ToFacade(in Vector3_slice self)
    {
        var fetched = new List<Vector3>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static Vector3_slice FromFacade(in List<Vector3> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct Vector3_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private Vector3 value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe Vector3 ToFacade(in Vector3_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static Vector3_result FromFacade(in Vector3 self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Vector3_result {ok = 1, value = __item_from_facade};
    }
}