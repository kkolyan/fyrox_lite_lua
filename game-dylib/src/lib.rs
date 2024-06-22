//! Wrapper for hot-reloadable plugin.
use fyrox_no_lua::game::Plugin;
use fyrox_no_lua::game::Game;

#[no_mangle]
pub fn fyrox_plugin() -> Box<dyn Plugin> {
    Box::new(Game::default())
}
