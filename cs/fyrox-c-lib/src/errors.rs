use std::backtrace::Backtrace;
use std::process::exit;
use to_vec::ToVec;

#[extend::ext]
pub impl<T> Result<T, crate::LangSpecificError> {
    fn handle_scripting_error(self) -> Option<T> {
        match self {
            Ok(it) => Some(it),
            Err(err) => {
                println!("\nScripting Runtime Error:\n{}", err);
                print_backtrace_and_exit();
            }
        }
    }
}

pub(crate) fn print_backtrace_and_exit() -> ! {
    let backtrace = Backtrace::capture().to_string();
    let backtrace_lines = backtrace.lines().to_vec();
    let mut frames = Vec::new();
    for i in 0..backtrace_lines.len() / 2 {
        frames.push(format!("\n{}\n{}", backtrace_lines[i * 2], backtrace_lines[i * 2 + 1]));
    }
    let frames = frames.into_iter().filter(|it| it.contains("fyrox_c::")).to_vec();
    println!("\nRust backtrace:\n{}", frames.join("\n"));
    exit(666);
}