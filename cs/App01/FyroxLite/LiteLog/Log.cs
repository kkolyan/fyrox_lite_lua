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
namespace FyroxLite.LiteLog;

// fyrox_lite::lite_log::LiteLog
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Log
{

    public static void Info(string msg)
    {
        unsafe {
            fyrox_lite_lite_log_LiteLog_Info(msg);
        }
    }

    public static void Warn(string msg)
    {
        unsafe {
            fyrox_lite_lite_log_LiteLog_Warn(msg);
        }
    }

    public static void Err(string msg)
    {
        unsafe {
            fyrox_lite_lite_log_LiteLog_Err(msg);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_Info(string msg);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_Warn(string msg);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_Err(string msg);
}