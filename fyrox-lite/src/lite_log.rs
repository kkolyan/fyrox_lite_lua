use fyrox::core::log::Log;
use lite_macro::lite_api;

#[derive(Debug, Clone)]
pub struct LiteLog {}

#[lite_api(class=Log)]
impl LiteLog {
    pub fn info(msg: String) {
        Log::info(msg.as_str());
    }
    pub fn warn(msg: String) {
        Log::warn(msg.as_str());
    }
    pub fn err(msg: String) {
        Log::err(msg.as_str());
    }
}
