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
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
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
    private static unsafe partial UserScript_result fyrox_lite_lite_plugin_LitePlugin_Get(string class_name);
}