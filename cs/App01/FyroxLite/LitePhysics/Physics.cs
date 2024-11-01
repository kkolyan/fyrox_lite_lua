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
public readonly partial static class Physics
{
    private readonly NativeHandle handle;
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

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Intersection_slice fyrox_lite_lite_physics_LitePhysics_cast_ray(RayCastOptions* opts);
}