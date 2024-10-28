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
namespace FyroxLite.LiteMath;

// fyrox_lite::lite_math::PodVector3
[StructLayout(LayoutKind.Sequential)]
public struct Vector3
{
    public float X;
    public float Y;
    public float Z;
}