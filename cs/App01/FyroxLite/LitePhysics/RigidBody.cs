// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite.Internal;
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
using System.Numerics;
using System.Drawing;
using FyroxLite.Internal;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LitePhysics;

// fyrox_lite::lite_physics::LiteRigidBody
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct RigidBody
{
    private readonly NativeHandle handle;

    public RigidBody(NativeHandle handle)
    {
        this.handle = handle;
    }

    public void ApplyForce(Vector3 force)
    {
        unsafe {
            var _force = NativeVector3.FromFacade(force);
            fyrox_lite_lite_physics_LiteRigidBody_apply_force(this, &_force);
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_physics_LiteRigidBody_apply_force(RigidBody self, NativeVector3* force);
}

[StructLayout(LayoutKind.Sequential)]
internal struct RigidBody_optional
{
    private RigidBody value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RigidBody? ToFacade(in RigidBody_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
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
            return new RigidBody_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new RigidBody_optional { value = __item_from_facade, has_value = 1 };
    }
}