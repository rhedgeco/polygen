use heck::ToPascalCase;
use indent::indent_by;
use indoc::formatdoc;
use polygen::items::{FieldType, PolyStruct, StructField};

use crate::{generator::polytype::render_typename, utils};

pub fn render_struct(_lib_name: impl AsRef<str>, s: &PolyStruct) -> String {
    // crate struct template
    let ident = s.name.to_pascal_case();
    let generics = if s.generics.is_empty() {
        format!("")
    } else {
        let generics = utils::render_each(s.generics.iter(), ", ", |g| g.ident.into());
        format!("<{generics}>")
    };
    let doc = formatdoc! {"
        [StructLayout(LayoutKind.Sequential)]
        public struct {ident}{generics}
        {{
            polygen-inner
        }}"
    };

    // render out inner items
    let inner = utils::render_each(s.fields.iter().enumerate(), "\n", render_struct_field);

    // replace
    doc.replace("polygen-inner", &indent_by(4, inner))
}

fn render_struct_field((index, field): (usize, &StructField)) -> String {
    let ty = match field.ty {
        FieldType::Generic(g) => format!("{g}"),
        FieldType::Typed(s) => render_typename(s),
    };

    let name = match field.name {
        "_" => format!("_polygen_field{index}"),
        name => name.into(),
    };

    // combine and render
    format!("private {ty} {name};")
}
