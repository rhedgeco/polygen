use std::collections::HashMap;

use heck::ToPascalCase;
use once_cell::sync::Lazy;
use polygen::items::PolyStruct;

static TYPE_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
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

pub fn render_typename(s: &PolyStruct) -> String {
    // early return if the typename is built in
    if let Some(ident) = TYPE_MAP.get(s.name) {
        return ident.to_string();
    }

    // get and format the modules and typename
    let ident = s.name.to_pascal_case();
    let mut module = "".to_string();
    for m in s.module.split("::").skip(1) {
        module += &format!("{}.", m.to_pascal_case());
    }

    // return the combined module and typename
    format!("{module}{ident}")
}
