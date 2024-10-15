//! Editor with your game connected to it as a plugin.
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;

fn main() {
    Log::set_verbosity(MessageKind::Warning);
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
		use fyrox_lua::fyrox_plugin::LuaPlugin;
        if let Err(err) = editor.add_dynamic_plugin_custom(LuaPlugin::default()) {
            Log::err(err);
        }
    }

    editor.run(event_loop)
}
