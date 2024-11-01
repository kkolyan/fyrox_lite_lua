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
public readonly partial static class Scene
{
    private readonly NativeHandle handle;

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