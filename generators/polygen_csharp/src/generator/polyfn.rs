use indoc::formatdoc;
use polygen::items::{PolyField, PolyFn};

use crate::utils;

use super::polytype::convert_polytype;

pub fn render_function(lib_name: impl AsRef<str>, f: &PolyFn) -> String {
    let ident = f.ident;
    let lib_name = lib_name.as_ref();
    let entry_point = f.export_ident;
    let out_type = convert_polytype(f.output.as_ref());
    let doc = formatdoc! {"
        [\"{lib_name}\", EntryPoint = \"{entry_point}\"]
        public static {out_type} {ident}(polygen-inner);"
    };

    let inner = utils::render_each(f.inputs.iter(), ", ", render_function_input);
    doc.replace("polygen-inner", &inner)
}

fn render_function_input(f: &PolyField) -> String {
    let name = f.name;
    let ty = convert_polytype(Some(&f.ty));
    format!("{ty} {name}")
}
