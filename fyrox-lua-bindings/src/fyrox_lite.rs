#![allow(unused_variables)]

use std::{
    borrow::Borrow, marker::{PhantomData, Sized}, mem, ops::{Deref, DerefMut}
};

use crate::{fyrox_lite_class::UserDataClass, fyrox_utils::PluginsRefMut_Ext, lua_error, script::LuaScript};
use fyrox::{
    core::{color::Color, log::Log},
    gui::{brush::Brush, text::TextBuilder, widget::WidgetBuilder}, window::CursorGrabMode,
};
use fyrox_lite_api::{
    lite_math::{LiteQuaternion, LiteVector3},
    lite_node::{LiteNode, LiteRoutingStrategy},
    lite_physics::LiteRigidBody,
    lite_prefab::LitePrefab,
    lite_scene::LiteScene,
    lite_ui::{LiteText, LiteUiNode}, lite_window::LiteWindow, script_context::with_script_context,
};
use mlua::{
    AnyUserData, FromLuaMulti, Lua, MetaMethod, Table, UserData, UserDataMethods, UserDataRef,
    UserDataRefMut, Value,
};
use send_wrapper::SendWrapper;

#[derive(Clone, Copy, Debug)]
pub struct Traitor<T>(T);

impl<T> Deref for Traitor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Traitor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Traitor<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }

    pub fn inner(&self) -> &T {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

#[allow(unused_variables)]
impl UserData for Traitor<LiteRigidBody> {

    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "RigidBody");
    }
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "apply_force",
            |lua, this, force: UserDataRef<Traitor<LiteVector3>>| {
                this.apply_force(*force.inner());
                Ok(())
            },
        );
    }
}

#[allow(unused_variables)]
impl UserData for Traitor<LiteVector3> {
    
    #[rustfmt::skip]
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Vector3");

        fields.add_field_method_get("x", |lua, this| Ok(this.0.get_x()));
        fields.add_field_method_get("y", |lua, this| Ok(this.0.get_y()));
        fields.add_field_method_get("z", |lua, this| Ok(this.0.get_z()));

        fields.add_field_method_set("x", |lua, this, value: f32| { this.0.set_x(value); Ok(()) });
        fields.add_field_method_set("y", |lua, this, value: f32| { this.0.set_y(value); Ok(()) });
        fields.add_field_method_set("z", |lua, this, value: f32| { this.0.set_z(value); Ok(()) });
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "mul",
            |lua, (this, o): (UserDataRef<Traitor<LiteVector3>>, f32)| Ok(Traitor(this.mul(o))),
        );

        methods.add_function(
            "add",
            |lua,
             (this, o): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| { Ok(Traitor(this.add(*o.inner()))) },
        );
        methods.add_function(
            "sub",
            |lua,
             (this, o): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| { Ok(Traitor(this.sub(*o.inner()))) },
        );
        methods.add_function(
            "normalize",
            |lua, this: UserDataRef<Traitor<LiteVector3>>| Ok(Traitor(this.normalize())),
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
}

impl UserData for UserDataClass<LiteVector3> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("new", |lua, cls, (x, y, z): (f32, f32, f32)| {
            Ok(Traitor::new(LiteVector3::new(x, y, z)))
        });
    }

    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_function_get("X",|lua, this| Ok(Traitor(LiteVector3::x_axis())));
        fields.add_field_function_get("Y", |lua, this| Ok(Traitor(LiteVector3::y_axis())));
        fields.add_field_function_get("X", |lua, this| Ok(Traitor(LiteVector3::z_axis())));

        fields.add_field_function_get("ZERO", |lua, this| Ok(Traitor(LiteVector3::zero())));
    }
}

#[allow(unused_variables)]
impl UserData for Traitor<LiteQuaternion> {

    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Quaternion");
    }
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "mul",
            |lua, (this, o): (UserDataRef<Traitor<LiteQuaternion>>, AnyUserData)| {
                if let Ok(o) = o.borrow::<Traitor<LiteVector3>>() {
                    return Ok(lua.create_userdata(Traitor(this.mul__LiteVector(o.0))));
                }
                if let Ok(o) = o.borrow::<Traitor<LiteQuaternion>>() {
                    return Ok(lua.create_any_userdata(Traitor(this.mul__LiteQuaternion(o.0))));
                }
                Err(mlua::Error::runtime("invalid operand type"))
            },
        );
    }
}

impl UserData for UserDataClass<LiteQuaternion> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "face_towards",
            |lua, cls,
             (dir, up): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| {
                Ok(Traitor(LiteQuaternion::face_towards(
                    *dir.inner(),
                    *up.inner(),
                )))
            },
        );
        methods.add_method(
            "from_axis_angle",
            |lua, cls, (axis, angle): (UserDataRef<Traitor<LiteVector3>>, f32)| {
                Ok(Traitor(LiteQuaternion::from_axis_angle(
                    *axis.inner(),
                    angle,
                )))
            },
        );
    }
}

impl UserData for Traitor<LiteRoutingStrategy> {}

impl UserData for Traitor<LiteUiNode> {}

