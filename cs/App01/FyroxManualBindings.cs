using System.Runtime.InteropServices;
using FyroxLite;
using FyroxLite.LiteMath;
internal partial class FyroxManualBindings {
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static partial void init_fyrox(NativeScriptedApp app);
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct NativeInstanceId {
        internal int value;
    }
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct NativeClassId {
        internal int value;
    }
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct NativeScriptProperty {
        internal byte* name;
        internal NativeValueType ty;
    }
    internal enum NativeValueType {
        @bool,
        f32,
        f64,
        i16,
        i32,
        i64,
        String,
        Node,
        UiNode,
        Prefab,
        Vector3,
        Quaternion,
    }
    [StructLayout(LayoutKind.Explicit)]
    internal unsafe struct NativeValue {
        [FieldOffset(0)]
        internal bool @bool;
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
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct NativeHandle {
        internal ulong high;
        internal ulong low;
    }
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct NativeScriptMetadata {
        internal NativeClassId id;
        internal byte* uuid;
        internal NativeScriptKind kind;
        internal byte* name;
        internal byte* parent;
        internal bool has_on_init;
        internal bool has_on_start;
        internal bool has_on_deinit;
        internal bool has_on_os_event;
        internal bool has_on_update;
        internal bool has_on_message;
        internal NativeScriptProperty* properties;
        internal ushort properties_len;
    }
    internal enum NativeScriptKind {
        Node,
        Global,
    }
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct NativeScriptedApp {
        internal NativeScriptMetadata* scripts;
        internal ushort scripts_len;
        internal NativeScriptAppFunctions functions;
    }
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct NativeScriptAppFunctions {
        internal IntPtr on_init;
        internal IntPtr on_start;
        internal IntPtr on_deinit;
        internal IntPtr on_os_event;
        internal IntPtr on_update;
        internal IntPtr on_message;
        internal IntPtr on_game_init;
        internal IntPtr on_game_update;
        internal IntPtr on_game_on_os_event;
        internal IntPtr create_script_instance;
        internal IntPtr set_property;
    }
    internal delegate void NodeOnUpdate(NativeInstanceId thiz, float dt);
    internal delegate void NodeOnInit(NativeInstanceId thiz);
    internal delegate void NodeOnDeinit(NativeInstanceId thiz);
    internal delegate void NodeOnStart(NativeInstanceId thiz);
    internal delegate void NodeOnOsEvent(NativeInstanceId thiz);
    internal delegate void NodeOnMessage(NativeInstanceId thiz);
    internal delegate void GameOnInit(NativeInstanceId thiz);
    internal delegate void GameOnUpdate(NativeInstanceId thiz);
    internal delegate void GameOnOsEvent(NativeInstanceId thiz);
    internal delegate NativeInstanceId CreateScriptInstance(NativeClassId thiz);
    internal delegate void SetProperty(NativeInstanceId thiz, ushort property, NativeValue value);
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe struct UserScriptMessage {
        internal long id;
    }
}