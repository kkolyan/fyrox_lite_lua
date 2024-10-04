use core::fmt;
use std::{cmp::Ordering, collections::HashSet, os::raw::c_void};

use mlua::{Number, Table, Value};


pub fn fmt_pretty(
	value: &Value,
	fmt: &mut fmt::Formatter,
	recursive: bool,
	ident: usize,
	visited: &mut HashSet<*const c_void>,
) -> fmt::Result {
	match value {
		Value::Nil => write!(fmt, "nil"),
		Value::Boolean(b) => write!(fmt, "{b}"),
		Value::LightUserData(ud) if ud.0.is_null() => write!(fmt, "null"),
		Value::LightUserData(ud) => write!(fmt, "lightuserdata: {:?}", ud.0),
		Value::Integer(i) => write!(fmt, "{i}"),
		Value::Number(n) => write!(fmt, "{n}"),
		Value::Vector(v) => write!(fmt, "{v}"),
		Value::String(s) => write!(fmt, "{s:?}"),
		Value::Table(t) if recursive && !visited.contains(&t.to_pointer()) => {
			table_fmt_pretty(t, fmt, ident, visited)
		}
		t @ Value::Table(_) => write!(fmt, "table: {:?}", t.to_pointer()),
		f @ Value::Function(_) => write!(fmt, "function: {:?}", f.to_pointer()),
		t @ Value::Thread(_) => write!(fmt, "thread: {:?}", t.to_pointer()),
		u @ Value::UserData(_ud) => {
			// Try `__name/__type` first then `__tostring`
			let name = u.to_string().ok();
			let s = name
				.map(|name| format!("{name}: {:?}", u.to_pointer()))
				.unwrap_or_else(|| format!("userdata: {:?}", u.to_pointer()));
			write!(fmt, "{s}")
		}
		Value::Error(e) if recursive => write!(fmt, "{e:?}"),
		Value::Error(_) => write!(fmt, "error"),
	}
}

fn table_fmt_pretty(
	table: &Table,
	fmt: &mut fmt::Formatter,
	ident: usize,
	visited: &mut HashSet<*const c_void>,
) -> fmt::Result {
	visited.insert(table.to_pointer());

	let t = table.clone();
	// Collect key/value pairs into a vector so we can sort them
	let mut pairs = t.pairs::<Value, Value>().flatten().collect::<Vec<_>>();
	// Sort keys
	pairs.sort_by(|(a, _), (b, _)| cmp(a, b));
	if pairs.is_empty() {
		return write!(fmt, "{{}}");
	}
	writeln!(fmt, "{{")?;
	for (key, value) in pairs {
		write!(fmt, "{}[", " ".repeat(ident + 2))?;
		fmt_pretty(&key, fmt, false, ident + 2, visited)?;
		write!(fmt, "] = ")?;
		fmt_pretty(&value, fmt, true, ident + 2, visited)?;
		writeln!(fmt, ",")?;
	}
	write!(fmt, "{}}}", " ".repeat(ident))
}



// Compares two values.
// Used to sort values for Debug printing.
fn cmp(a: &Value, other: &Value) -> Ordering {
	fn cmp_num(a: Number, b: Number) -> Ordering {
		match (a, b) {
			_ if a < b => Ordering::Less,
			_ if a > b => Ordering::Greater,
			_ => Ordering::Equal,
		}
	}

	match (a, other) {
		// Nil
		(Value::Nil, Value::Nil) => Ordering::Equal,
		(Value::Nil, _) => Ordering::Less,
		(_, Value::Nil) => Ordering::Greater,
		// Null (a special case)
		(Value::LightUserData(ud1), Value::LightUserData(ud2)) if ud1 == ud2 => Ordering::Equal,
		(Value::LightUserData(ud1), _) if ud1.0.is_null() => Ordering::Less,
		(_, Value::LightUserData(ud2)) if ud2.0.is_null() => Ordering::Greater,
		// Boolean
		(Value::Boolean(a), Value::Boolean(b)) => a.cmp(b),
		(Value::Boolean(_), _) => Ordering::Less,
		(_, Value::Boolean(_)) => Ordering::Greater,
		// Integer && Number
		(Value::Integer(a), Value::Integer(b)) => a.cmp(b),
		(&Value::Integer(a), &Value::Number(b)) => cmp_num(a as Number, b),
		(&Value::Number(a), &Value::Integer(b)) => cmp_num(a, b as Number),
		(&Value::Number(a), &Value::Number(b)) => cmp_num(a, b),
		(Value::Integer(_) | Value::Number(_), _) => Ordering::Less,
		(_, Value::Integer(_) | Value::Number(_)) => Ordering::Greater,
		// String
		(Value::String(a), Value::String(b)) => a.as_bytes().cmp(b.as_bytes()),
		(Value::String(_), _) => Ordering::Less,
		(_, Value::String(_)) => Ordering::Greater,
		// Other variants can be randomly ordered
		(a, b) => a.to_pointer().cmp(&b.to_pointer()),
	}
}