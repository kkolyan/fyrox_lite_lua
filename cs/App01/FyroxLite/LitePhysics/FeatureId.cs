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