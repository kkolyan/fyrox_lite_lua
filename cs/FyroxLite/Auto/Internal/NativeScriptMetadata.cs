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
internal unsafe partial struct NativeScriptMetadata {
    internal NativeClassId id;
    internal NativeString uuid;
    internal NativeScriptKind kind;
    internal NativeString name;
    internal NativeBool has_global_on_init;
    internal NativeBool has_global_on_update;
    internal NativeBool has_node_on_init;
    internal NativeBool has_node_on_start;
    internal NativeBool has_node_on_deinit;
    internal NativeBool has_node_on_update;
    internal NativeBool has_node_on_message;
    internal NativeScriptProperty_slice properties;
}