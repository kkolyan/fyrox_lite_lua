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
internal struct Vector3_optional {
    internal Vector3 Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector3? ToFacade(in Vector3_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector3_optional FromFacade(in Vector3? value) => new Vector3_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct Vector3_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal Vector3 value;
    [FieldOffset(sizeof(int))]
    internal string err;
}