namespace App01;
using App01;
using System.Runtime.InteropServices;
public partial class FyroxLiteBindings {
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeHandle fyrox_lite_Prefab_instantiate_at(NativeHandle __this, NativeVector3 position, NativeQuaternion orientation);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Log_info(NativeString msg);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Log_warn(NativeString msg);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Log_err(NativeString msg);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Window_set_cursor_grab(NativeCursorGrabMode mode);
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeCursorGrabMode {
        public ubyte tag;
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Scene_load_async(NativeString scene_path);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeIntersection_array fyrox_lite_Physics_cast_ray(NativeRayCastOptions opts);
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeIntersection {
        public NativeHandle collider;
        public NativeVector3 normal;
        public NativeVector3 position;
        public NativeFeatureId feature;
        public float toi;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeFeatureId {
        public ubyte tag;
        public NativeFeatureIdVariantContainer value;
    }
    [StructLayout(LayoutKind.Explicit)]
    public unsafe struct NativeFeatureIdVariantContainer {
        
        [FieldOffset(0)]
        public NativeFeatureId_Vertex Vertex;
        
        [FieldOffset(0)]
        public NativeFeatureId_Edge Edge;
        
        [FieldOffset(0)]
        public NativeFeatureId_Face Face;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeFeatureId_Vertex {
        public int _0;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeFeatureId_Edge {
        public int _0;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeFeatureId_Face {
        public int _0;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeRayCastOptions {
        public NativeVector3 ray_origin;
        public NativeVector3 ray_direction;
        public float max_len;
        public NativeInteractionGroups_option groups;
        public bool sort_results;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeInteractionGroups {
        public int memberships;
        public int filter;
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_RigidBody_apply_force(NativeHandle __this, NativeVector3 force);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Input_is_mouse_button_down(int button);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Input_is_mouse_button_up(int button);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Input_is_mouse_button(int button);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Input_is_key_down(NativeKeyCode key);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Input_is_key_up(NativeKeyCode key);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Input_is_key(NativeKeyCode key);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeVector2 fyrox_lite_Input_get_mouse_move();
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeVector2 fyrox_lite_Input_get_mouse_scroll();
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeKeyCode {
        public ubyte tag;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeMouseButton {
        public ubyte tag;
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Text_set_text_async(NativeHandle __this, NativeString text);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeHandle fyrox_lite_Text_new(NativeTextBuilder state);
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeTextBuilder {
        public NativeBrush_option foregound;
        public f32_option font_size;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeBrush {
        public ubyte tag;
        public NativeBrushVariantContainer value;
    }
    [StructLayout(LayoutKind.Explicit)]
    public unsafe struct NativeBrushVariantContainer {
        
        [FieldOffset(0)]
        public NativeBrush_Solid Solid;
        
        [FieldOffset(0)]
        public NativeBrush_LinearGradient LinearGradient;
        
        [FieldOffset(0)]
        public NativeBrush_RadialGradient RadialGradient;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeBrush_Solid {
        public NativeHandle _0;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeBrush_LinearGradient {
        public NativeVector2 from;
        public NativeVector2 to;
        public NativeGradientPoint_array stops;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeBrush_RadialGradient {
        public NativeVector2 center;
        public NativeGradientPoint_array stops;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeGradientPoint {
        public float stop;
        public NativeHandle color;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeVector3 {
        public float x;
        public float y;
        public float z;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeVector2 {
        public float x;
        public float y;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeVector2i {
        public long x;
        public long y;
    }
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeQuaternion {
        public float i;
        public float j;
        public float k;
        public float w;
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeHandle_result fyrox_lite_Plugin_get(NativeString class_name);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeHandle_option fyrox_lite_Node_as_rigid_body(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeString_option fyrox_lite_Node_get_name(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Node_get_alive(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Node_destroy(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeVector3 fyrox_lite_Node_get_global_position(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeVector3 fyrox_lite_Node_get_local_position(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeQuaternion fyrox_lite_Node_get_local_rotation(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Node_send_hierarchical(NativeHandle __this, NativeRoutingStrategy routing, NativeInstanceId payload);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Node_set_local_position(NativeHandle __this, NativeVector3 new_pos);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Node_set_local_rotation(NativeHandle __this, NativeQuaternion value);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Node_subscribe_to(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeHandle_option fyrox_lite_Node_find_collider_in_children(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Node_get_valid(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeHandle fyrox_lite_Node_get_parent(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeHandle_result fyrox_lite_Node_add_script(NativeHandle __this, NativeString class_name);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeHandle_option_result fyrox_lite_Node_find_script(NativeHandle __this, NativeString class_name);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeQuaternion fyrox_lite_Node_get_global_rotation(NativeHandle __this);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial bool fyrox_lite_Node_tag_is(NativeHandle __this, NativeString tag);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial void fyrox_lite_Node_set_tag(NativeHandle __this, NativeString tag);
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    public static partial NativeString fyrox_lite_Node_get_tag(NativeHandle __this);
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct NativeRoutingStrategy {
        public ubyte tag;
    }
}