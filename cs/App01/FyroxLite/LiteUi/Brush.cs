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

// fyrox_lite::lite_ui::Brush
[StructLayout(LayoutKind.Sequential)]
public struct Brush
{
    public Color? SolidColor;
    public LinearGradient? LinearGradient;
    public RadialGradient? RadialGradient;
}