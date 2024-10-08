pub mod engine_class;
pub mod struct_class;
pub mod type_to_lua;
pub mod enum_class;

#[macro_export]
macro_rules! writelnu {
    ($dst:expr, $($arg:tt)*) => {{
		use std::fmt::Write;
		writeln!($dst, $($arg)*).unwrap()
    }}
}