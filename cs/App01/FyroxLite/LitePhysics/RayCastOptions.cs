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

// fyrox_lite::lite_physics::LiteRayCastOptions
[StructLayout(LayoutKind.Sequential)]
public struct RayCastOptions
{
    public Vector3 RayOrigin {
        get => _ray_origin;
        set => _ray_origin = value;
    }
    public Vector3 RayDirection {
        get => _ray_direction;
        set => _ray_direction = value;
    }
    public float MaxLen {
        get => _max_len;
        set => _max_len = value;
    }
    public InteractionGroups? Groups {
        get => InteractionGroups_optional.ToFacade(_groups);
        set => _groups = InteractionGroups_optional.FromFacade(value);
    }
    public bool SortResults {
        get => _sort_results;
        set => _sort_results = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private Vector3 _ray_origin;
    private Vector3 _ray_direction;
    private float _max_len;
    private InteractionGroups_optional _groups;
    private bool _sort_results;
}
[StructLayout(LayoutKind.Sequential)]
internal struct RayCastOptions_optional {
    internal RayCastOptions Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RayCastOptions? ToFacade(in RayCastOptions_optional value) => value.HasValue ? value.Value : null;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RayCastOptions_optional FromFacade(in RayCastOptions? value) => new RayCastOptions_optional { Value = value ?? default, HasValue = value.HasValue };
}
[StructLayout(LayoutKind.Explicit)]
internal struct RayCastOptions_result {
    [FieldOffset(0)]
    internal int ok;
    [FieldOffset(sizeof(int))]
    internal RayCastOptions value;
    [FieldOffset(sizeof(int))]
    internal string err;
}