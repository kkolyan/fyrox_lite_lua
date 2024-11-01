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
using FyroxLite.LiteBase;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LiteNode;

// fyrox_lite::lite_node::LiteNode
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Node
{

    public RigidBody? AsRigidBody()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_as_rigid_body(this);
            return RigidBody_optional.ToFacade(__ret);
        }
    }
    public string Name
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_name(this);
                return string_result.ToFacade(__ret);
            }
        }
    }
    public bool Alive
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_alive(this);
                return __ret;
            }
        }
    }

    public void Destroy()
    {
        unsafe {
            
            fyrox_lite_lite_node_LiteNode_destroy(this);
        }
    }
    public Vector3 GlobalPosition
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_global_position(this);
                return __ret;
            }
        }
    }
    public Vector3 LocalPosition
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_local_position(this);
                return __ret;
            }
        }
        set
        {
            unsafe {
                var _value = value;
                fyrox_lite_lite_node_LiteNode_set_local_position(this, &_value);
            }
        }
    }
    public Quaternion LocalRotation
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_local_rotation(this);
                return __ret;
            }
        }
        set
        {
            unsafe {
                var _value = value;
                fyrox_lite_lite_node_LiteNode_set_local_rotation(this, &_value);
            }
        }
    }

    public void SendHierarchical(RoutingStrategy routing, object payload)
    {
        unsafe {
            var _routing = routing;
            var _payload = UserScriptMessage.FromFacade(payload);
            fyrox_lite_lite_node_LiteNode_send_hierarchical(this, _routing, _payload);
        }
    }

    public void SubscribeTo()
    {
        unsafe {
            
            fyrox_lite_lite_node_LiteNode_subscribe_to(this);
        }
    }

    public Node? FindColliderInChildren()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_find_collider_in_children(this);
            return Node_optional.ToFacade(__ret);
        }
    }
    public bool Valid
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_valid(this);
                return __ret;
            }
        }
    }
    public Node Parent
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_parent(this);
                return __ret;
            }
        }
    }

    public T AddScript<T>() where T : class
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_add_script(this, typeof(T).Name);
            return UserScript_result.ToFacade(__ret) as T;
        }
    }

    public T? FindScript<T>() where T : class
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_find_script(this, typeof(T).Name);
            return UserScript_optional_result.ToFacade(__ret) as T;
        }
    }
    public Quaternion GlobalRotation
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_global_rotation(this);
                return __ret;
            }
        }
    }

    public bool TagIs(string tag)
    {
        unsafe {
            var _tag = tag;
            var __ret = fyrox_lite_lite_node_LiteNode_tag_is(this, _tag);
            return __ret;
        }
    }
    public string Tag
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_tag(this);
                return __ret;
            }
        }
        set
        {
            unsafe {
                var _value = value;
                fyrox_lite_lite_node_LiteNode_set_tag(this, _value);
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial RigidBody_optional fyrox_lite_lite_node_LiteNode_as_rigid_body(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial string_result fyrox_lite_lite_node_LiteNode_get_name(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_get_alive(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_destroy(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector3 fyrox_lite_lite_node_LiteNode_get_global_position(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector3 fyrox_lite_lite_node_LiteNode_get_local_position(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Quaternion fyrox_lite_lite_node_LiteNode_get_local_rotation(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_send_hierarchical(Node self, RoutingStrategy routing, UserScriptMessage payload);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_set_local_position(Node self, Vector3* new_pos);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_set_local_rotation(Node self, Quaternion* value);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_subscribe_to(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node_optional fyrox_lite_lite_node_LiteNode_find_collider_in_children(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_get_valid(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node fyrox_lite_lite_node_LiteNode_get_parent(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial UserScript_result fyrox_lite_lite_node_LiteNode_add_script(Node self, string class_name);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial UserScript_optional_result fyrox_lite_lite_node_LiteNode_find_script(Node self, string class_name);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Quaternion fyrox_lite_lite_node_LiteNode_get_global_rotation(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_tag_is(Node self, string tag);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_set_tag(Node self, string tag);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial string fyrox_lite_lite_node_LiteNode_get_tag(Node self);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Node_optional
{
    internal Node Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Node? ToFacade(in Node_optional value)
    {
        if (value.HasValue)
        {
            var __item = value.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Node_optional FromFacade(in Node? value)
    {
        if (value == null)
        {
            return new Node_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Node_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}