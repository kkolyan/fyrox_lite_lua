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
internal struct Intersection_optional
{
    internal Intersection Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Intersection? ToFacade(in Intersection_optional value)
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
    public static Intersection_optional FromFacade(in Intersection? value)
    {
        if (value == null)
        {
            return new Intersection_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Intersection_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Intersection_slice
{
    private unsafe Intersection* begin;
    private int length;
    internal List<Intersection>? Fetched;

    internal unsafe Intersection_slice(Intersection* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref Intersection_slice self)
    {
        var fetched = new List<Intersection>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<Intersection> ToFacade(in Intersection_slice self)
    {
        var fetched = new List<Intersection>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static Intersection_slice FromFacade(in List<Intersection> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct Intersection_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal Intersection Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe Intersection ToFacade(in Intersection_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static Intersection_result FromFacade(in Intersection self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Intersection_result {Ok = 1, Value = __item_from_facade};
    }
}