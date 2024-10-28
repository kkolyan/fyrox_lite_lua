using FyroxLite.LiteWindow;
using FyroxLite.LiteInput;
using FyroxLite.LiteMath;
using FyroxLite.LiteLog;
using FyroxLite.LitePrefab;
using FyroxLite.LiteUi;
using FyroxLite.LitePlugin;
using FyroxLite.LitePhysics;
using FyroxLite.LiteNode;
using FyroxLite.LiteScene;
using System.Runtime.InteropServices;
namespace FyroxLite.LiteScene;

// fyrox_lite::lite_scene::LiteScene
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Scene
{

    public static void LoadAsync(string scene_path)
    {
        unsafe {
            fyrox_lite_lite_scene_LiteScene_LoadAsync(scene_path);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_scene_LiteScene_LoadAsync(string scene_path);
}