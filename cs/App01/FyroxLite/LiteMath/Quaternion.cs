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
internal struct Quaternion_optional {
    internal Quaternion Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Quaternion? ToFacade(in Quaternion_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Quaternion_optional FromFacade(in Quaternion? value) => new Quaternion_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct Quaternion_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal Quaternion value;
    [FieldOffset(sizeof(int))]
    internal string err;
}