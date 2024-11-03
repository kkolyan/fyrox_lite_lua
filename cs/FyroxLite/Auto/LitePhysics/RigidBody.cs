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

// fyrox_lite::lite_physics::LiteRigidBody
public partial struct RigidBody : IEquatable<RigidBody>
{
    private readonly NativeHandle handle;

    internal RigidBody(NativeHandle handle)
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

    public bool Equals(RigidBody other)
    {
        return handle.Equals(other.handle);
    }

    public override bool Equals(object? obj)
    {
        return obj is RigidBody other && Equals(other);
    }

    public override int GetHashCode()
    {
        return handle.GetHashCode();
    }

    public static bool operator ==(RigidBody left, RigidBody right)
    {
        return left.Equals(right);
    }

    public static bool operator !=(RigidBody left, RigidBody right)
    {
        return !left.Equals(right);
    }
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