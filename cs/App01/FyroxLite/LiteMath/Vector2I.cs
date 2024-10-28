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
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
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
internal struct Vector2i_optional {
    internal Vector2i Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector2i? ToFacade(in Vector2i_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector2i_optional FromFacade(in Vector2i? value) => new Vector2i_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct Vector2i_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal Vector2i value;
    [FieldOffset(sizeof(int))]
    internal string err;
}