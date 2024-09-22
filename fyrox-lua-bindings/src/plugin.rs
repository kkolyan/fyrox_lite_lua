use crate::lua_utils::log_error;
use crate::script::LuaScript;
use crate::script_class::ScriptClass;
use crate::script_data::ScriptData;
use crate::script_def::ScriptDefinition;
use crate::script_def::ScriptKind;
use crate::script_def::ScriptMetadata;
use crate::script_object::ScriptObject;
use crate::typed_userdata::TypedUserData;
use fyrox::core::log::Log;
use fyrox::core::pool::Handle;
use fyrox::core::reflect::prelude::*;
use fyrox::core::reflect::Reflect;
use fyrox::core::visitor::prelude::*;
use fyrox::core::visitor::Visit;
use fyrox::event::Event;
use fyrox::gui::message::UiMessage;
use fyrox::plugin::Plugin;
use fyrox::plugin::PluginContext;
use fyrox::plugin::PluginRegistrationContext;
use fyrox::scene::Scene;
use fyrox::script::constructor::ScriptConstructor;
use fyrox::script::Script;
use fyrox::script::ScriptContext;
use fyrox::walkdir::DirEntry;
use fyrox::walkdir::WalkDir;
use fyrox_lite_api::lite_scene::load_scene_async;
use fyrox_lite_api::wrapper_reflect;
use mlua::Function;
use mlua::Lua;
use mlua::Table;
use mlua::Value;
use send_wrapper::SendWrapper;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Visit, Reflect, Debug)]
pub struct LuaPlugin {
    #[visit(skip)]
    #[reflect(hidden)]
    pub vm: &'static Lua,

    #[visit(skip)]
    #[reflect(hidden)]
    pub failed: bool,

    scripts: RefCell<PluginScriptList>,
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
        LuaPlugin {
            vm: Box::leak(Box::new(Lua::new())),
            failed: false,
            scripts: Default::default(),
        }
    }
}

thread_local! {
    static LOADING_CLASS_NAME: RefCell<Option<&'static str>> = Default::default();
}
impl Plugin for LuaPlugin {
    fn register(&self, context: PluginRegistrationContext) {
        // mlua has approach with lifetimes that makes very difficult storing Lua types
        // here and there in Rust. But we need a single Lua VM instance for the whole life
        // of game process, so that's ok to make it 'static.
        let lua: &'static mut Lua = Box::leak(Box::new(Lua::new()));

        log_error(
            "set 'package.path'",
            lua.load("package.path = 'scripts/?.lua;scripts/?/init.lua'")
                .eval::<()>(),
        );

        let classes: Rc<RefCell<HashMap<&'static str, Table<'static>>>> = Default::default();

        {
            let classes = classes.clone();
            lua.globals()
                .set(
                    "script_class",
                    lua.create_function(move |lua, _args: ()| {
                        LOADING_CLASS_NAME.with(|class_name| {
                            let class_name = class_name
                                .borrow()
                                .expect("script_class() called out of permitted context");
                            let table = lua.create_table()?;

                            classes
                                .as_ref()
                                .borrow_mut()
                                .insert(class_name, table.clone());

                            Ok(ScriptClass {
                                name: class_name,
                                table: Default::default(),
                            })
                        })
                    })
                    .unwrap(),
                )
                .unwrap();
        }
        lua.globals()
            .set(
                "load_scene_async",
                lua.create_function_mut(|lua, scene_path: mlua::String| {
                    load_scene_async(scene_path.to_str()?);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        for entry in WalkDir::new("scripts").into_iter().flatten() {
            load_script(
                &context,
                lua,
                &entry,
                classes.clone(),
                &mut self.scripts.borrow_mut(),
            );
        }
    }

    fn init(&mut self, scene_path: Option<&str>, mut context: PluginContext) {
        Lua::new()
            .load("print('hello Fyrox'); print(_VERSION)")
            .eval::<()>()
            .unwrap();
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        for script in self.scripts.borrow_mut().0.iter_mut() {
            script.invoke_callback(&mut context, "init", |lua| {
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
            script.invoke_callback(context, "update", |_lua| Ok(()));
        }
    }
}

fn load_script(
    context: &PluginRegistrationContext,
    lua: &'static Lua,
    entry: &DirEntry,
    class_data: Rc<RefCell<HashMap<&'static str, Table<'static>>>>,
    plugin_scripts: &mut PluginScriptList,
) {
    if !entry.file_type().is_file() {
        return;
    }
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

    LOADING_CLASS_NAME.with(|class_name| {
        *class_name.borrow_mut() = Some(metadata.class);
        lua.globals()
            .get::<_, Function>("require")
            .unwrap()
            .call::<&str, ()>(metadata.class)
            .unwrap();
        *class_name.borrow_mut() = None;
    });

    let class_loading = lua
        .load("return function(x) require(x) end")
        .eval::<Function>()
        .and_then(|it| it.call::<_, ()>(metadata.class));
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

    let assembly_name = "scripts/**.lua";
    let definition = Arc::new(ScriptDefinition {
        metadata,
        class_data: SendWrapper::new(class_data.as_ref().borrow().get(name).unwrap().clone()),
        assembly_name,
    });

    match definition.metadata.kind {
        ScriptKind::Script(uuid) => {
            context
                .serialization_context
                .script_constructors
                .add_custom(
                    uuid,
                    ScriptConstructor {
                        constructor: Box::new(move || {
                            Script::new(LuaScript {
                                data: ScriptData::Packed(ScriptObject::new(&definition)),
                                name: name.to_string(),
                            })
                        }),
                        name: name.to_string(),
                        source_path: entry.path().to_string_lossy().to_string().leak(),
                        assembly_name,
                    },
                );
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
struct PluginScriptList(Vec<LuaScript>);

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
