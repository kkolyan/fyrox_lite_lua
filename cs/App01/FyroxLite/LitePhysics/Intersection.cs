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

// it iterates over the unmanaged memory (Vec allocated by Rust and stored for the length of a frame in the arena).
// if user attempts to iterate this iterator after backing data is disposed,
// the methods throws exception (hash is used to check if the backing data is still alive to make it
// possible to throw exceptions instead of SIGSEGV-ing)
[StructLayout(LayoutKind.Sequential)]
public struct IntersectionIterator : IEnumerator<Intersection> {
    // hash is a random number,  allocated in unmanaged memory next to the items with the same lifetime.
    // arena (Vec<(Hash,Vec<Intersection>)>) is zeroed at the end of every frame.
    private unsafe int* hash;
    private unsafe Intersection* items;
    private int length;
    private int position;
    private int expectedHash;

    public Intersection Current
    {
        get
        {
            unsafe {
              if (*hash != expectedHash) {
                 throw new Exception("iterator is not valid anymore (it's valid only for one frame)");
              }
              return *(items + position);
            }
        }
    }

    public bool MoveNext() {
        if (position < length - 2) {
            position ++;
            return true;
        }
        return false;
    }

    public void Dispose()
    {
    }

    public void Reset() => position = 0;

    object? IEnumerator.Current => Current;
}