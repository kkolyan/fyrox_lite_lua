#![allow(unused_variables)]

use std::{borrow::Borrow, mem};

use crate::{
    debug::VerboseLuaValue,
    fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
    fyrox_utils::PluginsRefMut_Ext,
    lua_error,
    script::LuaScript,
    script_class::ScriptClass,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};
use fyrox::core::{
        algebra::{UnitQuaternion, Vector3},
        log::Log,
    };
use fyrox_lite::{
    lite_node::{LiteNode, LiteRoutingStrategy},
    lite_physics::{LitePhysics, LiteRayCastOptions, LiteRigidBody},
    lite_prefab::LitePrefab,
    lite_scene::LiteScene,
    lite_ui::{Brush, Color, LiteText, LiteUiNode},
    lite_window::{LiteCursorGrabMode, LiteWindow},
    script_context::with_script_context,
    spi::UserScript,
    LiteDataType,
};
use fyrox_lite_math::{LiteQuaternion, LiteVector3};
use mlua::{
    AnyUserData, Lua, MetaMethod, MultiValue, Table, UserDataFields, UserDataMethods, UserDataRef,
    UserDataRefMut, Value,
};
use send_wrapper::SendWrapper;

type UserScriptImpl<'a> = TypedUserData<'a, ScriptObject>;

impl FyroxUserData for LiteRigidBody {
    const CLASS_NAME: &'static str = "RigidBody";

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_method_mut(
            "apply_force",
            |lua, this, force: UserDataRef<Traitor<LiteVector3>>| {
                let force: Vector3<f32> = force.inner().0;
                this.apply_force(force.into());
                Ok(())
            },
        );
    }
}


impl Traitor<LiteNode> {
    fn get_script(&self, class_name: &str) -> Option<TypedUserData<'static, ScriptObject>> {
        self.find_script(class_name.to_string(), ())
    }
}

impl FyroxUserData for LiteScene {
    const CLASS_NAME: &'static str = "Scene";

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method_mut("load_async", |lua, this, scene_path: mlua::String| {
            LiteScene::load_async(scene_path.to_str()?.to_string());
            Ok(())
        });
    }
}

impl FyroxUserData for LiteText {
    const CLASS_NAME: &'static str = "Text";
    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method_mut("new", |lua, this, state: Table| {
            let text = fyrox_lite::lite_ui::TextBuilder {
                foregound: state.get::<_, Option<UserDataRef<Traitor<Brush>>>>("foreground")?.map(|it| it.inner().clone()),
                font_size: state.get::<_, Option<f32>>("font_size")?,
            };
            Ok(Traitor::new(LiteText::new(text)))
        });
    }

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_method_mut("set_text_async", |lua, this, text: mlua::String| {
            this.inner_mut().set_text_async(text.to_str()?.to_string());
            Ok(())
        });
    }
}

impl FyroxUserData for Brush {
    const CLASS_NAME: &'static str = "Brush";

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method("solid", |lua, cls, color: UserDataRef<Color>| {
            Ok(Traitor::new(Brush::Solid(*color)))
        });
    }
}

impl FyroxUserData for Color {
    const CLASS_NAME: &'static str = "Color";

    fn add_class_fields<'lua, F: UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field("BLACK", Traitor::new(Color {r: 0, g: 0, b: 0, a: 1}));
    }
}

impl FyroxUserData for LiteWindow {
    const CLASS_NAME: &'static str = "Window";

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method_mut(
            "set_cursor_grab",
            |lua, cls, mode: UserDataRef<Traitor<LiteCursorGrabMode>>| {
                LiteWindow::set_cursor_grab(*mode.borrow().inner());
                Ok(())
            },
        );
    }
}

impl FyroxUserData for LiteCursorGrabMode {
    const CLASS_NAME: &'static str = "CursorGrabMode";

