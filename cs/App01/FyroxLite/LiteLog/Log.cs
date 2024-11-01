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

[StructLayout(LayoutKind.Sequential)]
internal struct Log_optional
{
    private Log value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Log? ToFacade(in Log_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
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
            return new Log_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Log_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}