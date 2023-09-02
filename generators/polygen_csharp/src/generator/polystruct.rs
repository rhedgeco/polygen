use heck::ToPascalCase;
use indent::indent_by;
use indoc::formatdoc;
use polygen::items::{PolyField, PolyStruct};

use crate::{generator::polytype::convert_polytype, utils};

pub fn render_struct(s: &PolyStruct) -> String {
    // crate struct template
    let ident = s.ident.to_pascal_case();
    let doc = formatdoc! {"
        public struct {ident}
        {{
            polygen-inner
        }}"
    };

    // replace
    let inner = utils::render_each(s.fields.iter().enumerate(), "\n", render_struct_field);
    doc.replace("polygen-inner", &indent_by(4, inner))
}

fn render_struct_field(f: (usize, &PolyField)) -> String {
    // create type and field name
    let index = f.0;
    let ty = convert_polytype(Some(&f.1.ty));
    let name = match f.1.name {
        "_" => format!("_polygen_field{index}"),
        name => name.into(),
    };

    // combine and render
    format!("private {ty} {name};")
}
