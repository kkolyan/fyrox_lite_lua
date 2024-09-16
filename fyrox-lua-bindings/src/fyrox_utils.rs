use fyrox::script::PluginsRefMut;

use crate::plugin::LuaPlugin;

#[extend::ext]
pub impl PluginsRefMut<'_> {
    fn lua(&self) -> &LuaPlugin {
        self.get::<LuaPlugin>()
    }

    fn lua_mut(&mut self) -> &mut LuaPlugin {
        self.get_mut::<LuaPlugin>()
    }
}
