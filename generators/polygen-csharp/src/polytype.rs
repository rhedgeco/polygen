use crate::utils;

use std::collections::HashMap;

use heck::ToPascalCase;
use once_cell::sync::Lazy;
use polygen::items::{PolyStruct, PolyType};

static PRIMITIVES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("u8", "byte"),
        ("u16", "ushort"),
        ("u32", "uint"),
        ("u64", "ulong"),
        ("usize", "nuint"),
        ("i8", "sbyte"),
        ("i16", "short"),
        ("i32", "int"),
        ("i64", "long"),
        ("isize", "nint"),
        ("bool", "bool"),
        ("f32", "float"),
        ("f64", "double"),
    ])
});

pub fn render_typename(t: Option<&PolyType>) -> String {
    match t {
        None => format!("void"),
        Some(PolyType::Pointer(t)) => format!("{}*", render_typename_data(Some(t))),
        Some(PolyType::Primitive(p)) => format!("{}", PRIMITIVES.get(p).unwrap()),
        Some(PolyType::Struct(s)) => render_structname(s),
    }
}

pub fn render_typename_data(t: Option<&PolyType>) -> String {
    match t {
        None => format!("void"),
        Some(PolyType::Pointer(t)) => format!("{}*", render_typename_data(Some(t))),
        Some(PolyType::Primitive(p)) => format!("{}", PRIMITIVES.get(p).unwrap()),
        Some(PolyType::Struct(s)) => {
            let structname = render_structname(s);
            format!("{structname}.Data")
        }
    }
}

pub fn render_structname(s: &PolyStruct) -> String {
    let mut modules = utils::join(s.module.split("::").skip(1), ".", |m| m.to_pascal_case());
    if modules.len() > 0 {
        modules = format!("{modules}.");
    }

    let mut generics = utils::join(s.generics.iter(), ", ", |g| render_typename(Some(g.ty)));
    if generics.len() > 0 {
        generics = format!("<{generics}>");
    }

    let name = s.name.to_pascal_case();
    format!("{modules}{name}{generics}")
}
