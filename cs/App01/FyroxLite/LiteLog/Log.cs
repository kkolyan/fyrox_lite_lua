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
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_Info(string msg);

    public static void Warn(string msg)
    {
        unsafe {
            fyrox_lite_lite_log_LiteLog_Warn(msg);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_Warn(string msg);

    public static void Err(string msg)
    {
        unsafe {
            fyrox_lite_lite_log_LiteLog_Err(msg);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_Err(string msg);
}