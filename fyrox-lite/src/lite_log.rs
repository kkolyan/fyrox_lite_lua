use fyrox::core::log::Log;
use fyrox_lite_macro::fyrox_lite;

#[derive(Debug)]
pub struct LiteLog {}

#[fyrox_lite(Log)]
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
