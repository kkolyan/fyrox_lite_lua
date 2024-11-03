// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite.Internal;
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
using System.Numerics;
using System.Drawing;
using FyroxLite.LiteBase;
using FyroxLite.Internal;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LitePlugin;

// fyrox_lite::lite_plugin::LitePlugin
[StructLayout(LayoutKind.Sequential)]
public static partial class Plugin
{

    public static T Get<T>() where T : class
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_plugin_LitePlugin_get(NativeString.FromFacade(typeof(T).Name));
            return UserScript_result.ToFacade(__ret) as T;
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial UserScript_result fyrox_lite_lite_plugin_LitePlugin_get(NativeString class_name);
}