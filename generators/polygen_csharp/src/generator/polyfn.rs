use heck::ToLowerCamelCase;
use indoc::formatdoc;
use polygen::items::{PolyFn, PolyStruct};

use crate::{
    generator::polytype::{render_typename, TYPE_MAP},
    utils,
};

pub fn render_function(lib_name: impl AsRef<str>, f: &PolyFn) -> String {
    let name = f.name;
    let lib_name = lib_name.as_ref();
    let export_name = f.export_name;

    let params = utils::render_each(f.params.inputs.iter(), ", ", |i| {
        let name = i.name.to_lower_camel_case();
        match TYPE_MAP.get(i.ty.name) {
            None => format!("{name}.data"),
            Some(_) => name,
        }
    });
    let fn_params = utils::render_each(f.params.inputs.iter(), ", ", |i| {
        let name = i.name.to_lower_camel_case();
        let ty = render_fn_type(Some(i.ty));
        format!("{ty} {name}")
    });
    let export_params = utils::render_each(f.params.inputs.iter(), ", ", |i| {
        let name = i.name.to_lower_camel_case();
        let ty = match TYPE_MAP.get(i.ty.name) {
            None => format!("{}.Data", render_fn_type(Some(i.ty))),
            Some(s) => s.to_string(),
        };
        format!("{ty} {name}")
    });

    let out_type;
    let inner_transfer;
    let export_out_type;
    match &f.params.output {
        None => {
            out_type = format!("void");
            inner_transfer = format!("{export_name}({params})");
            export_out_type = format!("void");
        }
        Some(s) => {
            if let Some(s) = TYPE_MAP.get(s.name) {
                out_type = format!("{s}");
                inner_transfer = format!("{export_name}({params})");
                export_out_type = out_type.clone();
            } else {
                out_type = render_fn_type(Some(s));
                inner_transfer = format!("new {out_type}({export_name}({params}))");
                export_out_type = format!("{out_type}.Data");
            }
        }
    }

    formatdoc! {"
        [DllImport(\"{lib_name}\", CallingConvention = CallingConvention.Cdecl)]
        private static extern {export_out_type} {export_name}({export_params});
        public static {out_type} {name}({fn_params}) => {inner_transfer};"
    }
}

pub fn render_fn_type(ty: Option<&PolyStruct>) -> String {
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
