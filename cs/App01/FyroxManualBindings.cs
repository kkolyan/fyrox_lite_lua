namespace App01;
using System.Runtime.InteropServices;
using App01;
public partial class FyroxManualBindings {
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void init_fyrox(NativeScriptedApp app);
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeInstanceId {
        public int value;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeClassId {
        public int value;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeScriptProperty {
        public byte* name;
        public NativeValueType ty;
    }
    public enum NativeValueType {
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
    public unsafe struct NativeValue {
        
        [FieldOffset(0)]
        public bool @bool;
        
        [FieldOffset(0)]
        public float f32;
        
        [FieldOffset(0)]
        public double f64;
        
        [FieldOffset(0)]
        public short i16;
        
        [FieldOffset(0)]
        public int i32;
        
        [FieldOffset(0)]
        public long i64;
        
        [FieldOffset(0)]
        public NativeString String;
        
        [FieldOffset(0)]
        public NativeHandle Handle;
        
        [FieldOffset(0)]
        public NativeVector3 Vector3;
        
        [FieldOffset(0)]
        public NativeQuaternion Quaternion;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeHandle {
        public ulong high;
        public ulong low;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeScriptMetadata {
        public NativeClassId id;
        public byte* uuid;
        public NativeScriptKind kind;
        public byte* name;
        public byte* parent;
        public bool has_on_init;
        public bool has_on_start;
        public bool has_on_deinit;
        public bool has_on_os_event;
        public bool has_on_update;
        public bool has_on_message;
        public NativeScriptProperty* properties;
        public ushort properties_len;
    }
    public enum NativeScriptKind {
        Node,
        Global,
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeScriptedApp {
        public NativeScriptMetadata* scripts;
        public ushort scripts_len;
        public NativeScriptAppFunctions functions;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeScriptAppFunctions {
        public IntPtr on_init;
        public IntPtr on_start;
        public IntPtr on_deinit;
        public IntPtr on_os_event;
        public IntPtr on_update;
        public IntPtr on_message;
        public IntPtr on_game_init;
        public IntPtr on_game_update;
        public IntPtr on_game_on_os_event;
        public IntPtr create_script_instance;
        public IntPtr set_property;
    }
    public delegate void NodeOnUpdate(NativeInstanceId thiz, float dt);
    public delegate void NodeOnInit(NativeInstanceId thiz);
    public delegate void NodeOnDeinit(NativeInstanceId thiz);
    public delegate void NodeOnStart(NativeInstanceId thiz);
    public delegate void NodeOnOsEvent(NativeInstanceId thiz);
    public delegate void NodeOnMessage(NativeInstanceId thiz);
    public delegate void GameOnInit(NativeInstanceId thiz);
    public delegate void GameOnUpdate(NativeInstanceId thiz);
    public delegate void GameOnOsEvent(NativeInstanceId thiz);
    public delegate NativeInstanceId CreateScriptInstance(NativeClassId thiz);
    public delegate void SetProperty(NativeInstanceId thiz, ushort property, NativeValue value);
}