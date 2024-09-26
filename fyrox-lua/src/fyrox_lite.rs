#![allow(unused_variables)]

use std::{
    borrow::Borrow,
    mem,
};

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
use fyrox::{
    core::{color::Color, log::Log},
    gui::{brush::Brush, text::TextBuilder, widget::WidgetBuilder},
    window::CursorGrabMode,
};
use fyrox_lite::{
    lite_math::{LiteQuaternion, LiteVector3},
    lite_node::{LiteNode, LiteRoutingStrategy},
    lite_physics::{LitePhysics, LiteRayCastOptions, LiteRigidBody},
    lite_prefab::LitePrefab,
    lite_scene::LiteScene,
    lite_ui::{LiteText, LiteUiNode},
    lite_window::LiteWindow,
    script_context::with_script_context,
};
use mlua::{
    AnyUserData, Lua, MetaMethod, MultiValue, Table, UserDataFields,
    UserDataMethods, UserDataRef, UserDataRefMut, Value,
};
use send_wrapper::SendWrapper;

impl FyroxUserData for LiteRigidBody {
    const CLASS_NAME: &'static str = "RigidBody";

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_method_mut(
            "apply_force",
            |lua, this, force: UserDataRef<Traitor<LiteVector3>>| {
                this.apply_force(*force.inner());
                Ok(())
            },
        );
    }
}

impl FyroxUserData for LiteVector3 {
    const CLASS_NAME: &'static str = "Vector3";

    #[rustfmt::skip]
    fn add_instance_fields<'lua, F: UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("x", |lua, this| Ok(this.inner().clone().get_x()));
        fields.add_field_method_get("y", |lua, this| Ok(this.inner().clone().get_y()));
        fields.add_field_method_get("z", |lua, this| Ok(this.inner().clone().get_z()));

        fields.add_field_method_set("x", |lua, this, value: f32| { this.inner_mut().set_x(value); Ok(()) });
        fields.add_field_method_set("y", |lua, this, value: f32| { this.inner_mut().set_y(value); Ok(()) });
        fields.add_field_method_set("z", |lua, this, value: f32| { this.inner_mut().set_z(value); Ok(()) });
    }

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_function(
            "mul",
            |lua, (this, o): (UserDataRef<Traitor<LiteVector3>>, f32)| {
                Ok(Traitor::new(this.mul(o)))
            },
        );

        methods.add_function(
            "add",
            |lua,
             (this, o): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| { Ok(Traitor::new(this.add(*o.inner()))) },
        );
        methods.add_function(
            "sub",
            |lua,
             (this, o): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| { Ok(Traitor::new(this.sub(*o.inner()))) },
        );
        methods.add_function(
            "normalize",
            |lua, this: UserDataRef<Traitor<LiteVector3>>| Ok(Traitor::new(this.normalize())),
        );
        methods.add_function(
            "magnitude",
            |lua, this: UserDataRef<Traitor<LiteVector3>>| Ok(this.magnitude()),
        );
        methods.add_function_mut(
            "normalize_inplace",
            |lua, mut this: UserDataRefMut<Traitor<LiteVector3>>| {
                this.normalize_inplace();
                Ok(())
            },
        );
    }

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method("new", |lua, cls, (x, y, z): (f32, f32, f32)| {
            Ok(Traitor::new(LiteVector3::new(x, y, z)))
        });
    }

    fn add_class_fields<'lua, F: UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_function_get("X", |lua, this| Ok(Traitor::new(LiteVector3::x_axis())));
        fields.add_field_function_get("Y", |lua, this| Ok(Traitor::new(LiteVector3::y_axis())));
        fields.add_field_function_get("Z", |lua, this| Ok(Traitor::new(LiteVector3::z_axis())));

        fields.add_field_function_get("ZERO", |lua, this| Ok(Traitor::new(LiteVector3::zero())));
    }
}

