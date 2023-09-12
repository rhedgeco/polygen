use heck::ToPascalCase;
use indent::indent_by;
use indoc::formatdoc;
use polygen::items::{FieldType, PolyStruct, StructField};

use crate::{
    generator::polytype::{render_typename, TYPE_MAP},
    utils,
};

pub fn render_struct(_lib_name: impl AsRef<str>, s: &PolyStruct, _i: Option<&()>) -> String {
    // crate struct template
    let ident = s.name.to_pascal_case();
    let generics = if s.generics.is_empty() {
        format!("")
    } else {
        let generics = utils::render_each(s.generics.iter(), ", ", |g| g.ident.into());
        format!("<{generics}>")
    };
    let doc = formatdoc! {"
        public class {ident}{generics}
        {{
            private Data _data;
            public readonly ref Data data => ref _data;
            
            internal {ident}(Data newData)
            {{
                _data = newData;
            }}

            [StructLayout(LayoutKind.Sequential)]
            public struct Data{generics}
            {{
                polygen-inner
            }}
        }}"
    };

    // render out inner items
    let inner = utils::render_each(s.fields.iter().enumerate(), "\n", render_struct_field);

    // replace
    doc.replace("polygen-inner", &indent_by(8, inner))
}

fn render_struct_field((index, field): (usize, &StructField)) -> String {
    let ty = match field.ty {
        FieldType::Generic(g) => format!("{g}"),
        FieldType::Typed(s) => match TYPE_MAP.get(s.name) {
            Some(s) => s.to_string(),
            None => format!("{}.Data", render_typename(s)),
        },
    };

    let name = match field.name {
        "_" => format!("_polygen_field{index}"),
        name => name.into(),
    };

    // combine and render
    format!("private {ty} {name};")
}
