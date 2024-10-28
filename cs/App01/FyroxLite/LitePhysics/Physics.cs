using FyroxLite.LiteWindow;
using FyroxLite.LiteInput;
using FyroxLite.LiteMath;
using FyroxLite.LiteLog;
using FyroxLite.LitePrefab;
using FyroxLite.LiteUi;
using FyroxLite.LitePlugin;
using FyroxLite.LitePhysics;
using FyroxLite.LiteNode;
using FyroxLite.LiteScene;
using System.Runtime.InteropServices;
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

    public static List<Intersection> CastRay(RayCastOptions opts)
    {
        unsafe {
            return fyrox_lite_lite_physics_LitePhysics_CastRay(opts);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial List<Intersection> fyrox_lite_lite_physics_LitePhysics_CastRay(RayCastOptions opts);
}