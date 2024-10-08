use std::fmt::Debug;

use lite_macro::lite_api;

use crate::script_context::with_script_context;

#[derive(Debug, Clone)]
pub struct LiteScene;

#[lite_api(class=Scene)]
impl LiteScene {
    pub fn load_async(scene_path: String) {
        with_script_context(|sc| {
            sc.async_scene_loader
                .as_mut()
                .expect("async scene loader not available")
                .request(scene_path);
        })
    }
}
