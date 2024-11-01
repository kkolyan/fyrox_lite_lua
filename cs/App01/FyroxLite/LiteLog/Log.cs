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
namespace FyroxLite.LiteLog;

// fyrox_lite::lite_log::LiteLog
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Log
{

    public static void Info(string msg)
    {
        unsafe {
            var _msg = msg;
            fyrox_lite_lite_log_LiteLog_info(_msg);
        }
    }

    public static void Warn(string msg)
    {
        unsafe {
            var _msg = msg;
            fyrox_lite_lite_log_LiteLog_warn(_msg);
        }
    }

    public static void Err(string msg)
    {
        unsafe {
            var _msg = msg;
            fyrox_lite_lite_log_LiteLog_err(_msg);
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_info(string msg);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_warn(string msg);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_err(string msg);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Log_optional
{
    internal Log Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Log? ToFacade(in Log_optional value)
    {
        if (value.HasValue)
        {
            var __item = value.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Log_optional FromFacade(in Log? value)
    {
        if (value == null)
        {
            return new Log_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Log_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}