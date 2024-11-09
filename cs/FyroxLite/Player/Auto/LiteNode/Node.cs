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

// fyrox_lite::lite_node::LiteNode
public partial struct Node : IEquatable<Node>
{
    private readonly NativeHandle handle;

    internal Node(NativeHandle handle)
    {
        this.handle = handle;
    }

    public RigidBody? AsRigidBody()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_as_rigid_body(this);
            return RigidBody_optional_result.ToFacade(__ret);
        }
    }
    public string Name
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_name(this);
                return NativeString_result.ToFacade(__ret);
            }
        }
    }
    public bool Alive
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_alive(this);
                return NativeBool.ToFacade(__ret);
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
                return NativeVector3.ToFacade(__ret);
            }
        }
    }
    public Vector3 LocalPosition
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_local_position(this);
                return NativeVector3.ToFacade(__ret);
            }
        }
        set
        {
            unsafe {
                var _value = NativeVector3.FromFacade(value);
                var __ret = fyrox_lite_lite_node_LiteNode_set_local_position(this, &_value);
                void_result.ToFacade(__ret);
            }
        }
    }
    public Quaternion LocalRotation
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_local_rotation(this);
                return NativeQuaternion.ToFacade(__ret);
            }
        }
        set
        {
            unsafe {
                var _value = NativeQuaternion.FromFacade(value);
                var __ret = fyrox_lite_lite_node_LiteNode_set_local_rotation(this, &_value);
                void_result.ToFacade(__ret);
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
                return NativeBool.ToFacade(__ret);
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
            
            var __ret = fyrox_lite_lite_node_LiteNode_add_script(this, NativeClassId.By<T>.Resolve());
            return NativeInstanceId_result.ToFacade(__ret) as T;
        }
    }

    public T? FindScript<T>() where T : class
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_find_script(this, NativeClassId.By<T>.Resolve());
            return NativeInstanceId_optional_result.ToFacade(__ret) as T;
        }
    }
    public Quaternion GlobalRotation
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_global_rotation(this);
                return NativeQuaternion.ToFacade(__ret);
            }
        }
    }

    public bool TagIs(string tag)
    {
        unsafe {
            var _tag = NativeString.FromFacade(tag);
            var __ret = fyrox_lite_lite_node_LiteNode_tag_is(this, _tag);
            return NativeBool.ToFacade(__ret);
        }
    }
    public string Tag
    {
        get
        {
            unsafe {
                var __ret = fyrox_lite_lite_node_LiteNode_get_tag(this);
                return NativeString.ToFacade(__ret);
            }
        }
        set
        {
            unsafe {
                var _value = NativeString.FromFacade(value);
                fyrox_lite_lite_node_LiteNode_set_tag(this, _value);
            }
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial RigidBody_optional_result fyrox_lite_lite_node_LiteNode_as_rigid_body(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeString_result fyrox_lite_lite_node_LiteNode_get_name(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_node_LiteNode_get_alive(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_destroy(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeVector3 fyrox_lite_lite_node_LiteNode_get_global_position(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeVector3 fyrox_lite_lite_node_LiteNode_get_local_position(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeQuaternion fyrox_lite_lite_node_LiteNode_get_local_rotation(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_send_hierarchical(Node self, RoutingStrategy routing, UserScriptMessage payload);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void_result fyrox_lite_lite_node_LiteNode_set_local_position(Node self, NativeVector3* new_pos);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void_result fyrox_lite_lite_node_LiteNode_set_local_rotation(Node self, NativeQuaternion* value);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_subscribe_to(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node_optional fyrox_lite_lite_node_LiteNode_find_collider_in_children(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_node_LiteNode_get_valid(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node fyrox_lite_lite_node_LiteNode_get_parent(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeInstanceId_result fyrox_lite_lite_node_LiteNode_add_script(Node self, NativeClassId class_id);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeInstanceId_optional_result fyrox_lite_lite_node_LiteNode_find_script(Node self, NativeClassId class_id);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeQuaternion fyrox_lite_lite_node_LiteNode_get_global_rotation(Node self);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_node_LiteNode_tag_is(Node self, NativeString tag);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_set_tag(Node self, NativeString tag);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeString fyrox_lite_lite_node_LiteNode_get_tag(Node self);

    public bool Equals(Node other)
    {
        return handle.Equals(other.handle);
    }

    public override bool Equals(object? obj)
    {
        return obj is Node other && Equals(other);
    }

    public override int GetHashCode()
    {
        return handle.GetHashCode();
    }

    public static bool operator ==(Node left, Node right)
    {
        return left.Equals(right);
    }

    public static bool operator !=(Node left, Node right)
    {
        return !left.Equals(right);
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Node_optional
{
    internal Node value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Node? ToFacade(in Node_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
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
            return new Node_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new Node_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Node_result
{
    internal int ok;
    internal Node_result_value value;

    internal static unsafe Node ToFacade(in Node_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static Node_result FromFacade(in Node self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Node_result {ok = 1, value = new Node_result_value { ok = __item_from_facade } };
    }

    internal static Node_result FromFacadeError(in string err)
    {
        return new Node_result {ok = 0, value = new Node_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct Node_result_value
{
    [FieldOffset(0)]
    internal Node ok;

    [FieldOffset(0)]
    internal NativeString err;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Node_optional_result
{
    internal int ok;
    internal Node_optional_result_value value;

    internal static unsafe Node? ToFacade(in Node_optional_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = Node_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static Node_optional_result FromFacade(in Node? self)
    {
        var __item = self;
        var __item_from_facade = Node_optional.FromFacade(__item);
        return new Node_optional_result {ok = 1, value = new Node_optional_result_value { ok = __item_from_facade } };
    }

    internal static Node_optional_result FromFacadeError(in string err)
    {
        return new Node_optional_result {ok = 0, value = new Node_optional_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct Node_optional_result_value
{
    [FieldOffset(0)]
    internal Node_optional ok;

    [FieldOffset(0)]
    internal NativeString err;
}