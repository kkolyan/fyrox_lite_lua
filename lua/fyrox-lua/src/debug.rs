use std::{collections::HashSet, fmt::Debug};

use mlua::{Lua, MultiValue, Value};

use crate::fmt_pretty::fmt_pretty;

pub fn var_dump(_lua: &Lua, args: MultiValue) -> mlua::Result<()> {
    for i in 0..args.len() {
        if i > 0 {
            print!("\t");
        }
        let arg = args.get(i).expect("WTF, `i` is from the loop");
        print!("{:?}", VerboseLuaValue::new(arg.clone()));
    }
    println!();
    Ok(())
}

// for some reason, print calls are heavily buffered.
pub fn override_print(vm: &Lua) {
    vm.globals()
    .set(
        "print",
        vm.create_function(|_lua, args: MultiValue| {
            for i in 0..args.len() {
                if i > 0 {
                    print!("\t");
                }
                let arg = args.get(i).expect("WTF, `i` is from the loop");
                print!("{}", arg.to_string()?);
            }
            println!();
            Ok(())
        })
        .unwrap(),
    )
    .unwrap();
}

#[derive(Clone)]
pub struct VerboseLuaValue<'a>(Value<'a>);

impl<'a> VerboseLuaValue<'a> {
    pub fn new(value: Value<'a>) -> Self {
        Self(value)
    }
}

impl<'a> Debug for VerboseLuaValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_pretty(&self.0, f, true, 0, &mut HashSet::new())
        // match &self.0 {
        //     Value::Table(it) => write!(f, "{:?}", it),
        //     Value::UserData(it) => {
        //         let tostring = it
        //             .get_metatable()
        //             .unwrap()
        //             .get::<Option<Function>>(MetaMethod::ToString.name())
        //             .unwrap();
        //         if let Some(tostring) = tostring {
        //             match tostring.call::<_, mlua::String>(it) {
        //                 Ok(it) => {
        //                     write!(f, "{}", it.to_string_lossy())
        //                 }
        //                 Err(err) => {
        //                     write!(f, "{}", err)
        //                 }
        //             }
        //         } else {
        //             write!(f, "UserData")
        //         }
        //     }
        //     other => write!(f, "{:#?}", other),
        // }
    }
}
