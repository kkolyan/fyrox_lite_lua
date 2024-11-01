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
    public static CursorGrabMode CursorGrab
    {
        set
        {
            unsafe {
                var _value = value;
                fyrox_lite_lite_window_LiteWindow_set_cursor_grab(_value);
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_window_LiteWindow_set_cursor_grab(CursorGrabMode mode);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Window_optional
{
    private Window value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Window? ToFacade(in Window_optional value)
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
    public static Window_optional FromFacade(in Window? value)
    {
        if (value == null)
        {
            return new Window_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Window_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}