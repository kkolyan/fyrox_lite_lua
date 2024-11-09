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

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
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
    internal RigidBody value;
    internal int has_value;

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

[StructLayout(LayoutKind.Sequential)]
internal struct RigidBody_result
{
    internal int ok;
    internal RigidBody_result_value value;

    internal static unsafe RigidBody ToFacade(in RigidBody_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static RigidBody_result FromFacade(in RigidBody self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new RigidBody_result {ok = 1, value = new RigidBody_result_value { ok = __item_from_facade } };
    }

    internal static RigidBody_result FromFacadeError(in string err)
    {
        return new RigidBody_result {ok = 0, value = new RigidBody_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct RigidBody_result_value
{
    [FieldOffset(0)]
    internal RigidBody ok;

    [FieldOffset(0)]
    internal NativeString err;
}

[StructLayout(LayoutKind.Sequential)]
internal struct RigidBody_optional_result
{
    internal int ok;
    internal RigidBody_optional_result_value value;

    internal static unsafe RigidBody? ToFacade(in RigidBody_optional_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = RigidBody_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static RigidBody_optional_result FromFacade(in RigidBody? self)
    {
        var __item = self;
        var __item_from_facade = RigidBody_optional.FromFacade(__item);
        return new RigidBody_optional_result {ok = 1, value = new RigidBody_optional_result_value { ok = __item_from_facade } };
    }

    internal static RigidBody_optional_result FromFacadeError(in string err)
    {
        return new RigidBody_optional_result {ok = 0, value = new RigidBody_optional_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct RigidBody_optional_result_value
{
    [FieldOffset(0)]
    internal RigidBody_optional ok;

    [FieldOffset(0)]
    internal NativeString err;
}