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

// fyrox_lite::lite_math::PodVector2
[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeVector2
{
    private float _x;
    private float _y;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeVector2_optional
{
    private NativeVector2 value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector2? ToFacade(in NativeVector2_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeVector2.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeVector2_optional FromFacade(in Vector2? value)
    {
        if (value == null)
        {
            return new NativeVector2_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = NativeVector2.FromFacade(__item);
        return new NativeVector2_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeVector2_slice
{
    internal unsafe NativeVector2* begin;
    internal int length;

    internal unsafe NativeVector2_slice(NativeVector2* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Vector2> ToFacade(in NativeVector2_slice self)
    {
        var fetched = new List<Vector2>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeVector2.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static NativeVector2_slice FromFacade(in List<Vector2> self)
    {
        // NativeVector2.FromFacade(__item)
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeVector2_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeVector2 value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe Vector2 ToFacade(in NativeVector2_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeVector2.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static NativeVector2_result FromFacade(in Vector2 self)
    {
        var __item = self;
        var __item_from_facade = NativeVector2.FromFacade(__item);
        return new NativeVector2_result {ok = 1, value = __item_from_facade};
    }
}