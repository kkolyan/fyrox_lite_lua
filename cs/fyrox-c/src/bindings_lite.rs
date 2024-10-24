
use crate::{bindings_manual::*, native_utils};
    
pub extern "C" fn fyrox_lite_Prefab_instantiate_at(__this: NativeInstanceId, position: NativeVector3, orientation: NativeQuaternion) -> NativeNode {
    let position = position.into();
    let orientation = orientation.into();
    let __result = __this.instantiate_at(position, orientation);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Log_info(msg: NativeString) -> () {
    let msg = String::from_utf8(<u8 as NativeType>::from_native_array(msg)).unwrap();
    let __result = fyrox_lite::lite_log::LiteLog::info(msg);
    __result
}
pub extern "C" fn fyrox_lite_Log_warn(msg: NativeString) -> () {
    let msg = String::from_utf8(<u8 as NativeType>::from_native_array(msg)).unwrap();
    let __result = fyrox_lite::lite_log::LiteLog::warn(msg);
    __result
}
pub extern "C" fn fyrox_lite_Log_err(msg: NativeString) -> () {
    let msg = String::from_utf8(<u8 as NativeType>::from_native_array(msg)).unwrap();
    let __result = fyrox_lite::lite_log::LiteLog::err(msg);
    __result
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeRawKeyEvent {
    pub physical_key: NativePhysicalKey,
    pub state: NativeElementState,
}
native_utils!(RawKeyEvent, Native_RawKeyEvent_array, Native_RawKeyEvent_option, Native_RawKeyEvent_result);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeKeyEvent {
    pub physical_key: NativePhysicalKey,
    pub state: NativeElementState,
    pub repeat: bool,
}
native_utils!(KeyEvent, Native_KeyEvent_array, Native_KeyEvent_option, Native_KeyEvent_result);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeTouch {
    pub phase: NativeTouchPhase,
    pub location: NativeInstanceId,
    pub force: NativeNativeForceOption,
    pub id: i64,
}
native_utils!(Touch, Native_Touch_array, Native_Touch_option, Native_Touch_result);
pub extern "C" fn fyrox_lite_InnerSizeWriter_request_inner_size(__this: NativeInstanceId, new_inner_size: NativeInstanceId) -> bool {
    let new_inner_size = new_inner_size.into();
    let __result = __this.request_inner_size(new_inner_size);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Window_set_cursor_grab(mode: NativeCursorGrabMode) -> () {
    let mode = mode.into();
    let __result = fyrox_lite::lite_window::LiteWindow::set_cursor_grab(mode);
    __result
}
pub extern "C" fn fyrox_lite_Scene_load_async(scene_path: NativeString) -> () {
    let scene_path = String::from_utf8(<u8 as NativeType>::from_native_array(scene_path)).unwrap();
    let __result = fyrox_lite::lite_scene::LiteScene::load_async(scene_path);
    __result
}
pub extern "C" fn fyrox_lite_Physics_cast_ray(opts: NativeInstanceId) -> NativeNativeInstanceIdArray {
    let opts = opts.into();
    let __result = fyrox_lite::lite_physics::LitePhysics::cast_ray(opts);
    let __result = <NativeInstanceId as NativeType>::to_native_option(__result);
    __result
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeIntersection {
    pub collider: NativeNode,
    pub normal: NativeVector3,
    pub position: NativeVector3,
    pub feature: NativeFeatureId,
    pub toi: f32,
}
native_utils!(Intersection, Native_Intersection_array, Native_Intersection_option, Native_Intersection_result);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeRayCastOptions {
    pub ray_origin: NativeVector3,
    pub ray_direction: NativeVector3,
    pub max_len: f32,
    pub groups: NativeNativeInstanceIdOption,
    pub sort_results: bool,
}
native_utils!(RayCastOptions, Native_RayCastOptions_array, Native_RayCastOptions_option, Native_RayCastOptions_result);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeInteractionGroups {
    pub memberships: i32,
    pub filter: i32,
}
native_utils!(InteractionGroups, Native_InteractionGroups_array, Native_InteractionGroups_option, Native_InteractionGroups_result);
pub extern "C" fn fyrox_lite_RigidBody_apply_force(__this: NativeInstanceId, force: NativeVector3) -> () {
    let force = force.into();
    let __result = __this.apply_force(force);
    __result
}
pub extern "C" fn fyrox_lite_Text_set_text_async(__this: NativeInstanceId, text: NativeString) -> () {
    let text = String::from_utf8(<u8 as NativeType>::from_native_array(text)).unwrap();
    let __result = __this.set_text_async(text);
    __result
}
pub extern "C" fn fyrox_lite_Text_new(state: NativeInstanceId) -> NativeText {
    let state = state.into();
    let __result = fyrox_lite::lite_ui::LiteText::new(state);
    let __result = __result.into();
    __result
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeTextBuilder {
    pub foregound: NativeNativeBrushOption,
    pub font_size: Nativef32Option,
}
native_utils!(TextBuilder, Native_TextBuilder_array, Native_TextBuilder_option, Native_TextBuilder_result);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeGradientPoint {
    pub stop: f32,
    pub color: NativeColor,
}
native_utils!(GradientPoint, Native_GradientPoint_array, Native_GradientPoint_option, Native_GradientPoint_result);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeVector2 {
    pub x: f32,
    pub y: f32,
}
native_utils!(Vector2, Native_Vector2_array, Native_Vector2_option, Native_Vector2_result);
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeVector2i {
    pub x: i64,
    pub y: i64,
}
native_utils!(Vector2i, Native_Vector2i_array, Native_Vector2i_option, Native_Vector2i_result);
pub extern "C" fn fyrox_lite_Plugin_get(class_name: NativeString) -> NativeNativeInstanceIdResult {
    let class_name = String::from_utf8(<u8 as NativeType>::from_native_array(class_name)).unwrap();
    let __result = fyrox_lite::lite_plugin::LitePlugin::get(class_name);
    let __result = <NativeInstanceId as NativeType>::to_native_result(__result);
    __result
}
pub extern "C" fn fyrox_lite_Node_as_rigid_body(__this: NativeInstanceId) -> NativeNativeRigidBodyOption {
    let __result = __this.as_rigid_body();
    let __result = <NativeRigidBody as NativeType>::to_native_option(__result);
    __result
}
pub extern "C" fn fyrox_lite_Node_get_name(__this: NativeInstanceId) -> NativeNativeStringOption {
    let __result = __this.get_name();
    let __result = <NativeString as NativeType>::to_native_option(__result);
    __result
}
pub extern "C" fn fyrox_lite_Node_get_alive(__this: NativeInstanceId) -> bool {
    let __result = __this.get_alive();
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Node_destroy(__this: NativeInstanceId) -> () {
    let __result = __this.destroy();
    __result
}
pub extern "C" fn fyrox_lite_Node_get_global_position(__this: NativeInstanceId) -> NativeVector3 {
    let __result = __this.get_global_position();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_get_local_position(__this: NativeInstanceId) -> NativeVector3 {
    let __result = __this.get_local_position();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_get_local_rotation(__this: NativeInstanceId) -> NativeQuaternion {
    let __result = __this.get_local_rotation();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_send_hierarchical(__this: NativeInstanceId, routing: NativeRoutingStrategy, payload: NativeMessageId) -> () {
    let routing = routing.into();
    let payload = payload.into();
    let __result = __this.send_hierarchical(routing, payload);
    __result
}
pub extern "C" fn fyrox_lite_Node_set_local_position(__this: NativeInstanceId, new_pos: NativeVector3) -> () {
    let new_pos = new_pos.into();
    let __result = __this.set_local_position(new_pos);
    __result
}
pub extern "C" fn fyrox_lite_Node_set_local_rotation(__this: NativeInstanceId, value: NativeQuaternion) -> () {
    let value = value.into();
    let __result = __this.set_local_rotation(value);
    __result
}
pub extern "C" fn fyrox_lite_Node_subscribe_to(__this: NativeInstanceId) -> () {
    let __result = __this.subscribe_to();
    __result
}
pub extern "C" fn fyrox_lite_Node_find_collider_in_children(__this: NativeInstanceId) -> NativeNativeNodeOption {
    let __result = __this.find_collider_in_children();
    let __result = <NativeNode as NativeType>::to_native_option(__result);
    __result
}
pub extern "C" fn fyrox_lite_Node_get_valid(__this: NativeInstanceId) -> bool {
    let __result = __this.get_valid();
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Node_get_parent(__this: NativeInstanceId) -> NativeNode {
    let __result = __this.get_parent();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_add_script(__this: NativeInstanceId, class_name: NativeString) -> NativeNativeInstanceIdResult {
    let class_name = String::from_utf8(<u8 as NativeType>::from_native_array(class_name)).unwrap();
    let __result = __this.add_script(class_name);
    let __result = <NativeInstanceId as NativeType>::to_native_result(__result);
    __result
}
pub extern "C" fn fyrox_lite_Node_find_script(__this: NativeInstanceId, class_name: NativeString) -> NativeNativeNativeInstanceIdOptionResult {
    let class_name = String::from_utf8(<u8 as NativeType>::from_native_array(class_name)).unwrap();
    let __result = __this.find_script(class_name);
    let __result = <NativeNativeInstanceIdOption as NativeType>::to_native_result(__result);
    __result
}
pub extern "C" fn fyrox_lite_Node_get_global_rotation(__this: NativeInstanceId) -> NativeQuaternion {
    let __result = __this.get_global_rotation();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_tag_is(__this: NativeInstanceId, tag: NativeString) -> bool {
    let tag = String::from_utf8(<u8 as NativeType>::from_native_array(tag)).unwrap();
    let __result = __this.tag_is(tag);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Node_set_tag(__this: NativeInstanceId, tag: NativeString) -> () {
    let tag = String::from_utf8(<u8 as NativeType>::from_native_array(tag)).unwrap();
    let __result = __this.set_tag(tag);
    __result
}
pub extern "C" fn fyrox_lite_Node_get_tag(__this: NativeInstanceId) -> NativeString {
    let __result = __this.get_tag();
    let __result = Arena::allocate_vec(__result.into_bytes());
    __result
}
pub extern "C" fn fyrox_lite_Quaternion_face_towards(dir: NativeVector3, up: NativeVector3) -> NativeQuaternion {
    let dir = dir.into();
    let up = up.into();
    let __result = fyrox_lite_math::lite_math::LiteQuaternion::face_towards(dir, up);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Quaternion_from_axis_angle(axis: NativeVector3, angle: f32) -> NativeQuaternion {
    let axis = axis.into();
    let angle = angle;
    let __result = fyrox_lite_math::lite_math::LiteQuaternion::from_axis_angle(axis, angle);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Quaternion_mul_vec(__this: NativeInstanceId, o: NativeVector3) -> NativeVector3 {
    let o = o.into();
    let __result = __this.mul_vec(o);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Quaternion_mul_quat(__this: NativeInstanceId, rot_delta: NativeQuaternion) -> NativeQuaternion {
    let rot_delta = rot_delta.into();
    let __result = __this.mul_quat(rot_delta);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_get_x(__this: NativeInstanceId) -> f32 {
    let __result = __this.get_x();
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Vector3_get_y(__this: NativeInstanceId) -> f32 {
    let __result = __this.get_y();
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Vector3_get_z(__this: NativeInstanceId) -> f32 {
    let __result = __this.get_z();
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Vector3_set_x(__this: NativeInstanceId, value: f32) -> () {
    let value = value;
    let __result = __this.set_x(value);
    __result
}
pub extern "C" fn fyrox_lite_Vector3_set_y(__this: NativeInstanceId, value: f32) -> () {
    let value = value;
    let __result = __this.set_y(value);
    __result
}
pub extern "C" fn fyrox_lite_Vector3_set_z(__this: NativeInstanceId, value: f32) -> () {
    let value = value;
    let __result = __this.set_z(value);
    __result
}
pub extern "C" fn fyrox_lite_Vector3_get_X() -> NativeVector3 {
    let __result = fyrox_lite_math::lite_math::LiteVector3::get_X();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_get_Y() -> NativeVector3 {
    let __result = fyrox_lite_math::lite_math::LiteVector3::get_Y();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_get_Z() -> NativeVector3 {
    let __result = fyrox_lite_math::lite_math::LiteVector3::get_Z();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_get_ZERO() -> NativeVector3 {
    let __result = fyrox_lite_math::lite_math::LiteVector3::get_ZERO();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_new(x: f32, y: f32, z: f32) -> NativeVector3 {
    let x = x;
    let y = y;
    let z = z;
    let __result = fyrox_lite_math::lite_math::LiteVector3::new(x, y, z);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_mul(__this: NativeInstanceId, o: f32) -> NativeVector3 {
    let o = o;
    let __result = __this.mul(o);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_add(__this: NativeInstanceId, o: NativeVector3) -> NativeVector3 {
    let o = o.into();
    let __result = __this.add(o);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_normalize(__this: NativeInstanceId) -> NativeVector3 {
    let __result = __this.normalize();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_sub(__this: NativeInstanceId, o: NativeVector3) -> NativeVector3 {
    let o = o.into();
    let __result = __this.sub(o);
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Vector3_magnitude(__this: NativeInstanceId) -> f32 {
    let __result = __this.magnitude();
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Vector3_normalize_inplace(__this: NativeInstanceId) -> () {
    let __result = __this.normalize_inplace();
    __result
}