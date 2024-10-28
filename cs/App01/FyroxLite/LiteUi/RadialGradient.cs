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

// fyrox_lite::lite_ui::RadialGradient
[StructLayout(LayoutKind.Sequential)]
public struct RadialGradient
{
    public Vector2 Center {
        get => _center;
        set => _center = value;
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
    private Vector2 _center;
    private GradientPoint_array _stops;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RadialGradient_optional {
    internal RadialGradient Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RadialGradient? ToFacade(in RadialGradient_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RadialGradient_optional FromFacade(in RadialGradient? value) => new RadialGradient_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct RadialGradient_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal RadialGradient value;
    [FieldOffset(sizeof(int))]
    internal string err;
}