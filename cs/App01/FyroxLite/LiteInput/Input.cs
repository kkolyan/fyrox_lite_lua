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
namespace FyroxLite.LiteInput;

// fyrox_lite::lite_input::Input
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Input
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
            return __ret;
        }
    }

    public static bool IsMouseButtonUp(int button)
    {
        unsafe {
            var _button = button;
            var __ret = fyrox_lite_lite_input_Input_is_mouse_button_up(_button);
            return __ret;
        }
    }

    public static bool IsMouseButton(int button)
    {
        unsafe {
            var _button = button;
            var __ret = fyrox_lite_lite_input_Input_is_mouse_button(_button);
            return __ret;
        }
    }

    public static bool IsKeyDown(KeyCode key)
    {
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key_down(_key);
            return __ret;
        }
    }

    public static bool IsKeyUp(KeyCode key)
    {
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key_up(_key);
            return __ret;
        }
    }

    public static bool IsKey(KeyCode key)
    {
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key(_key);
            return __ret;
        }
    }
    public static Vector2 MouseMove
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_input_Input_get_mouse_move();
                return __ret;
            }
        }
    }
    public static Vector2 MouseScroll
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_input_Input_get_mouse_scroll();
                return __ret;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_is_mouse_button_down(int button);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_is_mouse_button_up(int button);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_is_mouse_button(int button);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_is_key_down(KeyCode key);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_is_key_up(KeyCode key);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_is_key(KeyCode key);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector2 fyrox_lite_lite_input_Input_get_mouse_move();

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector2 fyrox_lite_lite_input_Input_get_mouse_scroll();
}

[StructLayout(LayoutKind.Sequential)]
internal struct Input_optional
{
    internal Input Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Input? ToFacade(in Input_optional value)
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
    public static Input_optional FromFacade(in Input? value)
    {
        if (value == null)
        {
            return new Input_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Input_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}