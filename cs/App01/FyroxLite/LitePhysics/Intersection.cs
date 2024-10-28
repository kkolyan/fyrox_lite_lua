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
namespace FyroxLite.LitePhysics;

// fyrox_lite::lite_physics::LiteIntersection
[StructLayout(LayoutKind.Sequential)]
public struct Intersection
{
    public Node Collider {
        get => _collider;
        set => _collider = value;
    }
    public Vector3 Normal {
        get => _normal;
        set => _normal = value;
    }
    public Vector3 Position {
        get => _position;
        set => _position = value;
    }
    public FeatureId Feature {
        get => _feature;
        set => _feature = value;
    }
    public float Toi {
        get => _toi;
        set => _toi = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private Node _collider;
    private Vector3 _normal;
    private Vector3 _position;
    private FeatureId _feature;
    private float _toi;
}
[StructLayout(LayoutKind.Sequential)]
internal struct Intersection_optional {
    internal Intersection Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Intersection? ToFacade(in Intersection_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Intersection_optional FromFacade(in Intersection? value) => new Intersection_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct Intersection_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal Intersection value;
    [FieldOffset(sizeof(int))]
    internal string err;
}