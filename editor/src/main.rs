//! Editor with your game connected to it as a plugin.
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;

fn main() {
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
		use fyrox_lua::plugin::LuaPlugin;
        editor.add_game_plugin(LuaPlugin::default());
    }

    editor.run(event_loop)
}
