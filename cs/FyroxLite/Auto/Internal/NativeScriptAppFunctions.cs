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


[StructLayout(LayoutKind.Sequential)]
internal unsafe partial struct NativeScriptAppFunctions {
    private IntPtr _on_init;
    internal FyroxNativeGlobal.NodeOnInit on_init
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.NodeOnInit>(_on_init);
        set => _on_init = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _on_start;
    internal FyroxNativeGlobal.NodeOnStart on_start
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.NodeOnStart>(_on_start);
        set => _on_start = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _on_deinit;
    internal FyroxNativeGlobal.NodeOnDeinit on_deinit
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.NodeOnDeinit>(_on_deinit);
        set => _on_deinit = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _on_update;
    internal FyroxNativeGlobal.NodeOnUpdate on_update
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.NodeOnUpdate>(_on_update);
        set => _on_update = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _on_message;
    internal FyroxNativeGlobal.NodeOnMessage on_message
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.NodeOnMessage>(_on_message);
        set => _on_message = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _on_game_init;
    internal FyroxNativeGlobal.GameOnInit on_game_init
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.GameOnInit>(_on_game_init);
        set => _on_game_init = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _on_game_update;
    internal FyroxNativeGlobal.GameOnUpdate on_game_update
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.GameOnUpdate>(_on_game_update);
        set => _on_game_update = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _create_script_instance;
    internal FyroxNativeGlobal.CreateScriptInstance create_script_instance
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.CreateScriptInstance>(_create_script_instance);
        set => _create_script_instance = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _dispose_message;
    internal FyroxNativeGlobal.DisposeMessage dispose_message
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.DisposeMessage>(_dispose_message);
        set => _dispose_message = Marshal.GetFunctionPointerForDelegate(value);
    }
    private IntPtr _dispose_script;
    internal FyroxNativeGlobal.DisposeScript dispose_script
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.DisposeScript>(_dispose_script);
        set => _dispose_script = Marshal.GetFunctionPointerForDelegate(value);
    }
}