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
namespace FyroxLite.LiteUi;

// fyrox_lite::lite_ui::LiteText
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Text
{

    public void SetTextAsync(string text)
    {
        unsafe {
            fixed (Text* self = &this) fyrox_lite_lite_ui_LiteText_SetTextAsync(self, text);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_ui_LiteText_SetTextAsync(Text* self, string text);

    public static Text New(TextBuilder state)
    {
        unsafe {
            return fyrox_lite_lite_ui_LiteText_New(state);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Text fyrox_lite_lite_ui_LiteText_New(TextBuilder state);
}