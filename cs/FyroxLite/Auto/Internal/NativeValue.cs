// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite.Internal;
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
using System.Numerics;
using System.Drawing;
using FyroxLite.Internal;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.Internal;


[StructLayout(LayoutKind.Explicit)]
internal unsafe partial struct NativeValue {

    [FieldOffset(0)]
    internal NativeBool @bool;

    [FieldOffset(0)]
    internal float f32;

    [FieldOffset(0)]
    internal double f64;

    [FieldOffset(0)]
    internal short i16;

    [FieldOffset(0)]
    internal int i32;

    [FieldOffset(0)]
    internal long i64;

    [FieldOffset(0)]
    internal NativeString String;

    [FieldOffset(0)]
    internal NativeHandle Handle;

    [FieldOffset(0)]
    internal NativeVector3 Vector3;

    [FieldOffset(0)]
    internal NativeQuaternion Quaternion;
}