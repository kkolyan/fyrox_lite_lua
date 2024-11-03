// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite.Internal;
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
using System.Numerics;
using System.Drawing;
using FyroxLite.Internal;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.Internal;


[StructLayout(LayoutKind.Sequential)]
internal unsafe partial struct NativeScriptAppFunctions {
    private IntPtr _load_scripts;
    internal FyroxNativeGlobal.LoadScripts load_scripts
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.LoadScripts>(_load_scripts);
        set => _load_scripts = Marshal.GetFunctionPointerForDelegate(value);
    }
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
    private IntPtr _set_property;
    internal FyroxNativeGlobal.SetProperty set_property
    {
        get => Marshal.GetDelegateForFunctionPointer<FyroxNativeGlobal.SetProperty>(_set_property);
        set => _set_property = Marshal.GetFunctionPointerForDelegate(value);
    }
}