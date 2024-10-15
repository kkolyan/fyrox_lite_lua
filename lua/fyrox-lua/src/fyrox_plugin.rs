use crate::debug::override_print;
use crate::debug::var_dump;
use crate::fyrox_script::invoke_callback;
use crate::fyrox_script::LuaScript;
use crate::generated::registry::register_classes;
use crate::lua_utils::log_error;
use crate::script_class::ScriptClass;
use crate::script_data::ScriptData;
use crate::script_def::ScriptDefinition;
use crate::script_def::ScriptKind;
use crate::script_def::ScriptMetadata;
use crate::script_object::ScriptObject;
use crate::typed_userdata::TypedUserData;
use fyrox::core::log::Log;
use fyrox::core::notify::EventKind;
use fyrox::core::reflect::prelude::*;
use fyrox::core::reflect::Reflect;
use fyrox::core::visitor::prelude::*;
use fyrox::core::visitor::Visit;
use fyrox::core::watcher::FileSystemWatcher;
use fyrox::plugin::DynamicPlugin;
use fyrox::plugin::Plugin;
use fyrox::plugin::PluginContext;
use fyrox::plugin::PluginRegistrationContext;
use fyrox::script::constructor::ScriptConstructor;
use fyrox::script::PluginsRefMut;
use fyrox::script::Script;
use fyrox::script::ScriptContext;
use fyrox::walkdir::DirEntry;
use fyrox::walkdir::WalkDir;
use fyrox_lite::wrapper_reflect;
use mlua::Function;
use mlua::Lua;
use mlua::UserDataRefMut;
use mlua::Value;
use send_wrapper::SendWrapper;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;

#[derive(Visit, Reflect)]
pub struct LuaPlugin {
    #[visit(skip)]
    #[reflect(hidden)]
    pub vm: &'static Lua,

    #[visit(skip)]
    #[reflect(hidden)]
    pub failed: bool,

    #[visit(skip)]
    #[reflect(hidden)]
    pub watcher: FileSystemWatcher,

    #[visit(skip)]
    #[reflect(hidden)]
    pub scripts_dir: String,

    pub scripts: RefCell<PluginScriptList>,
}

impl Debug for LuaPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LuaPlugin")
            .field("vm", &self.vm)
            .field("failed", &self.failed)
            .field("scripts", &self.scripts)
            .finish()
    }
}

impl LuaPlugin {
    pub fn get<'r, 'a>(sc: &'r ScriptContext<'a, '_, '_>) -> &'a LuaPlugin
    where
        'r: 'a,
    {
        return sc.plugins.get::<LuaPlugin>();
    }

    pub fn get_mut<'r, 'a, 'b>(sc: &'r mut ScriptContext<'a, '_, '_>) -> Mut<'b, LuaPlugin>
    where
        'r: 'b,
        'r: 'a,
    {
        return Mut(sc.plugins.get_mut::<LuaPlugin>());
    }
}

pub struct Mut<'a, T>(&'a mut T);

impl<'a, T> Deref for Mut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T> DerefMut for Mut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl Default for LuaPlugin {
    fn default() -> Self {
        Self::new("scripts")
    }
}

thread_local! {
    pub static LUA: RefCell<Option<&'static mlua::Lua>> = RefCell::new(None);
}

impl LuaPlugin {
    pub fn new(scripts_dir: &str) -> Self {
        let vm = Box::leak(Box::new(Lua::new()));
        LUA.set(Some(vm));
        let lua_version = vm.load("return _VERSION").eval::<mlua::String>().unwrap();
        println!("Lua Version: {}", lua_version.to_str().unwrap_or("unknown"));
        override_print(vm);

        vm.globals()
            .set("PINS", vm.create_table().unwrap())
            .unwrap();

        log_error(
            "set 'package.path'",
            vm.load(format!(
                "package.path = '{}/?.lua;{}/?/init.lua'",
                scripts_dir, scripts_dir
            ))
            .eval::<()>(),
        );

        {
            vm.globals()
                .set(
                    "script_class",
                    vm.create_function(move |_lua, _args: ()| {
                        LOADING_CLASS_NAME.with(|class_name| {
                            let class_name = class_name
                                .borrow()
                                .expect("script_class() called out of permitted context");

                            Ok(ScriptClass {
                                name: class_name,
                                table: Default::default(),
                                def: Default::default(),
                            })
                        })
                    })
                    .unwrap(),
                )
                .unwrap();
        }

        vm.globals()
            .set("var_dump", vm.create_function(var_dump).unwrap())
            .unwrap();

        register_classes(vm);

        LuaPlugin {
            // mlua has approach with lifetimes that makes very difficult storing Lua types
            // here and there in Rust. But we need a single Lua VM instance for the whole life
            // of game process, so that's ok to make it 'static.
            vm,
            failed: false,
            scripts_dir: scripts_dir.into(),
            watcher: FileSystemWatcher::new(scripts_dir, Duration::from_millis(500)).unwrap(),
            scripts: Default::default(),
        }
    }
}

