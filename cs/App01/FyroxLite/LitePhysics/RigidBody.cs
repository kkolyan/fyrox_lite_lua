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

// fyrox_lite::lite_physics::LiteRigidBody
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct RigidBody
{

    public void ApplyForce(Vector3 force)
    {
        unsafe {
            fixed (RigidBody* self = &this) fyrox_lite_lite_physics_LiteRigidBody_ApplyForce(self, force);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_physics_LiteRigidBody_ApplyForce(RigidBody* self, Vector3 force);
}