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

// fyrox_lite::lite_prefab::LitePrefab
public partial struct Prefab : IEquatable<Prefab>
{
    private readonly NativeHandle handle;

    internal Prefab(NativeHandle handle)
    {
        this.handle = handle;
    }

    public Node InstantiateAt(Vector3 position, Quaternion orientation)
    {
        unsafe {
            var _position = NativeVector3.FromFacade(position);
            var _orientation = NativeQuaternion.FromFacade(orientation);
            var __ret = fyrox_lite_lite_prefab_LitePrefab_instantiate_at(this, &_position, &_orientation);
            return Node_result.ToFacade(__ret);
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node_result fyrox_lite_lite_prefab_LitePrefab_instantiate_at(Prefab self, NativeVector3* position, NativeQuaternion* orientation);

    public bool Equals(Prefab other)
    {
        return handle.Equals(other.handle);
    }

    public override bool Equals(object? obj)
    {
        return obj is Prefab other && Equals(other);
    }

    public override int GetHashCode()
    {
        return handle.GetHashCode();
    }

    public static bool operator ==(Prefab left, Prefab right)
    {
        return left.Equals(right);
    }

    public static bool operator !=(Prefab left, Prefab right)
    {
        return !left.Equals(right);
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Prefab_optional
{
    internal Prefab value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Prefab? ToFacade(in Prefab_optional value)
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
    public static Prefab_optional FromFacade(in Prefab? value)
    {
        if (value == null)
        {
            return new Prefab_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new Prefab_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Prefab_result
{
    internal int ok;
    internal Prefab_result_value value;

    internal static unsafe Prefab ToFacade(in Prefab_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static Prefab_result FromFacade(in Prefab self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Prefab_result {ok = 1, value = new Prefab_result_value { ok = __item_from_facade } };
    }

    internal static Prefab_result FromFacadeError(in string err)
    {
        return new Prefab_result {ok = 0, value = new Prefab_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct Prefab_result_value
{
    [FieldOffset(0)]
    internal Prefab ok;

    [FieldOffset(0)]
    internal NativeString err;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Prefab_optional_result
{
    internal int ok;
    internal Prefab_optional_result_value value;

    internal static unsafe Prefab? ToFacade(in Prefab_optional_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = Prefab_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static Prefab_optional_result FromFacade(in Prefab? self)
    {
        var __item = self;
        var __item_from_facade = Prefab_optional.FromFacade(__item);
        return new Prefab_optional_result {ok = 1, value = new Prefab_optional_result_value { ok = __item_from_facade } };
    }

    internal static Prefab_optional_result FromFacadeError(in string err)
    {
        return new Prefab_optional_result {ok = 0, value = new Prefab_optional_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct Prefab_optional_result_value
{
    [FieldOffset(0)]
    internal Prefab_optional ok;

    [FieldOffset(0)]
    internal NativeString err;
}