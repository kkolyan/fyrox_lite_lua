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
internal struct RayCastOptions_optional
{
    internal RayCastOptions Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RayCastOptions? ToFacade(in RayCastOptions_optional value)
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
    public static RayCastOptions_optional FromFacade(in RayCastOptions? value)
    {
        if (value == null)
        {
            return new RayCastOptions_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new RayCastOptions_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct RayCastOptions_slice
{
    private unsafe RayCastOptions* begin;
    private int length;
    internal List<RayCastOptions> Fetched;

    internal static unsafe void Fetch(ref RayCastOptions_slice self)
    {
        var fetched = new List<RayCastOptions>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<RayCastOptions> ToFacade(in RayCastOptions_slice self)
    {
        var fetched = new List<RayCastOptions>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static RayCastOptions_slice FromFacade(in List<RayCastOptions> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct RayCastOptions_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal RayCastOptions Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe RayCastOptions ToFacade(in RayCastOptions_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static RayCastOptions_result FromFacade(in RayCastOptions self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new RayCastOptions_result {Ok = 1, Value = __item_from_facade};
    }
}