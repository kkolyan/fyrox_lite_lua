// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

// fyrox_lite::lite_physics::LitePhysics
public static partial class Physics
{
    //public const int EXCLUDE_FIXED = 1 << 1;
    //public const int EXCLUDE_KINEMATIC = 1 << 2;
    //public const int EXCLUDE_DYNAMIC = 1 << 3;
    //public const int EXCLUDE_SENSORS = 1 << 4;
    //public const int EXCLUDE_SOLIDS = 1 << 5;
    //public const int ONLY_DYNAMIC = LitePhysics :: EXCLUDE_FIXED | LitePhysics :: EXCLUDE_KINEMATIC;
    //public const int ONLY_KINEMATIC = LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_FIXED;
    //public const int ONLY_FIXED = LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_KINEMATIC;

    public static List<Intersection> CastRay(RayCastOptions opts)
    {
        unsafe {
            var _opts = opts;
            var __ret = fyrox_lite_lite_physics_LitePhysics_cast_ray(&_opts);
            return Intersection_slice.ToFacade(__ret);
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Intersection_slice fyrox_lite_lite_physics_LitePhysics_cast_ray(RayCastOptions* opts);
}