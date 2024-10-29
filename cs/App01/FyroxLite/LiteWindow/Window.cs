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
namespace FyroxLite.LiteWindow;

// fyrox_lite::lite_window::LiteWindow
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Window
{

    public static void SetCursorGrab(CursorGrabMode mode)
    {
        unsafe {
            var _mode = mode;
            fyrox_lite_lite_window_LiteWindow_SetCursorGrab(_mode);
        }
    }
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_window_LiteWindow_SetCursorGrab(CursorGrabMode mode);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Window_optional
{
    internal Window Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Window? ToFacade(in Window_optional value)
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
    public static Window_optional FromFacade(in Window? value)
    {
        if (value == null)
        {
            return new Window_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Window_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}