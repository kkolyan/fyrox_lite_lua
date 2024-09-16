use crate::lua_utils::log_error;
use crate::script::LuaScript;
use crate::script_data::ScriptData;
use crate::script_def::ScriptDefinition;
use crate::script_def::ScriptMetadata;
use crate::script_object::ScriptObject;
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
use mlua::Function;
use mlua::Lua;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::Arc;

#[derive(Visit, Reflect, Debug)]
pub struct LuaPlugin {
    #[visit(skip)]
    #[reflect(hidden)]
    pub vm: &'static Lua,

    #[visit(skip)]
    #[reflect(hidden)]
    pub failed: bool,
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
        }
    }
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

        for entry in WalkDir::new("scripts").into_iter().flatten() {
            load_script(&context, lua, &entry);
        }
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        Lua::new()
            .load("print('hello Fyrox'); print(_VERSION)")
            .eval::<()>()
            .unwrap();
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }
}

fn load_script(context: &PluginRegistrationContext, lua: &mut Lua, entry: &DirEntry) {
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
    let uuid = metadata.uuid;

    let assembly_name = "scripts/**.lua";
    let definition = Arc::new(ScriptDefinition {
        metadata,
        assembly_name,
    });

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
    Log::info(format!(
        "script registered: {}",
        entry.path().to_string_lossy()
    ));
}
