//! Wrapper for hot-reloadable plugin.
use fyrox_no_lua::game::Game;
use fyrox_no_lua::game::Plugin;

#[no_mangle]
pub fn fyrox_plugin() -> Box<dyn Plugin> {
    Box::new(Game::default())
}
