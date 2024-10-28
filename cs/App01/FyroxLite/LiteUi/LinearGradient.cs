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

// fyrox_lite::lite_ui::LinearGradient
[StructLayout(LayoutKind.Sequential)]
public struct LinearGradient
{
    public Vector2 From {
        get => _from;
        set => _from = value;
    }
    public Vector2 To {
        get => _to;
        set => _to = value;
    }
    public List<GradientPoint> Stops {
        get => GradientPoint_array.ToFacade(_stops);
        set => _stops = GradientPoint_array.FromFacade(value);
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private Vector2 _from;
    private Vector2 _to;
    private GradientPoint_array _stops;
}
[StructLayout(LayoutKind.Sequential)]
internal struct LinearGradient_optional {
    internal LinearGradient Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static LinearGradient? ToFacade(in LinearGradient_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static LinearGradient_optional FromFacade(in LinearGradient? value) => new LinearGradient_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct LinearGradient_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal LinearGradient value;
    [FieldOffset(sizeof(int))]
    internal string err;
}