    fn add_class_fields<'lua, F: UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field("None", Traitor::new(LiteCursorGrabMode::None));
        fields.add_field("Locked", Traitor::new(LiteCursorGrabMode::Locked));
        fields.add_field("Confined", Traitor::new(LiteCursorGrabMode::Confined));
    }
}

pub struct LitePlugin;

impl FyroxUserData for LitePlugin {
    const CLASS_NAME: &'static str = "Plugin";

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method_mut("get", |lua, cls, class_name: mlua::String| {
            with_script_context(|ctx| {
                let plugin = ctx
                    .plugins
                    .as_mut()
                    .ok_or_else(|| lua_error!("plugins not available here"))?;
                for script in plugin.lua_mut().scripts.borrow_mut().inner_mut().iter_mut() {
                    if script.name == class_name.to_str()? {
                        return Ok(script.data.inner_unpacked().unwrap());
                    }
                }
                Err(lua_error!("plugin not found: {}", class_name.to_str()?))
            })
        });
    }
}
impl FyroxUserData for LiteRoutingStrategy {
    const CLASS_NAME: &'static str = "RoutingStrategy";

    fn add_class_fields<'lua, F: UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field("Up", Traitor::new(LiteRoutingStrategy::Up));
        fields.add_field("Down", Traitor::new(LiteRoutingStrategy::Down));
    }
}

impl FyroxUserData for LitePhysics {
    const CLASS_NAME: &'static str = "Physics";

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method("cast_ray", |lua, cls, (opts, results): (Table, Table)| {
            let opts = LiteRayCastOptions {
                ray_origin: opts
                    .get::<_, UserDataRef<Traitor<LiteVector3>>>("ray_origin")?
                    .inner()
                    .to_owned()
                    .into(),
                ray_direction: opts
                    .get::<_, UserDataRef<Traitor<LiteVector3>>>("ray_direction")?
                    .inner()
                    .to_owned()
                    .into(),
                max_len: opts.get::<_, f32>("max_len")?,
                groups: Default::default(),
                sort_results: opts.get::<_, bool>("sort_results")?,
            };
            let results_vec = LitePhysics::cast_ray(opts);
            for result in results_vec {
                let hit = lua.create_table()?;
                hit.set("collider", Traitor::new(result.collider))?;
                results.push(hit)?;
            }
            Ok(())
        });
    }
}

impl FyroxUserData for Log {
    const CLASS_NAME: &'static str = "Log";

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method("info", |lua, cls, args: MultiValue| {
            let s = sformat(args);
            Log::info(s.as_str());
            Ok(())
        });
        methods.add_method("err", |lua, cls, args: MultiValue| {
            let s = sformat(args);
            Log::err(s.as_str());
            Ok(())
        });
        methods.add_method("warn", |lua, cls, args: MultiValue| {
            let s = sformat(args);
            Log::warn(s.as_str());
            Ok(())
        });
    }
}

fn sformat(args: MultiValue) -> String {
    let mut pattern = args.get(0).unwrap().as_str().unwrap().to_owned();
    for i in 1..args.len() {
        let to = format!("{:?}", VerboseLuaValue::new(args.get(i).unwrap().clone()));
        pattern = pattern.replacen("%s", to.as_str(), 1);
    }
    pattern
}

