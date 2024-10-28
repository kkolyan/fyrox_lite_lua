using FyroxLite.LiteWindow;
using FyroxLite.LiteInput;
using FyroxLite.LiteMath;
using FyroxLite.LiteLog;
using FyroxLite.LitePrefab;
using FyroxLite.LiteUi;
using FyroxLite.LitePlugin;
using FyroxLite.LitePhysics;
using FyroxLite.LiteNode;
using FyroxLite.LiteScene;
using System.Runtime.InteropServices;
namespace FyroxLite.LiteNode;

// fyrox_lite::lite_node::LiteNode
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Node
{

    public RigidBody? AsRigidBody()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_AsRigidBody(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial RigidBody? fyrox_lite_lite_node_LiteNode_AsRigidBody(Node* self, );

    public string? GetName()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetName(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial string? fyrox_lite_lite_node_LiteNode_GetName(Node* self, );

    public bool GetAlive()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetAlive(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_GetAlive(Node* self, );

    public void Destroy()
    {
        unsafe {
            fixed (Node* self = &this) fyrox_lite_lite_node_LiteNode_Destroy(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_Destroy(Node* self, );

    public Vector3 GetGlobalPosition()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetGlobalPosition(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector3 fyrox_lite_lite_node_LiteNode_GetGlobalPosition(Node* self, );

    public Vector3 GetLocalPosition()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetLocalPosition(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Vector3 fyrox_lite_lite_node_LiteNode_GetLocalPosition(Node* self, );

    public Quaternion GetLocalRotation()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetLocalRotation(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Quaternion fyrox_lite_lite_node_LiteNode_GetLocalRotation(Node* self, );

    public void SendHierarchical(RoutingStrategy routing,T payload)
    {
        unsafe {
            fixed (Node* self = &this) fyrox_lite_lite_node_LiteNode_SendHierarchical(self, routing,payload);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SendHierarchical(Node* self, RoutingStrategy routing,T payload);

    public void SetLocalPosition(Vector3 new_pos)
    {
        unsafe {
            fixed (Node* self = &this) fyrox_lite_lite_node_LiteNode_SetLocalPosition(self, new_pos);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SetLocalPosition(Node* self, Vector3 new_pos);

    public void SetLocalRotation(Quaternion value)
    {
        unsafe {
            fixed (Node* self = &this) fyrox_lite_lite_node_LiteNode_SetLocalRotation(self, value);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SetLocalRotation(Node* self, Quaternion value);

    public void SubscribeTo()
    {
        unsafe {
            fixed (Node* self = &this) fyrox_lite_lite_node_LiteNode_SubscribeTo(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SubscribeTo(Node* self, );

    public Node? FindColliderInChildren()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_FindColliderInChildren(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node? fyrox_lite_lite_node_LiteNode_FindColliderInChildren(Node* self, );

    public bool GetValid()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetValid(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_GetValid(Node* self, );

    public Node GetParent()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetParent(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Node fyrox_lite_lite_node_LiteNode_GetParent(Node* self, );

    public T AddScript<T>()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_AddScript(self, typeof(T).Name);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial T fyrox_lite_lite_node_LiteNode_AddScript(Node* self, string class_name);

    public T? FindScript<T>()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_FindScript(self, typeof(T).Name);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial T? fyrox_lite_lite_node_LiteNode_FindScript(Node* self, string class_name);

    public Quaternion GetGlobalRotation()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetGlobalRotation(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Quaternion fyrox_lite_lite_node_LiteNode_GetGlobalRotation(Node* self, );

    public bool TagIs(string tag)
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_TagIs(self, tag);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool fyrox_lite_lite_node_LiteNode_TagIs(Node* self, string tag);

    public void SetTag(string tag)
    {
        unsafe {
            fixed (Node* self = &this) fyrox_lite_lite_node_LiteNode_SetTag(self, tag);
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_node_LiteNode_SetTag(Node* self, string tag);

    public string GetTag()
    {
        unsafe {
            fixed (Node* self = &this) return fyrox_lite_lite_node_LiteNode_GetTag(self, );
        }
    }
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial string fyrox_lite_lite_node_LiteNode_GetTag(Node* self, );
}