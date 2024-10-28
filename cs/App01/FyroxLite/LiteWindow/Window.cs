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
namespace FyroxLite.LiteWindow;

// fyrox_lite::lite_window::LiteWindow
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Window
{

    public static void SetCursorGrab(CursorGrabMode mode)
    {
        unsafe {
            fyrox_lite_lite_window_LiteWindow_SetCursorGrab(mode);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_window_LiteWindow_SetCursorGrab(CursorGrabMode mode);
}