impl UserData for Traitor<LitePrefab> {}

#[allow(unused_variables)]
impl UserData for Traitor<LiteNode> {

    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Node");
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString.name(), |lua, this, args: ()| Ok(format!("{:?}", this.inner())));
        methods.add_method("parent", |a, b, args: ()| Ok(Traitor(b.parent())));
        methods.add_method_mut("as_rigid_body", |a, b, args: ()| {
            Ok(b.as_rigid_body().map(Traitor))
        });
        methods.add_method_mut("destroy", |a, b, args: ()| {
            b.destroy();
            Ok(())
        });
        methods.add_method_mut("global_position", |a, b, args: ()| {
            Ok(Traitor(b.global_position()))
        });
        methods.add_method_mut("global_rotation", |a, b, args: ()| {
            Ok(Traitor(b.global_rotation()))
        });
        methods.add_method_mut("local_position", |a, b, args: ()| {
            Ok(Traitor(b.local_position()))
        });
        methods.add_method_mut("local_rotation", |a, b, args: ()| {
            Ok(Traitor(b.local_rotation()))
        });
        methods.add_method_mut(
            "send_hierarchical",
            |a, b, (routing_strategy, value): (UserDataRef<Traitor<LiteRoutingStrategy>>, Value)| {
                // we use Lua interpreter as long as we use the process, so its lifetime is effectively static.
                let value: Value<'static> = unsafe { mem::transmute(value) };
                b.send_hierarchical(routing_strategy.0, SendWrapper::new(value));
                Ok(())
            },
        );
        methods.add_method_mut(
            "set_local_position",
            |a, b, value: UserDataRef<Traitor<LiteVector3>>| {
                b.set_local_position(value.0);
                Ok(())
            },
        );
        methods.add_method_mut(
            "set_local_rotation",
            |a, b, value: UserDataRef<Traitor<LiteQuaternion>>| {
                b.set_local_rotation(value.0);
                Ok(())
            },
        );
        methods.add_method_mut("subscribe_to", |a, b, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            b.subscribe_to::<SendWrapper<Value>>();
            Ok(())
        });
        methods.add_method("find_collider_in_children", |a, b, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            Ok(b.try_get_collider().map(Traitor))
        });
        methods.add_method("is_valid", |a, b, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            Ok(b.is_valid())
        });
        methods.add_method("get_script", |a, b, name: mlua::String| {
            b.find_script::<LuaScript, _>(|it| {
                if it.name == name.to_string_lossy() {
                    let script_data = it.data.clone();
                    return Some(script_data);
                }
                None
            });
            Ok(())
        });
    }
}

impl UserData for UserDataClass<LiteScene> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("load_async", |lua, this, scene_path: mlua::String| {
            LiteScene::load_async(scene_path.to_str()?);
            Ok(())
        });
    }
}

impl UserData for UserDataClass<LiteText> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
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
}

impl UserData for Traitor<LiteText> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_text_async", |lua, this, text: mlua::String| {
            this.inner_mut().set_text_async(text.to_str()?.to_string());
            Ok(())
        });
    }

    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Text");
    }
}

impl UserData for Traitor<LiteScene> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Scene");
    }
}

impl UserData for Traitor<Brush> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Brush");
    }
}

impl UserData for UserDataClass<Brush> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("solid", |lua, cls, color: UserDataRef<Traitor<Color>>| {
            Ok(Traitor::new(Brush::Solid(*color.inner())))
        });
    }
}

impl UserData for Traitor<Color> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Color");
    }
}

impl UserData for UserDataClass<Color> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field("BLACK", Traitor::new(Color::BLACK));
    }
}

impl UserData for Traitor<LiteWindow> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Window");
    }
}

impl UserData for UserDataClass<LiteWindow> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_cursor_grab", |lua, cls, mode: UserDataRef<Traitor<CursorGrabMode>>| {
            let _ = LiteWindow::set_cursor_grab(*mode.borrow().inner());
            Ok(())
        });
    }
}

impl UserData for Traitor<CursorGrabMode> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "CursorGrabMode");
    }
}

impl UserData for UserDataClass<CursorGrabMode> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field("None", Traitor::new(CursorGrabMode::None));
        fields.add_field("Locked", Traitor::new(CursorGrabMode::Locked));
        fields.add_field("Confined", Traitor::new(CursorGrabMode::Confined));
    }
}

pub struct LitePlugin;

impl UserData for Traitor<LitePlugin> {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), "Plugin");
    }
}

impl UserData for UserDataClass<LitePlugin> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("get", |lua, cls, class_name: mlua::String| {
            with_script_context(|ctx| {
                let plugin = ctx.plugins.as_mut().ok_or_else(|| lua_error!("plugins not available here"))?;
                for script in plugin.lua_mut().scripts.borrow_mut().inner_mut().iter_mut() {
                    if script.name == class_name.to_str()? {
                        return Ok(script.data.inner_unpacked().unwrap())
                    }
                }
                Err(lua_error!("plugin not found: {}", class_name.to_str()?))
            })
        });
    }
}
