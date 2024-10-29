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

// fyrox_lite::lite_physics::LiteFeatureId
[StructLayout(LayoutKind.Sequential)]
public struct FeatureId
{
    public FeatureKind Kind {
        get => _kind;
        set => _kind = value;
    }
    public int Id {
        get => _id;
        set => _id = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private FeatureKind _kind;
    private int _id;
}

[StructLayout(LayoutKind.Sequential)]
internal struct FeatureId_optional {
    internal FeatureId Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static FeatureId? ToFacade(in FeatureId_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static FeatureId_optional FromFacade(in FeatureId? value) => new FeatureId_optional { Value = value ?? default, HasValue = value.HasValue };
}

[StructLayout(LayoutKind.Explicit)]
internal struct FeatureId_result {
    [FieldOffset(0)]
    internal int ok;

    [FieldOffset(sizeof(int))]
    internal FeatureId value;

    [FieldOffset(sizeof(int))]
    internal string err;
}

// it iterates over the unmanaged memory (Vec allocated by Rust and stored for the length of a frame in the arena).
// if user attempts to iterate this iterator after backing data is disposed,
// the methods throws exception (hash is used to check if the backing data is still alive to make it
// possible to throw exceptions instead of SIGSEGV-ing)
[StructLayout(LayoutKind.Sequential)]
public struct FeatureIdIterator : IEnumerator<FeatureId> {
    // hash is a random number,  allocated in unmanaged memory next to the items with the same lifetime.
    // arena (Vec<(Hash,Vec<FeatureId>)>) is zeroed at the end of every frame.
    private unsafe int* hash;
    private unsafe FeatureId* items;
    private int length;
    private int position;
    private int expectedHash;

    public FeatureId Current
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