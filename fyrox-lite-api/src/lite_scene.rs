use crate::script_context::with_script_context;


pub fn load_scene_async(scene_path: &str) {
	with_script_context(|sc| {
        sc
            .async_scene_loader
			.as_mut()
			.expect("async scene loader not available")
            .request(scene_path);
	});
}