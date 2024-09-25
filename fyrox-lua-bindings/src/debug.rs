use std::{any::Any, fmt::Debug};

use mlua::{Function, Lua, MetaMethod, Value};

use crate::os_event::RustEnum;

pub fn var_dump(_lua: &Lua, value: Value, indent: &str) -> mlua::Result<()> {
    match value {
        Value::Nil => println!("{}nil", indent),
        Value::Boolean(it) => println!("{}{} # boolean", indent, it),
        Value::LightUserData(it) => println!("{}{:?}{:?}", indent, it.type_id(), it.0),
        Value::Integer(it) => println!("{}{} # integer", indent, it),
        Value::Number(it) => println!("{}{} # number", indent, it),
        Value::Vector(it) => println!("{}{:?} # vector (luau)", indent, it),
        Value::String(it) => println!("{}{} # string", indent, it.to_str()?),
        Value::Table(it) => println!("{}{:?}", indent, it),
        Value::Function(it) => println!("{}{:?}", indent, it),
        Value::Thread(it) => println!("{}{:?}", indent, it),
        Value::UserData(it) => {
            if let Ok(it) = it.borrow::<RustEnum>() {
                println!("{}{:?}", indent, it);
                return Ok(());
            }
            println!("{}UserData", indent);
        }
        Value::Error(it) => println!("{}{} # error", indent, it),
    }
    Ok(())
}

#[derive(Clone)]
pub struct VerboseLuaValue<'a>(Value<'a>);

impl<'a> VerboseLuaValue<'a> {
    pub fn new(value: Value<'a>) -> Self {
        Self(value)
    }

    pub fn inner(&self) -> &Value<'a> {
        &self.0
    }
}

impl<'a> Debug for VerboseLuaValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Value::Nil => writeln!(f, "nil"),
            Value::Boolean(it) => write!(f, "{:?}", it),
            Value::LightUserData(it) => write!(f, "{:?}", it),
            Value::Integer(it) => write!(f, "{:?}", it),
            Value::Number(it) => write!(f, "{:?}", it),
            Value::Vector(it) => write!(f, "{:?}", it),
            Value::String(it) => write!(f, "{:?}", it),
            Value::Table(it) => write!(f, "{:?}", it),
            Value::Function(it) => write!(f, "{:?}", it),
            Value::Thread(it) => write!(f, "{:?}", it),
            Value::UserData(it) => {
                let tostring = it
                    .get_metatable()
                    .unwrap()
                    .get::<Option<Function>>(MetaMethod::ToString.name())
                    .unwrap();
                if let Some(tostring) = tostring {
                    match tostring.call::<_, mlua::String>(it) {
                        Ok(it) => {
                            write!(f, "{}", it.to_string_lossy())
                        }
                        Err(err) => {
                            write!(f, "{}", err)
                        }
                    }
                } else {
                    write!(f, "UserData")
                }
            }
            Value::Error(it) => write!(f, "{:?}", it),
        }
    }
}
