// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Numerics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;


[StructLayout(LayoutKind.Sequential)]
internal unsafe partial struct NativeScriptProperty {
    internal int id;
    internal NativeString name;
    internal NativeValueType ty;
    internal NativeBool hide_in_inspector;
    internal NativeBool transient;
}