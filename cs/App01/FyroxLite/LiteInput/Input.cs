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
            return fyrox_lite_lite_input_Input_IsMouseButtonDown(button);
        }
    }

    public static bool IsMouseButtonUp(int button)
    {
        unsafe {
            return fyrox_lite_lite_input_Input_IsMouseButtonUp(button);
        }
    }

    public static bool IsMouseButton(int button)
    {
        unsafe {
            return fyrox_lite_lite_input_Input_IsMouseButton(button);
        }
    }

    public static bool IsKeyDown(KeyCode key)
    {
        unsafe {
            return fyrox_lite_lite_input_Input_IsKeyDown(key);
        }
    }

    public static bool IsKeyUp(KeyCode key)
    {
        unsafe {
            return fyrox_lite_lite_input_Input_IsKeyUp(key);
        }
    }

    public static bool IsKey(KeyCode key)
    {
        unsafe {
            return fyrox_lite_lite_input_Input_IsKey(key);
        }
    }

    public static Vector2 GetMouseMove()
    {
        unsafe {
            return fyrox_lite_lite_input_Input_GetMouseMove();
        }
    }

    public static Vector2 GetMouseScroll()
    {
        unsafe {
            return fyrox_lite_lite_input_Input_GetMouseScroll();
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_IsMouseButtonDown(int button);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_IsMouseButtonUp(int button);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_IsMouseButton(int button);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_IsKeyDown(KeyCode key);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_IsKeyUp(KeyCode key);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_input_Input_IsKey(KeyCode key);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector2 fyrox_lite_lite_input_Input_GetMouseMove();
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector2 fyrox_lite_lite_input_Input_GetMouseScroll();
}