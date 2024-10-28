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