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
            var _scene_path = NativeString.FromFacade(scene_path);
            fyrox_lite_lite_scene_LiteScene_load_async(_scene_path);
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_scene_LiteScene_load_async(NativeString scene_path);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Scene_optional
{
    private Scene value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Scene? ToFacade(in Scene_optional value)
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
    public static Scene_optional FromFacade(in Scene? value)
    {
        if (value == null)
        {
            return new Scene_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Scene_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}