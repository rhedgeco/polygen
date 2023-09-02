use polygen::items::{PolyStruct, PolyType};

pub fn convert_polytype(t: Option<&PolyType>) -> String {
    match t {
        None => format!("void"),
        Some(PolyType::Struct(s)) => convert_typename(s),
        Some(PolyType::Ref(t))
        | Some(PolyType::RefMut(t))
        | Some(PolyType::PtrMut(t))
        | Some(PolyType::PtrConst(t)) => {
            format!("*{}", convert_polytype(Some(t)))
        }
    }
}

fn convert_typename(s: &PolyStruct) -> String {
    let mut module = String::new();
    for mod_name in s.module.split("::").skip(1) {
        let mod_name = heck::AsPascalCase(mod_name);
        module += &format!("{mod_name}.");
    }

    let ident = match s.ident {
        "u8" => "byte",
        "u16" => "ushort",
        "u32" => "uint",
        "u64" => "ulong",
        "usize" => "nuint",

        "i8" => "sbyte",
        "i16" => "short",
        "i32" => "int",
        "i64" => "long",
        "isize" => "nint",

        "bool" => "bool",
        "f32" => "float",
        "f64" => "double",

        ident => ident,
    };

    module + ident
}