thread_local! {
    static LOADING_CLASS_NAME: RefCell<Option<&'static str>> = Default::default();
}
impl Plugin for LuaPlugin {
    fn register(&self, context: PluginRegistrationContext) {
        for entry in WalkDir::new(&self.scripts_dir).into_iter().flatten() {
            load_script(
                &context,
                self.vm,
                &entry,
                &mut self.scripts.borrow_mut(),
                self.assembly_name(),
            );
        }
    }

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) {
        for script in self.scripts.borrow_mut().0.iter_mut() {
            invoke_callback(&mut script.data, self.vm, &mut context, "init", |lua| {
                if let Some(scene_path) = scene_path {
                    Ok(Value::String(lua.create_string(scene_path)?))
                } else {
                    Ok(Value::Nil)
                }
            });
        }
    }

    fn update(&mut self, context: &mut PluginContext) {
        for script in self.scripts.borrow_mut().0.iter_mut() {
            invoke_callback(&mut script.data, self.vm, context, "update", |_lua| Ok(()));
        }
    }
}

fn load_script(
    context: &PluginRegistrationContext,
    lua: &'static Lua,
    entry: &DirEntry,
    plugin_scripts: &mut PluginScriptList,
    assembly_name: &'static str,
) {
    if !entry.file_type().is_file() {
        return;
    }

    println!("loading Lua script {:?}", entry.path());

    let metadata = ScriptMetadata::parse_file(entry.path());
    let metadata = match metadata {
        Ok(it) => it,
        Err(errs) => {
            for err in errs {
                Log::err(format!(
                    "failed to load script from file {}: {}",
                    &entry.path().to_string_lossy(),
                    err
                ));
            }
            return;
        }
    };

    let class_loading: mlua::Result<()> = LOADING_CLASS_NAME.with(|class_name| {
        *class_name.borrow_mut() = Some(metadata.class);

        lua.load(
            "
                return function(class) 
                    package.loaded[class] = nil
                    require(class)
                end",
        )
        .eval::<Function>()
        .and_then(|it| it.call::<_, ()>(metadata.class))?;

        *class_name.borrow_mut() = None;
        Ok(())
    });

    match class_loading {
        Ok(_) => {}
        Err(err) => {
            Log::err(format!(
                "Failed to load Lua class {:?}: {}",
                &metadata.class, err
            ));
            return;
        }
    }

    let name = metadata.class;

    let definition = Arc::new(ScriptDefinition {
        metadata,
        assembly_name,
    });

    let class = lua
        .globals()
        .get::<_, Option<UserDataRefMut<ScriptClass>>>(definition.metadata.class)
        .unwrap();
    let Some(mut class) = class else {
        Log::err(format!("invalid class file: {:?}", entry.path()));
        return;
    };

    class.def = Some(definition.clone());

    match definition.metadata.kind {
        ScriptKind::Script(uuid) => {
            let addition_result = context
                .serialization_context
                .script_constructors
                .add_custom(
                    uuid,
                    ScriptConstructor {
                        constructor: Box::new(move || {
                            Script::new(LuaScript {
                                data: ScriptData::Packed(ScriptObject::new(&definition)),
                                name: definition.metadata.class.to_string(),
                            })
                        }),
                        name: name.to_string(),
                        source_path: entry.path().to_string_lossy().to_string().leak(),
                        assembly_name,
                    },
                );
            if let Err(err) = addition_result {
                Log::err(err.to_string().as_str());
            }
        }
        ScriptKind::Plugin => {
            plugin_scripts.inner_mut().push(LuaScript {
                name: name.to_string(),
                data: ScriptData::Unpacked(SendWrapper::new(TypedUserData::new(
                    lua.create_userdata(ScriptObject::new(&definition)).unwrap(),
                ))),
            });
        }
    }

    Log::info(format!(
        "script registered: {}",
        entry.path().to_string_lossy()
    ));
}

#[derive(Debug, Default, Clone)]
pub struct PluginScriptList(Vec<LuaScript>);

impl PluginScriptList {
    pub fn inner(&self) -> &Vec<LuaScript> {
        &self.0
    }
    pub fn inner_mut(&mut self) -> &mut Vec<LuaScript> {
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
        let mut reload = false;
        while let Some(event) = self.watcher.try_get_event() {
            Log::info(format!("FS watcher event: {event:?}"));
            match &event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    reload = true;
                }
                _ => {}
            }
        }
        reload
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

    // fn prepare_to_reload(&self) {
    //     Log::info(format!("reloading Lua scripts"));

    //     self.vm
    //         .load(
    //             "
    //             PINS_BACKUP = {}
    //             for k, v in pairs(PINS) do
    //                 PINS_BACKUP[k] = v
    //             end
    //         ",
    //         )
    //         .eval::<()>()
    //         .unwrap();
    // }

    fn reload(
        &mut self,
        fill_and_register: &mut dyn FnMut(&mut dyn Plugin) -> Result<(), String>,
    ) -> Result<(), String> {

        self.scripts.borrow_mut().inner_mut().clear();
        let result = fill_and_register(self);

        // self.vm
        //     .load(
        //         "
        //         for k, v in pairs(PINS_BACKUP) do
        //             PINS[k] = v
        //         end
        //         PINS_BACKUP = nil
        //     ",
        //     )
        //     .eval::<()>()
        //     .unwrap();

        result
    }
}
