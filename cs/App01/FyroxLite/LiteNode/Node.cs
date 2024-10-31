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
            
            var __ret = fyrox_lite_lite_node_LiteNode_AsRigidBody(this);
            return RigidBody_optional.ToFacade(__ret);
        }
    }

    public string GetName()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetName(this);
            return string_result.ToFacade(__ret);
        }
    }

    public bool GetAlive()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetAlive(this);
            return __ret;
        }
    }

    public void Destroy()
    {
        unsafe {
            
            fyrox_lite_lite_node_LiteNode_Destroy(this);
        }
    }

    public Vector3 GetGlobalPosition()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetGlobalPosition(this);
            return __ret;
        }
    }

    public Vector3 GetLocalPosition()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetLocalPosition(this);
            return __ret;
        }
    }

    public Quaternion GetLocalRotation()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetLocalRotation(this);
            return __ret;
        }
    }

    public void SendHierarchical(RoutingStrategy routing, object payload)
    {
        unsafe {
            var _routing = routing;
            var _payload = UserScriptMessage.FromFacade(payload);
            fyrox_lite_lite_node_LiteNode_SendHierarchical(this, _routing, _payload);
        }
    }

    public void SetLocalPosition(Vector3 new_pos)
    {
        unsafe {
            var _new_pos = new_pos;
            fyrox_lite_lite_node_LiteNode_SetLocalPosition(this, &_new_pos);
        }
    }

    public void SetLocalRotation(Quaternion value)
    {
        unsafe {
            var _value = value;
            fyrox_lite_lite_node_LiteNode_SetLocalRotation(this, &_value);
        }
    }

    public void SubscribeTo()
    {
        unsafe {
            
            fyrox_lite_lite_node_LiteNode_SubscribeTo(this);
        }
    }

    public Node? FindColliderInChildren()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_FindColliderInChildren(this);
            return Node_optional.ToFacade(__ret);
        }
    }

    public bool GetValid()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetValid(this);
            return __ret;
        }
    }

    public Node GetParent()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetParent(this);
            return __ret;
        }
    }

    public T AddScript<T>() where T : class
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_AddScript(this, typeof(T).Name);
            return UserScript_result.ToFacade(__ret) as T;
        }
    }

    public T? FindScript<T>() where T : class
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_FindScript(this, typeof(T).Name);
            return UserScript_optional_result.ToFacade(__ret) as T;
        }
    }

    public Quaternion GetGlobalRotation()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetGlobalRotation(this);
            return __ret;
        }
    }

    public bool TagIs(string tag)
    {
        unsafe {
            var _tag = tag;
            var __ret = fyrox_lite_lite_node_LiteNode_TagIs(this, _tag);
            return __ret;
        }
    }

    public void SetTag(string tag)
    {
        unsafe {
            var _tag = tag;
            fyrox_lite_lite_node_LiteNode_SetTag(this, _tag);
        }
    }

    public string GetTag()
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_node_LiteNode_GetTag(this);
            return __ret;
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial RigidBody_optional fyrox_lite_lite_node_LiteNode_AsRigidBody(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial string_result fyrox_lite_lite_node_LiteNode_GetName(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_GetAlive(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_Destroy(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector3 fyrox_lite_lite_node_LiteNode_GetGlobalPosition(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector3 fyrox_lite_lite_node_LiteNode_GetLocalPosition(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Quaternion fyrox_lite_lite_node_LiteNode_GetLocalRotation(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SendHierarchical(Node self, RoutingStrategy routing, UserScriptMessage payload);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SetLocalPosition(Node self, Vector3* new_pos);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SetLocalRotation(Node self, Quaternion* value);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SubscribeTo(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node_optional fyrox_lite_lite_node_LiteNode_FindColliderInChildren(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_GetValid(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node fyrox_lite_lite_node_LiteNode_GetParent(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial UserScript_result fyrox_lite_lite_node_LiteNode_AddScript(Node self, string class_name);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial UserScript_optional_result fyrox_lite_lite_node_LiteNode_FindScript(Node self, string class_name);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Quaternion fyrox_lite_lite_node_LiteNode_GetGlobalRotation(Node self);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_TagIs(Node self, string tag);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SetTag(Node self, string tag);

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial string fyrox_lite_lite_node_LiteNode_GetTag(Node self);
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