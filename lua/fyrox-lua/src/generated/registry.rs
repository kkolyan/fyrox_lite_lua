
pub fn register_classes(lua: &mlua::Lua) {
    use crate::user_data_plus::FyroxUserData;

    fyrox_lite::lite_prefab::LitePrefab::register_class(lua);

    fyrox_lite::lite_log::LiteLog::register_class(lua);

    fyrox_lite::lite_event::Event::register_class(lua);

    fyrox_lite::lite_event::StartCause::register_class(lua);

    fyrox_lite::lite_event::WindowEvent::register_class(lua);

    fyrox_lite::lite_event::DeviceEvent::register_class(lua);

    fyrox_lite::lite_event::PhysicalKey::register_class(lua);

    fyrox_lite::lite_event::KeyCode::register_class(lua);

    fyrox_lite::lite_event::NativeKeyCode::register_class(lua);

    fyrox_lite::lite_event::KeyLocation::register_class(lua);

    fyrox_lite::lite_event::TouchPhase::register_class(lua);

    fyrox_lite::lite_event::Force::register_class(lua);

    fyrox_lite::lite_event::ElementState::register_class(lua);

    fyrox_lite::lite_event::MouseButton::register_class(lua);

    fyrox_lite::lite_event::MouseScrollDelta::register_class(lua);

    fyrox_lite::lite_event::InnerSizeWriter::register_class(lua);

    fyrox_lite::lite_window::LiteWindow::register_class(lua);

    fyrox_lite::lite_window::LiteCursorGrabMode::register_class(lua);

    fyrox_lite::lite_scene::LiteScene::register_class(lua);

    fyrox_lite::lite_physics::LitePhysics::register_class(lua);

    fyrox_lite::lite_physics::LiteFeatureId::register_class(lua);

    fyrox_lite::lite_physics::LiteRigidBody::register_class(lua);

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
