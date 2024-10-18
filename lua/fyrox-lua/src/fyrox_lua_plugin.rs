use crate::external_script_proxy::ExternalScriptProxy;
use crate::lua_lifecycle::create_plugin;
use crate::lua_lifecycle::invoke_callback;
use crate::lua_lifecycle::load_script;
use crate::lua_lifecycle::lua_vm;
use fyrox::core::log::Log;
use fyrox::core::notify::EventKind;
use fyrox::core::reflect::prelude::*;
use fyrox::core::reflect::Reflect;
use fyrox::core::visitor::prelude::*;
use fyrox::core::visitor::Visit;
use fyrox::core::watcher::FileSystemWatcher;
use fyrox::event::Event;
use fyrox::event::WindowEvent;
use fyrox::plugin::DynamicPlugin;
use fyrox::plugin::Plugin;
use fyrox::plugin::PluginContext;
use fyrox::plugin::PluginRegistrationContext;
use fyrox::script::PluginsRefMut;
use fyrox::walkdir::WalkDir;
use fyrox_lite::wrapper_reflect;
use mlua::Value;
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Visit, Reflect)]
pub struct LuaPlugin {

    #[visit(skip)]
    #[reflect(hidden)]
    pub failed: bool,

    #[visit(skip)]
    #[reflect(hidden)]
    pub scripts_dir: String,

    #[visit(skip)]
    #[reflect(hidden)]
    pub need_reload: bool,

    pub scripts: RefCell<PluginScriptList>,

    #[visit(skip)]
    #[reflect(hidden)]
    pub hot_reload: HotReload,
}

pub enum HotReload {
    Disabled,
    Enabled { watcher: FileSystemWatcher },
}

impl Debug for LuaPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LuaPlugin")
            .field("failed", &self.failed)
            .field("scripts", &self.scripts)
            .finish()
    }
}

impl LuaPlugin {
    pub fn with_hot_reload(hot_reload_enabled: bool) -> Self {
        Self::new("scripts", hot_reload_enabled)
    }

    pub fn new(scripts_dir: &str, hot_reload_enabled: bool) -> Self {
        create_plugin(scripts_dir, hot_reload_enabled)
    }

    pub fn check_for_script_changes(&mut self) {
        let HotReload::Enabled { watcher } = &self.hot_reload else {
            return;
        };
        let mut reload = false;
        while let Some(event) = watcher.try_get_event() {
            Log::info(format!("FS watcher event: {event:?}"));
            match &event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    reload = true;
                }
                _ => {}
            }
        }
        if reload {
            self.need_reload = true;
        }
    }
}

impl Default for LuaPlugin {
    fn default() -> Self {
        Self::new("scripts", true)
    }
}

impl Plugin for LuaPlugin {
    fn register(&self, context: PluginRegistrationContext) {
        for entry in WalkDir::new(&self.scripts_dir).into_iter().flatten() {
            load_script(
                &context,
                &entry,
                &mut self.scripts.borrow_mut(),
                self.assembly_name(),
            );
        }
    }

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) {
        for script in self.scripts.borrow_mut().0.iter_mut() {
            invoke_callback(&mut script.data,&mut context, "init", || {
                if let Some(scene_path) = scene_path {
                    Ok(Value::String(lua_vm().create_string(scene_path)?))
                } else {
                    Ok(Value::Nil)
                }
            });
        }
    }

    fn update(&mut self, context: &mut PluginContext) {
        for script in self.scripts.borrow_mut().0.iter_mut() {
            invoke_callback(&mut script.data, context, "update", || Ok(()));
        }
    }

    fn on_os_event(&mut self, event: &Event<()>, _context: PluginContext) {
        if let Event::WindowEvent {
            event: WindowEvent::Focused(true),
            ..
        } = event
        {
            Log::info(format!(
                "Window received focus. Checking for Lua script updates..."
            ));
            self.check_for_script_changes();
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PluginScriptList(Vec<ExternalScriptProxy>);

impl PluginScriptList {
    pub fn inner(&self) -> &Vec<ExternalScriptProxy> {
        &self.0
    }
    pub fn inner_mut(&mut self) -> &mut Vec<ExternalScriptProxy> {
        &mut self.0
    }
}

impl Visit for PluginScriptList {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut guard = visitor.enter_region(name)?;

        for item in self.inner_mut().iter_mut() {
            item.data.visit(item.name.as_str(), &mut guard)?;
        }
        Ok(())
    }
}

impl Reflect for PluginScriptList {
    wrapper_reflect! {0}

    fn source_path() -> &'static str
    where
        Self: Sized,
    {
        file!()
    }

    fn assembly_name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn type_assembly_name() -> &'static str
    where
        Self: Sized,
    {
        env!("CARGO_PKG_NAME")
    }
}

#[extend::ext]
pub impl PluginsRefMut<'_> {
    fn lua(&self) -> &LuaPlugin {
        self.get::<LuaPlugin>()
    }

    fn lua_mut(&mut self) -> &mut LuaPlugin {
        self.get_mut::<LuaPlugin>()
    }
}

impl DynamicPlugin for LuaPlugin {
    fn display_name(&self) -> String {
        format!("Lua Plugin (scripts path: {})", self.scripts_dir)
    }

    fn is_reload_needed_now(&self) -> bool {
        self.need_reload
    }

    fn as_loaded_ref(&self) -> &dyn Plugin {
        self
    }

    fn as_loaded_mut(&mut self) -> &mut dyn Plugin {
        self
    }

    fn is_loaded(&self) -> bool {
        true
    }

    fn prepare_to_reload(&mut self) {
        self.need_reload = false;
        Log::info(format!("reloading Lua scripts"));

        lua_vm()
            .load(
                "
                PINS_BACKUP = {}
                for k, v in pairs(PINS) do
                    PINS_BACKUP[k] = v
                end
            ",
            )
            .eval::<()>()
            .unwrap();
    }

    fn reload(
        &mut self,
        fill_and_register: &mut dyn FnMut(&mut dyn Plugin) -> Result<(), String>,
    ) -> Result<(), String> {
        self.scripts.borrow_mut().inner_mut().clear();
        let result = fill_and_register(self);

        lua_vm()
            .load(
                "
                for k, v in pairs(PINS_BACKUP) do
                    PINS[k] = v
                end
                PINS_BACKUP = nil
            ",
            )
            .eval::<()>()
            .unwrap();

        result
    }
}
