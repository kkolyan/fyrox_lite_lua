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
namespace FyroxLite.LitePlugin;

// fyrox_lite::lite_plugin::LitePlugin
[StructLayout(LayoutKind.Sequential)]
public readonly partial struct Plugin
{

    public static T Get<T>() where T : class
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_plugin_LitePlugin_Get(typeof(T).Name);
            return UserScript_result.ToFacade(__ret) as T;
        }
    }
    
    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial UserScript_result fyrox_lite_lite_plugin_LitePlugin_Get(string class_name);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Plugin_optional
{
    internal Plugin Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Plugin? ToFacade(in Plugin_optional value)
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
    public static Plugin_optional FromFacade(in Plugin? value)
    {
        if (value == null)
        {
            return new Plugin_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new Plugin_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}