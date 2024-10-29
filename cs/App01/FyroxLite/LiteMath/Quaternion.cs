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

// it iterates over the unmanaged memory (Vec allocated by Rust and stored for the length of a frame in the arena).
// if user attempts to iterate this iterator after backing data is disposed,
// the methods throws exception (hash is used to check if the backing data is still alive to make it
// possible to throw exceptions instead of SIGSEGV-ing)
[StructLayout(LayoutKind.Sequential)]
public struct QuaternionIterator : IEnumerator<Quaternion> {
    // hash is a random number,  allocated in unmanaged memory next to the items with the same lifetime.
    // arena (Vec<(Hash,Vec<Quaternion>)>) is zeroed at the end of every frame.
    private unsafe int* hash;
    private unsafe Quaternion* items;
    private int length;
    private int position;
    private int expectedHash;

    public Quaternion Current
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