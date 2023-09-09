use heck::ToPascalCase;
use polygen::items::PolyStruct;

use crate::utils;

pub fn convert_typename(s: Option<&PolyStruct>) -> String {
    let Some(s) = s else { return format!("void") };
    let mut module = String::new();
    for mod_name in s.module.split("::").skip(1) {
        let mod_name = mod_name.to_pascal_case();
        module += &format!("{mod_name}.");
    }

    let ident = match s.name {
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

        ident => {
            let generics = utils::render_each(s.generics.iter(), ", ", |g| {
                let gen_field = s.fields.iter().find(|f| f.ty_name == *g).unwrap();
                convert_typename(Some(gen_field.ty))
            });
            format!("{ident}<{generics}>")
        }
    };

    module + &ident
}
