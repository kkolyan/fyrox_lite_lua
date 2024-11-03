// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;


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
    internal NativeVector2 Vector2;

    [FieldOffset(0)]
    internal NativeVector2I Vector2I;

    [FieldOffset(0)]
    internal NativeQuaternion Quaternion;
}