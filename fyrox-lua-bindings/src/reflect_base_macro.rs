#[macro_export]
macro_rules! reflect_base {
    () => {
        fn source_path() -> &'static str {
            file!()
        }

        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }

        fn doc(&self) -> &'static str {
            ""
        }

        fn assembly_name(&self) -> &'static str {
            env!("CARGO_PKG_NAME")
        }

        fn type_assembly_name() -> &'static str {
            env!("CARGO_PKG_NAME")
        }

        fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }

        fn as_any(&self, func: &mut dyn FnMut(&dyn std::any::Any)) {
            func(self)
        }

        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn std::any::Any)) {
            func(self)
        }

        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self)
        }

        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self)
        }

        fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let this = std::mem::replace(self, value.take()?);
            Ok(Box::new(this))
        }
    };
}
