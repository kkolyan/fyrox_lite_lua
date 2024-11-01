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
namespace FyroxLite.LitePrefab;

// fyrox_lite::lite_prefab::LitePrefab
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Prefab
{

    public Node InstantiateAt(Vector3 position, Quaternion orientation)
    {
        unsafe {
            var _position = position;
            var _orientation = orientation;
            var __ret = fyrox_lite_lite_prefab_LitePrefab_instantiate_at(this, &_position, &_orientation);
            return __ret;
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node fyrox_lite_lite_prefab_LitePrefab_instantiate_at(Prefab self, Vector3* position, Quaternion* orientation);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Prefab_optional
{
    internal Prefab Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Prefab? ToFacade(in Prefab_optional value)
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
    public static Prefab_optional FromFacade(in Prefab? value)
    {
        if (value == null)
        {
            return new Prefab_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Prefab_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}