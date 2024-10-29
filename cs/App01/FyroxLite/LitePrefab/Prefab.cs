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
namespace FyroxLite.LitePrefab;

// fyrox_lite::lite_prefab::LitePrefab
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Prefab
{

    public Node InstantiateAt(Vector3 position,Quaternion orientation)
    {
        unsafe {
            fixed (Prefab* self = &this) return fyrox_lite_lite_prefab_LitePrefab_InstantiateAt(self, position,orientation);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node fyrox_lite_lite_prefab_LitePrefab_InstantiateAt(Prefab* self, Vector3 position,Quaternion orientation);
}