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
namespace FyroxLite.LitePlugin;

// fyrox_lite::lite_plugin::LitePlugin
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Plugin
{

    public static T Get<T>()
    {
        unsafe {
            return fyrox_lite_lite_plugin_LitePlugin_Get(typeof(T).Name);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial T fyrox_lite_lite_plugin_LitePlugin_Get(string class_name);
}