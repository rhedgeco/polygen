use heck::ToPascalCase;
use indent::indent_by;
use indoc::formatdoc;
use polygen::items::{PolyField, PolyStruct};

use crate::{generator::polytype::convert_typename, utils};

pub fn render_struct(_lib_name: impl AsRef<str>, s: &PolyStruct) -> String {
    // crate struct template
    let ident = s.name.to_pascal_case();
    let generics = if s.generics.is_empty() {
        format!("")
    } else {
        let generics = utils::render_each(s.generics.iter(), ", ", |g| g.to_string());
        format!("<{generics}>")
    };
    let doc = formatdoc! {"
        public struct {ident}{generics}
        {{
            polygen-inner
        }}"
    };

    // render out inner items
    let inner = utils::render_each(s.fields.iter().enumerate(), "\n", |f| {
        render_struct_field(s.generics, f)
    });

    // replace
    doc.replace("polygen-inner", &indent_by(4, inner))
}

fn render_struct_field(generics: &[&str], (index, field): (usize, &PolyField)) -> String {
    // create type and field name
    let ty = if generics.contains(&field.ty_name) {
        field.ty_name.to_string()
    } else {
        convert_typename(Some(&field.ty))
    };
    let name = match field.name {
        "_" => format!("_polygen_field{index}"),
        name => name.into(),
    };

    // combine and render
    format!("private {ty} {name};")
}
