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

// fyrox_lite::lite_physics::LiteRigidBody
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct RigidBody
{

    public void ApplyForce(Vector3 force)
    {
        unsafe {
            var _force = force;
            fyrox_lite_lite_physics_LiteRigidBody_ApplyForce(this, &_force);
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_physics_LiteRigidBody_ApplyForce(RigidBody self, Vector3* force);
}

[StructLayout(LayoutKind.Sequential)]
internal struct RigidBody_optional
{
    internal RigidBody Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RigidBody? ToFacade(in RigidBody_optional value)
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
    public static RigidBody_optional FromFacade(in RigidBody? value)
    {
        if (value == null)
        {
            return new RigidBody_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new RigidBody_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}