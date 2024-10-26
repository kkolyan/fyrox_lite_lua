
pub fn register_classes(lua: &mlua::Lua) {
    use crate::user_data_plus::FyroxUserData;
    fyrox_lite::lite_prefab::LitePrefab::register_class(lua);

    fyrox_lite::lite_log::LiteLog::register_class(lua);

    fyrox_lite::lite_window::LiteWindow::register_class(lua);

    fyrox_lite::lite_window::LiteCursorGrabMode::register_class(lua);

    fyrox_lite::lite_scene::LiteScene::register_class(lua);

    fyrox_lite::lite_physics::LitePhysics::register_class(lua);

    fyrox_lite::lite_physics::LiteFeatureId::register_class(lua);

    fyrox_lite::lite_physics::LiteRigidBody::register_class(lua);

    fyrox_lite::lite_input::Input::register_class(lua);

    fyrox_lite::lite_input::LiteKeyCode::register_class(lua);

    fyrox_lite::lite_input::LiteMouseButton::register_class(lua);

    fyrox_lite::lite_ui::LiteUiNode::register_class(lua);

    fyrox_lite::lite_ui::LiteText::register_class(lua);

    fyrox_lite::lite_ui::Brush::register_class(lua);

    fyrox_lite::lite_ui::Color::register_class(lua);

    fyrox_lite::lite_plugin::LitePlugin::register_class(lua);

    fyrox_lite::lite_node::LiteNode::register_class(lua);

    fyrox_lite::lite_node::LiteRoutingStrategy::register_class(lua);

    fyrox_lite_math::lite_math::LiteQuaternion::register_class(lua);

    fyrox_lite_math::lite_math::LiteVector3::register_class(lua);
}
