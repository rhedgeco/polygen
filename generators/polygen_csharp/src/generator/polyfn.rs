use indoc::formatdoc;
use polygen::items::{PolyField, PolyFn};

use crate::utils;

use super::polytype::convert_typename;

pub fn render_function(lib_name: impl AsRef<str>, f: &PolyFn) -> String {
    let name = f.name;
    let lib_name = lib_name.as_ref();
    let entry_point = f.export_name;
    let out_type = convert_typename(f.params.output.as_ref());
    let doc = formatdoc! {"
        [DllImport(\"{lib_name}\", EntryPoint = \"{entry_point}\", CallingConvention = CallingConvention.Cdecl)]
        public static {out_type} {name}(polygen-inner);"
    };

    let inner = utils::render_each(f.params.inputs.iter(), ", ", render_function_input);
    doc.replace("polygen-inner", &inner)
}

pub fn render_function_input(f: &PolyField) -> String {
    let name = f.name;
    let ty = convert_typename(Some(&f.ty));
    format!("{ty} {name}")
}
