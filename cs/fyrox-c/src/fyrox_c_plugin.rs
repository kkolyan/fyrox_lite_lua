use crate::external_script_proxy::invoke_callback;
use crate::external_script_proxy::ExternalScriptProxy;
use crate::scripted_app::APP;
use fyrox::core::log::Log;
use fyrox::core::notify::EventKind;
use fyrox::core::reflect::prelude::*;
use fyrox::core::reflect::Reflect;
use fyrox::core::visitor::prelude::*;
use fyrox::core::visitor::Visit;
use fyrox::core::watcher::FileSystemWatcher;
use fyrox::event::Event;
use fyrox::plugin::DynamicPlugin;
use fyrox::plugin::Plugin;
use fyrox::plugin::PluginContext;
use fyrox::plugin::PluginRegistrationContext;
use fyrox::script::constructor::ScriptConstructor;
use fyrox::script::PluginsRefMut;
use fyrox::script::Script;
use fyrox::walkdir::WalkDir;
use fyrox_lite::script_metadata::ScriptDefinition;
use fyrox_lite::script_object::ScriptObject;
use fyrox_lite::script_object_residence::ScriptResidence;
use fyrox_lite::wrapper_reflect;
use std::cell::RefCell;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Visit, Reflect)]
pub struct CPlugin {
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

impl Debug for CPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternalScriptPlugin")
            .field("failed", &self.failed)
            .field("scripts", &self.scripts)
            .finish()
    }
}

impl CPlugin {
    pub fn with_hot_reload(hot_reload_enabled: bool) -> Self {
        Self::new("scripts", hot_reload_enabled)
    }

    pub fn new(scripts_dir: &str, hot_reload_enabled: bool) -> Self {
        todo!()
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

impl Default for CPlugin {
    fn default() -> Self {
        Self::new("scripts", true)
    }
}

impl Plugin for CPlugin {
    fn register(&self, context: PluginRegistrationContext) {
        APP.with_borrow(|app| {
            for (uuid, md) in app.as_ref().unwrap().scripts.iter() {
                let def = Arc::new(ScriptDefinition {
                    metadata: md.md.clone(),
                    assembly_name: self.assembly_name(),
                });
                let name = def.metadata.class.clone();
                context
                    .serialization_context
                    .script_constructors
                    .add_custom(
                        *uuid,
                        ScriptConstructor {
                            name: name.to_string(),
                            source_path: "",
                            assembly_name: self.assembly_name(),
                            constructor: Box::new(move || {
                                Script::new(ExternalScriptProxy {
                                    name: name.to_string(),
                                    data: ScriptResidence::Packed(ScriptObject::new(&def)),
                                })
                            }),
                        },
                    )
                    .unwrap();
            }
        });
        for entry in WalkDir::new(&self.scripts_dir).into_iter().flatten() {}
    }

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) {
        for script in self.scripts.borrow_mut().0.iter_mut() {
            script.data.ensure_unpacked(&mut self.failed);
            invoke_callback(&mut context, |app| {
                (app.functions.on_game_init)(script.data.inner_unpacked().unwrap().handle);
            });
        }
    }

    fn update(&mut self, context: &mut PluginContext) {
        for script in self.scripts.borrow_mut().0.iter_mut() {
            script.data.ensure_unpacked(&mut self.failed);
            invoke_callback(context, |app| {
                (app.functions.on_game_update)(script.data.inner_unpacked().unwrap().handle);
            });
        }
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {}
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
    fn lua(&self) -> &CPlugin {
        self.get::<CPlugin>()
    }

    fn lua_mut(&mut self) -> &mut CPlugin {
        self.get_mut::<CPlugin>()
    }
}

impl DynamicPlugin for CPlugin {
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
        Log::info(format!("reloading external C-compatible scripts"));
    }

    fn reload(
        &mut self,
        fill_and_register: &mut dyn FnMut(&mut dyn Plugin) -> Result<(), String>,
    ) -> Result<(), String> {
        self.scripts.borrow_mut().inner_mut().clear();
        let result = fill_and_register(self);

        result
    }
}