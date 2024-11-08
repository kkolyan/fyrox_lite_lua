use std::fmt::Write;

pub fn to_hex_dump<T>(v: &T) -> String {
    let mut s = String::new();


    let p = v as *const T as *const u8;
    for i in 0..size_of::<T>() {
        if i > 0 && i % 4 == 0 {
            s.push(' ');
        }
        unsafe {
            let b = *p.add(i);
            write!(&mut s, "{:02X}", b).unwrap();
        }
    }
    s
}