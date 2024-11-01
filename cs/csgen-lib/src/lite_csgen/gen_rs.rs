
#[derive(Debug, Default)]
pub struct RustEmitter {
    pub code: String,
}

impl RustEmitter {
    pub fn emit_statement(&mut self, s: String) {
        self.code.push_str(&s);
        self.code.push_str("\n");
    }
}