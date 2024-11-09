//! Editor with your game connected to it as a plugin.
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::plugin::EditorPlugin;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;
use crate::fyrox_c_plugin::CPlugin;

#[no_mangle]
pub extern "C" fn fyrox_lite_editor_run() {
    Log::set_verbosity(MessageKind::Information);
    let event_loop = EventLoop::new().unwrap();
    let mut editor = Editor::new(Some(StartupData {
        working_directory: Default::default(),
        scenes: vec!["data/scene.rgs".into()],
    }));

    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "fyrox-lua_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libfyrox-lua_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libfyrox-lua_dylib.dylib";
        editor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        if let Err(err) = editor.add_dynamic_plugin_custom(CPlugin::with_hot_reload(true)) {
            Log::err(err);
        }

        // editor.add_editor_plugin(LuaPluginRefreshOnFocus);
    }

    editor.run(event_loop)
}

// #[cfg(not(feature = "dylib"))]
// struct LuaPluginRefreshOnFocus;

// #[cfg(not(feature = "dylib"))]
// impl EditorPlugin for LuaPluginRefreshOnFocus {
//
//     fn on_resumed(&mut self, #[allow(unused_variables)] editor: &mut Editor) {
//         for it in editor.engine.plugins_mut() {
//             if let Some(it) = it.cast_mut::<fyrox_lua::LuaPlugin>() {
//                 it.check_for_script_changes();
//             }
//         }
//     }
// }