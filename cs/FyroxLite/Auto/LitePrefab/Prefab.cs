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
            return __ret;
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node fyrox_lite_lite_prefab_LitePrefab_instantiate_at(Prefab self, NativeVector3* position, NativeQuaternion* orientation);

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
    private Prefab value;
    private int has_value;

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