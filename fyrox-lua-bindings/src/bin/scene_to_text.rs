use std::fmt::Write;
use std::{fmt, io};
use std::{fs};

use fyrox::core::visitor::FieldKind;
use fyrox::core::{pool::{Handle, Pool}, visitor::{Visitor, VisitorNode}};

fn main() {
	scene_to_text();
}

fn scene_to_text() {
    let data = fs::read("C:/dev/rust/fyrox_no_lua/data/guard_cylinder.rgs").unwrap();
    let visitor = Visitor::load_from_memory(&data).unwrap();
    let mut s = String::new();
    print_node(&visitor.nodes, visitor.root, "", &mut s).unwrap();
    fs::write("guard_cylinder.lua.yaml", s).unwrap();
}

fn print_node(
    nodes: &Pool<VisitorNode>,
    node: Handle<VisitorNode>,
    prefix: &str,
    w: &mut dyn Write,
) -> fmt::Result {
    if node.is_none() {
        return Ok(());
    }
    let n = &nodes[node];
    writeln!(w, "{}{}:", prefix, n.name)?;
    for field in n.fields.iter() {
		if field.name == "Id" {
			continue;
		}
		if field.name == "State" {
			continue;
		}
        write!(w, "{}  @{}: ", prefix, field.name)?;
		display_kind(&field.kind, w)?;
		writeln!(w)?;
    }
    for child in n.children.iter() {
        print_node(nodes, *child, format!("{}  ", prefix).as_str(), w)?;
    }
    Ok(())
}


fn display_kind(kind: &FieldKind, w: &mut dyn Write) -> fmt::Result {
	match kind {
		FieldKind::Bool(it) => write!(w, "{:?}", it)?,
		FieldKind::U8(it) => write!(w, "{:?}", it)?,
		FieldKind::I8(it) => write!(w, "{:?}", it)?,
		FieldKind::U16(it) => write!(w, "{:?}", it)?,
		FieldKind::I16(it) => write!(w, "{:?}", it)?,
		FieldKind::U32(it) => write!(w, "{:?}", it)?,
		FieldKind::I32(it) => write!(w, "{:?}", it)?,
		FieldKind::U64(it) => write!(w, "{:?}", it)?,
		FieldKind::I64(it) => write!(w, "{:?}", it)?,
		FieldKind::F32(it) => write!(w, "{:?}", it)?,
		FieldKind::F64(it) => write!(w, "{:?}", it)?,
		FieldKind::UnitQuaternion(it) => write!(w, "{:?}", it)?,
		FieldKind::Matrix4(it) => write!(w, "{:?}", it)?,
		FieldKind::BinaryBlob(it) => write!(w, "{:?}", String::from_utf8_lossy(it))?,
		FieldKind::Matrix3(it) => write!(w, "{:?}", it)?,
		FieldKind::Uuid(it) => write!(w, "{:?}", it)?,
		FieldKind::UnitComplex(it) => write!(w, "{:?}", it)?,
		FieldKind::PodArray { type_id, element_size, bytes } => write!(w, "{:?}", "PodArray")?,
		FieldKind::Matrix2(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2F32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3F32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4F32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2F64(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3F64(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4F64(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2U8(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3U8(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4U8(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2I8(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3I8(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4I8(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2U16(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3U16(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4U16(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2I16(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3I16(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4I16(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2U32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3U32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4U32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2I32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3I32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4I32(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2U64(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3U64(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4U64(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector2I64(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector3I64(it) => write!(w, "{:?}", it)?,
		FieldKind::Vector4I64(it) => write!(w, "{:?}", it)?,
	}
	Ok(())
}
