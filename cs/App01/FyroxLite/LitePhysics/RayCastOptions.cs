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
namespace FyroxLite.LitePhysics;

// fyrox_lite::lite_physics::LiteRayCastOptions
[StructLayout(LayoutKind.Sequential)]
public struct RayCastOptions
{
    public Vector3 RayOrigin;
    public Vector3 RayDirection;
    public float MaxLen;
    public InteractionGroups? Groups;
    public bool SortResults;
}