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

// fyrox_lite::lite_math::PodVector2
[StructLayout(LayoutKind.Sequential)]
public struct Vector2
{
    public float X {
        get => _x;
        set => _x = value;
    }
    public float Y {
        get => _y;
        set => _y = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private float _x;
    private float _y;
}
[StructLayout(LayoutKind.Sequential)]
internal struct Vector2_optional {
    internal Vector2 Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector2? ToFacade(in Vector2_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector2_optional FromFacade(in Vector2? value) => new Vector2_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct Vector2_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal Vector2 value;
    [FieldOffset(sizeof(int))]
    internal string err;
}