impl FyroxUserData for LiteNode {
    const CLASS_NAME: &'static str = "Node";

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
        type LiteNodeRef<'a> = UserDataRef<'a, Traitor<LiteNode>>;
        methods.add_meta_function(
            MetaMethod::Eq.name(),
            |lua: &Lua, (a, b): (Option<LiteNodeRef>, Option<LiteNodeRef>)| {
                if a.is_none() && b.is_none() {
                    return Ok(false);
                }
                let Some(a) = a else {
                    return Ok(false);
                };
                let Some(b) = b else {
                    return Ok(false);
                };
                Ok(a.inner() == b.inner())
            },
        );
        methods.add_method_mut(
            "add_script",
            |lua, this, script_class_name: mlua::String| {
                let script_class_name = script_class_name.to_str()?;
                if this.get_script(script_class_name).is_some() {
                    return Err(lua_error!(
                        "node already contains script of class {}: {:?}",
                        script_class_name,
                        this
                    ));
                }
                let class = lua
                    .globals()
                    .get::<_, Option<UserDataRef<ScriptClass>>>(script_class_name)?;
                let Some(class) = class else {
                    return Err(lua_error!("class not found: {}", script_class_name));
                };
                let Some(def) = &class.def else {
                    return Err(lua_error!("invalid class: {}", script_class_name));
                };
                let obj = lua.create_userdata(ScriptObject::new(def))?;
                let ud = TypedUserData::<ScriptObject>::new(obj);
                this.inner().add_script(ud)?;
                Ok(this.get_script(script_class_name))
            },
        );
        methods.add_method("parent", |a, b, args: ()| Ok(Traitor::new(b.parent())));
        methods.add_method_mut("as_rigid_body", |a, b, args: ()| {
            Ok(b.as_rigid_body().map(Traitor::new))
        });
        methods.add_method("is_alive", |a, b, args: ()| Ok(b.is_alive()));
        methods.add_method_mut("destroy", |a, b, args: ()| {
            b.destroy();
            Ok(())
        });
        methods.add_method_mut("global_position", |a, b, args: ()| {
            Ok(Traitor::new(LiteVector3::from(b.global_position())))
        });
        methods.add_method_mut("global_rotation", |a, b, args: ()| {
            Ok(Traitor::new(LiteQuaternion::from(b.global_rotation())))
        });
        methods.add_method_mut("local_position", |a, b, args: ()| {
            Ok(Traitor::new(LiteVector3::from(b.local_position())))
        });
        methods.add_method_mut("local_rotation", |a, b, args: ()| {
            Ok(Traitor::new(LiteQuaternion::from(b.local_rotation())))
        });
        methods.add_method_mut(
            "send_hierarchical",
            |a, this, (routing_strategy, value): (UserDataRef<Traitor<LiteRoutingStrategy>>, Value)| {
                // we use Lua interpreter as long as we use the process, so its lifetime is effectively static.
                let value: Value<'static> = unsafe { mem::transmute(value) };
                this.send_hierarchical::<UserScriptImpl>(*routing_strategy.inner(), Traitor::new(SendWrapper::new(value)));
                Ok(())
            },
        );
        methods.add_method_mut("subscribe_to", |a, this, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            this.subscribe_to::<UserScriptImpl>(());
            Ok(())
        });
        methods.add_method_mut(
            "set_local_position",
            |a, b, value: UserDataRef<Traitor<LiteVector3>>| {
                b.set_local_position(Vector3::from(value.inner().to_owned()).into());
                Ok(())
            },
        );
        methods.add_method_mut(
            "set_local_rotation",
            |a, b, value: UserDataRef<Traitor<LiteQuaternion>>| {
                b.set_local_rotation(UnitQuaternion::from(value.inner().to_owned()).into());
                Ok(())
            },
        );
        methods.add_method("find_collider_in_children", |a, b, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            Ok(b.try_get_collider().map(Traitor::new))
        });
        methods.add_method("is_valid", |a, b, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            Ok(b.is_valid())
        });
        methods.add_method("get_script", |a, this, name: mlua::String| {
            Ok(this.get_script(name.to_str()?))
        });
    }
}

impl FyroxUserData for LiteUiNode {
    const CLASS_NAME: &'static str = "UiNode";
}

impl FyroxUserData for LitePrefab {
    const CLASS_NAME: &'static str = "Prefab";

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_method_mut(
            "instantiate_at",
            |lua,
             this,
             (pos, rot): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteQuaternion>>,
            )| {
                let pos: Vector3<f32> = pos.inner().to_owned().into();
                let rot: UnitQuaternion<f32> = rot.inner().to_owned().into();
                Ok(Traitor::new(this.instantiate_at(pos.into(), rot.into())))
            },
        );
    }
}
