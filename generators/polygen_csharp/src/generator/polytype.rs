use heck::ToPascalCase;
use polygen::items::{PolyStruct, PolyType};

pub fn convert_polytype(t: Option<&PolyType>) -> String {
    match t {
        None => format!("void"),
        Some(PolyType::Struct(s)) => convert_typename(s),
        Some(PolyType::Ref(t))
        | Some(PolyType::RefMut(t))
        | Some(PolyType::PtrMut(t))
        | Some(PolyType::PtrConst(t)) => {
            format!("{}*", convert_polytype(Some(t)))
        }
    }
}

fn convert_typename(s: &PolyStruct) -> String {
    let mut module = String::new();
    for mod_name in s.module.split("::").skip(1) {
        let mod_name = mod_name.to_pascal_case();
        module += &format!("{mod_name}.");
    }

    let ident = match s.ident {
        "u8" => "byte".into(),
        "u16" => "ushort".into(),
        "u32" => "uint".into(),
        "u64" => "ulong".into(),
        "usize" => "nuint".into(),

        "i8" => "sbyte".into(),
        "i16" => "short".into(),
        "i32" => "int".into(),
        "i64" => "long".into(),
        "isize" => "nint".into(),

        "bool" => "bool".into(),
        "f32" => "float".into(),
        "f64" => "double".into(),

        ident => ident.to_pascal_case(),
    };

    module + &ident
}
