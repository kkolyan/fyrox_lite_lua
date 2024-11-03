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

// fyrox_lite::lite_scene::LiteScene
public static partial class Scene
{

    public static void LoadAsync(string scene_path)
    {
        unsafe {
            var _scene_path = NativeString.FromFacade(scene_path);
            fyrox_lite_lite_scene_LiteScene_load_async(_scene_path);
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_scene_LiteScene_load_async(NativeString scene_path);
}