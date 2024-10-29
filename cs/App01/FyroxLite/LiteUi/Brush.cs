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
namespace FyroxLite.LiteUi;

// fyrox_lite::lite_ui::Brush
[StructLayout(LayoutKind.Sequential)]
public struct Brush
{
    public Color? SolidColor {
        get => Color_optional.ToFacade(_solid_color);
        set => _solid_color = Color_optional.FromFacade(value);
    }
    public LinearGradient? LinearGradient {
        get => LinearGradient_optional.ToFacade(_linear_gradient);
        set => _linear_gradient = LinearGradient_optional.FromFacade(value);
    }
    public RadialGradient? RadialGradient {
        get => RadialGradient_optional.ToFacade(_radial_gradient);
        set => _radial_gradient = RadialGradient_optional.FromFacade(value);
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private Color_optional _solid_color;
    private LinearGradient_optional _linear_gradient;
    private RadialGradient_optional _radial_gradient;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Brush_optional {
    internal Brush Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Brush? ToFacade(in Brush_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Brush_optional FromFacade(in Brush? value) => new Brush_optional { Value = value ?? default, HasValue = value.HasValue };
}

[StructLayout(LayoutKind.Explicit)]
internal struct Brush_result {
    [FieldOffset(0)]
    internal int ok;

    [FieldOffset(sizeof(int))]
    internal Brush value;

    [FieldOffset(sizeof(int))]
    internal string err;
}

// it iterates over the unmanaged memory (Vec allocated by Rust and stored for the length of a frame in the arena).
// if user attempts to iterate this iterator after backing data is disposed,
// the methods throws exception (hash is used to check if the backing data is still alive to make it
// possible to throw exceptions instead of SIGSEGV-ing)
[StructLayout(LayoutKind.Sequential)]
public struct BrushIterator : IEnumerator<Brush> {
    // hash is a random number,  allocated in unmanaged memory next to the items with the same lifetime.
    // arena (Vec<(Hash,Vec<Brush>)>) is zeroed at the end of every frame.
    private unsafe int* hash;
    private unsafe Brush* items;
    private int length;
    private int position;
    private int expectedHash;

    public Brush Current
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