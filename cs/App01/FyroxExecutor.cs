using System.Numerics;
using System.Reflection;
using System.Runtime.InteropServices;
using App01;
using FyroxLite.Internal;
using FyroxLite.LiteNode;
using FyroxLite.LiteUi;

namespace FyroxLite;

public partial class FyroxExecutor
{
    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", EntryPoint = "fyrox_lite_executor_run",
        SetLastError = true)]
    private static partial void RunInternal();

    public static void Run()
    {
        List<NativeScriptMetadata> scripts = new();
        foreach (var assembly in AppDomain.CurrentDomain.GetAssemblies())
        {
            foreach (var type in assembly.GetTypes())
            {
                if (!type.IsClass || type.IsAbstract)
                {
                    continue;
                }
                if (type.IsAssignableTo(typeof(NodeScript)))
                {
                    var uuidAttr = type.GetCustomAttribute<UuidAttribute>();
                    if (uuidAttr == null)
                    {
                        throw new Exception($"Invalid script {type}: [Uuid] attribute required for Node scripts");
                    }

                    var properties = new List<NativeScriptProperty>();

                    foreach (var field in type.GetFields(BindingFlags.Instance))
                    {
                        properties.Add(new NativeScriptProperty
                        {
                            id = properties.Count,
                            name = NativeString.FromFacade(field.Name),
                            ty = ExtractFieldType(field.FieldType),
                            hide_in_inspector = NativeBool.FromFacade(field.GetCustomAttribute<HideInInspectorAttribute>() != null),
                            transient = NativeBool.FromFacade(field.GetCustomAttribute<TransientAttribute>() != null),
                        });
                    }

                    var metadata = new NativeScriptMetadata
                    {
                        id = new NativeClassId(scripts.Count + 1),
                        uuid = NativeString.FromFacade(uuidAttr.Uuid.ToString()),
                        kind = NativeScriptKind.Node,
                        name = NativeString.FromFacade(type.Name),
                        has_global_on_init = HasDeclaredMethod(type, nameof(GlobalScript.OnGlobalInit), []),
                        has_global_on_update = HasDeclaredMethod(type, nameof(GlobalScript.OnGlobalUpdate), []),
                        has_node_on_init = HasDeclaredMethod(type, nameof(NodeScript.OnInit), []),
                        has_node_on_start = HasDeclaredMethod(type, nameof(NodeScript.OnStart), []),
                        has_node_on_deinit = HasDeclaredMethod(type, nameof(NodeScript.OnDeinit), []),
                        has_node_on_update = HasDeclaredMethod(type, nameof(NodeScript.OnUpdate), [typeof(float)]),
                        has_node_on_message = HasDeclaredMethod(type, nameof(NodeScript.OnMessage), [typeof(object)]),
                        properties = NativeScriptProperty_slice.FromFacade(properties),
                    };
                    scripts.Add(metadata);
                }
            }
        }

        FyroxC.FyroxHello();
        unsafe
        {
            FyroxNativeGlobal.init_fyrox(new NativeScriptedApp
            {
                scripts = NativeScriptMetadata_slice.FromFacade(scripts),
                functions = new NativeScriptAppFunctions
                {
                    load_scripts = FyroxImpls.load_scripts,
                    on_init = FyroxImpls.on_init,
                    on_start = FyroxImpls.on_start,
                    on_deinit = FyroxImpls.on_deinit,
                    on_update = FyroxImpls.on_update,
                    on_message = FyroxImpls.on_message,
                    on_game_init = FyroxImpls.on_game_init,
                    on_game_update = FyroxImpls.on_game_update,
                    create_script_instance = FyroxImpls.create_script_instance,
                    set_property = FyroxImpls.set_property,
                },
            });
        }

        RunInternal();
    }

    private static NativeValueType ExtractFieldType(Type type)
    {
        if (type == typeof(bool)) return NativeValueType.@bool;
        if (type == typeof(float)) return NativeValueType.f32;
        if (type == typeof(double)) return NativeValueType.f64;
        if (type == typeof(short)) return NativeValueType.i16;
        if (type == typeof(int)) return NativeValueType.i32;
        if (type == typeof(long)) return NativeValueType.i64;
        if (type == typeof(string)) return NativeValueType.String;
        if (type == typeof(Vector3)) return NativeValueType.Vector3;
        if (type == typeof(Quaternion)) return NativeValueType.Quaternion;
        if (type == typeof(Node)) return NativeValueType.Node;
        if (type == typeof(UiNode)) return NativeValueType.UiNode;
        throw new Exception($"ERROR [FyroxLite] Unsupported field type: {type}");
    }

    private static NativeBool HasDeclaredMethod(Type type, string name, Type[] paramTypes)
    {
        return NativeBool.FromFacade(type.GetMethod(name,
            BindingFlags.DeclaredOnly | BindingFlags.Instance | BindingFlags.Public,
            paramTypes) != null);
    }
}