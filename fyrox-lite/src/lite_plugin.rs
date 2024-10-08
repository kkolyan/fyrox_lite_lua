use lite_macro::lite_api;

use crate::spi::UserScript;

#[derive(Debug, Clone)]
pub struct LitePlugin;

#[lite_api(class=Plugin)]
impl LitePlugin {
	/// find a plugin script by name
	pub fn get<T: UserScript>(#[class_name] class_name: String, _stub: T::UserScriptGenericStub) -> Result<T, T::LangSpecificError> {
		T::find_plugin_script(class_name.as_str())
	}
}