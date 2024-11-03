// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

// fyrox_lite::lite_input::Input
public static partial class Input
{
    public const int MouseLeft = 0;
    public const int MouseRight = 1;
    public const int MouseMiddle = 2;
    public const int MouseBack = 3;
    public const int MouseForward = 4;

    public static bool IsMouseButtonDown(int button)
    {
        unsafe {
            var _button = button;
            var __ret = fyrox_lite_lite_input_Input_is_mouse_button_down(_button);
            return NativeBool.ToFacade(__ret);
        }
    }

    public static bool IsMouseButtonUp(int button)
    {
        unsafe {
            var _button = button;
            var __ret = fyrox_lite_lite_input_Input_is_mouse_button_up(_button);
            return NativeBool.ToFacade(__ret);
        }
    }

    public static bool IsMouseButton(int button)
    {
        unsafe {
            var _button = button;
            var __ret = fyrox_lite_lite_input_Input_is_mouse_button(_button);
            return NativeBool.ToFacade(__ret);
        }
    }

    public static bool IsKeyDown(KeyCode key)
    {
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key_down(_key);
            return NativeBool.ToFacade(__ret);
        }
    }

    public static bool IsKeyUp(KeyCode key)
    {
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key_up(_key);
            return NativeBool.ToFacade(__ret);
        }
    }

    public static bool IsKey(KeyCode key)
    {
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key(_key);
            return NativeBool.ToFacade(__ret);
        }
    }
    public static Vector2 MouseMove
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_input_Input_get_mouse_move();
                return NativeVector2.ToFacade(__ret);
            }
        }
    }
    public static Vector2 MouseScroll
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_input_Input_get_mouse_scroll();
                return NativeVector2.ToFacade(__ret);
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_mouse_button_down(int button);

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_mouse_button_up(int button);

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_mouse_button(int button);

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_key_down(KeyCode key);

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_key_up(KeyCode key);

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_key(KeyCode key);

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeVector2 fyrox_lite_lite_input_Input_get_mouse_move();

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeVector2 fyrox_lite_lite_input_Input_get_mouse_scroll();
}