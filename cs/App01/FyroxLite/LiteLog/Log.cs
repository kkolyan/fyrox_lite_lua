// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
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
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LiteLog;

// fyrox_lite::lite_log::LiteLog
[StructLayout(LayoutKind.Sequential)]
public static partial class Log
{

    public static void Info(string msg)
    {
        unsafe {
            var _msg = NativeString.FromFacade(msg);
            fyrox_lite_lite_log_LiteLog_info(_msg);
        }
    }

    public static void Warn(string msg)
    {
        unsafe {
            var _msg = NativeString.FromFacade(msg);
            fyrox_lite_lite_log_LiteLog_warn(_msg);
        }
    }

    public static void Err(string msg)
    {
        unsafe {
            var _msg = NativeString.FromFacade(msg);
            fyrox_lite_lite_log_LiteLog_err(_msg);
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_info(NativeString msg);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_warn(NativeString msg);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_err(NativeString msg);
}