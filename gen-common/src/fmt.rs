use std::{ffi::OsStr, process::{self, Stdio}};

pub fn fmt_file(file: impl AsRef<OsStr>) {
    process::Command::new("rustfmt")
        .arg(file)
        .stderr(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
