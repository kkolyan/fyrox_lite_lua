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
using FyroxLite.LiteBase;
using FyroxLite.Internal;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.Internal;


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