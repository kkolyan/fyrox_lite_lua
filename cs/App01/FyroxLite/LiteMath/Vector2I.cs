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

// fyrox_lite::lite_math::PodVector2i
[StructLayout(LayoutKind.Sequential)]
public struct Vector2i
{
    public long X {
        get => _x;
        set => _x = value;
    }
    public long Y {
        get => _y;
        set => _y = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private long _x;
    private long _y;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Vector2i_optional
{
    internal Vector2i Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector2i? ToFacade(in Vector2i_optional value)
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
    public static Vector2i_optional FromFacade(in Vector2i? value)
    {
        if (value == null)
        {
            return new Vector2i_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Vector2i_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Vector2i_slice
{
    private unsafe Vector2i* begin;
    private int length;
    internal List<Vector2i>? Fetched;

    internal unsafe Vector2i_slice(Vector2i* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref Vector2i_slice self)
    {
        var fetched = new List<Vector2i>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<Vector2i> ToFacade(in Vector2i_slice self)
    {
        var fetched = new List<Vector2i>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static Vector2i_slice FromFacade(in List<Vector2i> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct Vector2i_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal Vector2i Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe Vector2i ToFacade(in Vector2i_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static Vector2i_result FromFacade(in Vector2i self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Vector2i_result {Ok = 1, Value = __item_from_facade};
    }
}