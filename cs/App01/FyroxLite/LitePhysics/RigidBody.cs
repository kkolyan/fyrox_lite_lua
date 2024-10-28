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