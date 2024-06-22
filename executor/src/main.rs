//! Executor with your game connected to it as a plugin.
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyrox::dpi::LogicalSize;
use fyrox::engine::executor::Executor;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;

fn main() {
    Log::set_verbosity(MessageKind::Warning);
    let mut window_attributes = WindowAttributes::default();
    window_attributes.inner_size = Some(LogicalSize::new(1280.0, 720.0).into());
    let mut executor = Executor::from_params(
        EventLoop::new().unwrap(),
        GraphicsContextParams {
            window_attributes,
            vsync: false,
            msaa_sample_count: None,
        },
    );

    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "game_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libgame_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libgame_dylib.dylib";
        executor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        use fyrox_no_lua::game::Game;
        executor.add_plugin(Game::default());
    }

    executor.run()
}
