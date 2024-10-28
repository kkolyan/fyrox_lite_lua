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
namespace FyroxLite.LiteUi;

// fyrox_lite::lite_ui::GradientPoint
[StructLayout(LayoutKind.Sequential)]
public struct GradientPoint
{
    public float Stop {
        get => _stop;
        set => _stop = value;
    }
    public Color Color {
        get => _color;
        set => _color = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private float _stop;
    private Color _color;
}
[StructLayout(LayoutKind.Sequential)]
internal struct GradientPoint_optional {
    internal GradientPoint Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static GradientPoint? ToFacade(in GradientPoint_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static GradientPoint_optional FromFacade(in GradientPoint? value) => new GradientPoint_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct GradientPoint_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal GradientPoint value;
    [FieldOffset(sizeof(int))]
    internal string err;
}