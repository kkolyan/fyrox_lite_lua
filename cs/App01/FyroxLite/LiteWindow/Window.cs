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