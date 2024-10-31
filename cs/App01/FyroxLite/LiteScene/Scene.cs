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
namespace FyroxLite.LiteScene;

// fyrox_lite::lite_scene::LiteScene
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Scene
{

    public static void LoadAsync(string scene_path)
    {
        unsafe {
            var _scene_path = scene_path;
            fyrox_lite_lite_scene_LiteScene_LoadAsync(_scene_path);
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_scene_LiteScene_LoadAsync(string scene_path);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Scene_optional
{
    internal Scene Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Scene? ToFacade(in Scene_optional value)
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
    public static Scene_optional FromFacade(in Scene? value)
    {
        if (value == null)
        {
            return new Scene_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Scene_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}