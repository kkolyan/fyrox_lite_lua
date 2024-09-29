use fyrox_lite_macro::fyrox_lite_engine_class;

use crate::script_context::with_script_context;

pub struct LiteScene;

#[fyrox_lite_engine_class("Scene")]
impl LiteScene {
    pub fn load_async(scene_path: &str) {
        with_script_context(|sc| {
            sc.async_scene_loader
                .as_mut()
                .expect("async scene loader not available")
                .request(scene_path);
        })
    }
}
