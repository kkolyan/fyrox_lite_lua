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


internal enum NativeValueType {
    @bool,
    f32,
    f64,
    i16,
    i32,
    i64,
    String,
    Handle,
    Prefab,
    Vector3,
    Vector2,
    Vector2I,
    Quaternion,
}