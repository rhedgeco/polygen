use indoc::formatdoc;
use polygen::items::{FnInput, PolyFn, PolyStruct};

use crate::{generator::polytype::render_typename, utils};

pub fn render_function(lib_name: impl AsRef<str>, f: &PolyFn) -> String {
    let name = f.name;
    let lib_name = lib_name.as_ref();
    let entry_point = f.export_name;
    let out_type = render_fn_type(f.params.output.as_ref());
    let doc = formatdoc! {"
        [DllImport(\"{lib_name}\", EntryPoint = \"{entry_point}\", CallingConvention = CallingConvention.Cdecl)]
        public static extern {out_type} {name}(polygen-inner);"
    };

    let inner = utils::render_each(f.params.inputs.iter(), ", ", render_function_input);
    doc.replace("polygen-inner", &inner)
}

fn render_function_input(f: &FnInput) -> String {
    let name = f.name;
    let ty = render_fn_type(Some(&f.ty));
    format!("{ty} {name}")
}

fn render_fn_type(ty: Option<&PolyStruct>) -> String {
    let Some(ty) = ty else {
        return format!("void");
    };

    let mut generics = format!("");
    if !ty.generics.is_empty() {
        let items = utils::render_each(ty.generics.iter(), ", ", |g| render_fn_type(Some(g.ty)));
        generics = format!("<{items}>");
    };

    let name = render_typename(ty);
    format!("{name}{generics}")
}