impl FyroxUserData for LiteQuaternion {
    const CLASS_NAME: &'static str = "Quaternion";

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_function(
            "mul",
            |lua, (this, o): (UserDataRef<Traitor<LiteQuaternion>>, AnyUserData)| {
                if let Ok(o) = o.borrow::<Traitor<LiteVector3>>() {
                    return Ok(
                        lua.create_userdata(Traitor::new(this.mul__LiteVector(*o.inner())))
                    );
                }
                if let Ok(o) = o.borrow::<Traitor<LiteQuaternion>>() {
                    return Ok(lua.create_userdata(Traitor::new(
                        this.mul__LiteQuaternion(*o.inner()),
                    )));
                }
                Err(mlua::Error::runtime("invalid operand type"))
            },
        );
    }

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method(
            "face_towards",
            |lua,
             cls,
             (dir, up): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| {
                Ok(Traitor::new(LiteQuaternion::face_towards(
                    *dir.inner(),
                    *up.inner(),
                )))
            },
        );
        methods.add_method(
            "from_axis_angle",
            |lua, cls, (axis, angle): (UserDataRef<Traitor<LiteVector3>>, f32)| {
                Ok(Traitor::new(LiteQuaternion::from_axis_angle(
                    *axis.inner(),
                    angle,
                )))
            },
        );
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
                Ok(Traitor::new(
                    this.instantiate_at(*pos.inner(), *rot.inner()),
                ))
            },
        );
    }
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
            |lua: &Lua,
             (a, b): (
                Option<LiteNodeRef>,
                Option<LiteNodeRef>,
            )| {
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
                // it's sound, because Lua outlives a process
                let ud: TypedUserData<'static, ScriptObject> = unsafe { mem::transmute(ud) };
                let data = crate::script_data::ScriptData::Unpacked(SendWrapper::new(ud));
                this.inner().add_script(LuaScript {
                    name: script_class_name.to_string(),
                    data,
                });
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
            Ok(Traitor::new(b.global_position()))
        });
        methods.add_method_mut("global_rotation", |a, b, args: ()| {
            Ok(Traitor::new(b.global_rotation()))
        });
        methods.add_method_mut("local_position", |a, b, args: ()| {
            Ok(Traitor::new(b.local_position()))
        });
        methods.add_method_mut("local_rotation", |a, b, args: ()| {
            Ok(Traitor::new(b.local_rotation()))
        });
        methods.add_method_mut(
            "send_hierarchical",
            |a, this, (routing_strategy, value): (UserDataRef<Traitor<LiteRoutingStrategy>>, Value)| {
                // we use Lua interpreter as long as we use the process, so its lifetime is effectively static.
                let value: Value<'static> = unsafe { mem::transmute(value) };
                this.send_hierarchical(*routing_strategy.inner(), SendWrapper::new(value));
                Ok(())
            },
        );
        methods.add_method_mut("subscribe_to", |a, this, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            this.subscribe_to::<SendWrapper<Value>>();
            Ok(())
        });
        methods.add_method_mut(
            "set_local_position",
            |a, b, value: UserDataRef<Traitor<LiteVector3>>| {
                b.set_local_position(*value.inner());
                Ok(())
            },
        );
        methods.add_method_mut(
            "set_local_rotation",
            |a, b, value: UserDataRef<Traitor<LiteQuaternion>>| {
                b.set_local_rotation(*value.inner());
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

impl Traitor<LiteNode> {
    fn get_script(&self, class_name: &str) -> Option<TypedUserData<'static, ScriptObject>> {
        self.find_script::<LuaScript, _>(|it| {
            if it.name == class_name {
                let script_data = it.data.inner_unpacked();
                return Some(script_data.expect("expected to be unpacked here"));
            }
            None
        })
    }
}

impl FyroxUserData for LiteScene {
    const CLASS_NAME: &'static str = "Scene";

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method_mut("load_async", |lua, this, scene_path: mlua::String| {
            LiteScene::load_async(scene_path.to_str()?);
            Ok(())
        });
    }
}

impl FyroxUserData for LiteText {
    const CLASS_NAME: &'static str = "Text";
    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method_mut("new", |lua, this, state: Table| {
            let mut widget = WidgetBuilder::new();
            if let Some(foreground) =
                state.get::<_, Option<UserDataRef<Traitor<Brush>>>>("foreground")?
            {
                widget = widget.with_foreground(foreground.inner().clone())
            }
            let mut text = TextBuilder::new(widget);
            if let Some(font_size) = state.get::<_, Option<f32>>("font_size")? {
                text = text.with_font_size(font_size);
            }
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
        methods.add_method("solid", |lua, cls, color: UserDataRef<Traitor<Color>>| {
            Ok(Traitor::new(Brush::Solid(*color.inner())))
        });
    }
}

impl FyroxUserData for Color {
    const CLASS_NAME: &'static str = "Color";

    fn add_class_fields<'lua, F: UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field("BLACK", Traitor::new(Color::BLACK));
    }
}

impl FyroxUserData for LiteWindow {
    const CLASS_NAME: &'static str = "Window";

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method_mut(
            "set_cursor_grab",
            |lua, cls, mode: UserDataRef<Traitor<CursorGrabMode>>| {
                let _ = LiteWindow::set_cursor_grab(*mode.borrow().inner());
                Ok(())
            },
        );
    }
}

impl FyroxUserData for CursorGrabMode {
    const CLASS_NAME: &'static str = "CursorGrabMode";

    fn add_class_fields<'lua, F: UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field("None", Traitor::new(CursorGrabMode::None));
        fields.add_field("Locked", Traitor::new(CursorGrabMode::Locked));
        fields.add_field("Confined", Traitor::new(CursorGrabMode::Confined));
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
                ray_origin: *opts
                    .get::<_, UserDataRef<Traitor<LiteVector3>>>("ray_origin")?
                    .inner(),
                ray_direction: *opts
                    .get::<_, UserDataRef<Traitor<LiteVector3>>>("ray_direction")?
                    .inner(),
                max_len: opts.get::<_, f32>("max_len")?,
                groups: Default::default(),
                sort_results: opts.get::<_, bool>("sort_results")?,
            };
            let mut results_vec = Vec::new();
            LitePhysics::cast_ray(opts, &mut results_vec);
            for result in results_vec {
                let hit = lua.create_table()?;
                hit.set("collider", Traitor::new(result.collider))?;
                results.push(hit)?;
            }
            Ok(())
        });
    }
}