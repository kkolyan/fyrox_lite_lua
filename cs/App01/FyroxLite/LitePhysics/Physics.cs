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

// fyrox_lite::lite_physics::LitePhysics
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Physics
{
    //public const int EXCLUDE_FIXED = 1 << 1;
    //public const int EXCLUDE_KINEMATIC = 1 << 2;
    //public const int EXCLUDE_DYNAMIC = 1 << 3;
    //public const int EXCLUDE_SENSORS = 1 << 4;
    //public const int EXCLUDE_SOLIDS = 1 << 5;
    //public const int ONLY_DYNAMIC = LitePhysics :: EXCLUDE_FIXED | LitePhysics :: EXCLUDE_KINEMATIC;
    //public const int ONLY_KINEMATIC = LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_FIXED;
    //public const int ONLY_FIXED = LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_KINEMATIC;

    public static List<Intersection> CastRay(RayCastOptions opts, List<Intersection> results)
    {
        unsafe {
            var _opts = opts;
            var __ret = fyrox_lite_lite_physics_LitePhysics_CastRay(_opts, _results);
            return Intersection_slice.ToFacade(__ret);
        }
    }
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Intersection_slice fyrox_lite_lite_physics_LitePhysics_CastRay(RayCastOptions opts, Intersection_slice results);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Physics_optional
{
    internal Physics Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Physics? ToFacade(in Physics_optional value)
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
    public static Physics_optional FromFacade(in Physics? value)
    {
        if (value == null)
        {
            return new Physics_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Physics_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}