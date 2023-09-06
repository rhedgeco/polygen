use heck::ToPascalCase;
use indent::indent_by;
use indoc::formatdoc;
use polygen::items::{ImplFn, PolyField, PolyImpl, PolyStruct};

use crate::{generator::polytype::convert_polytype, utils};

use super::polyfn::render_function_input;

pub fn render_struct(lib_name: impl AsRef<str>, s: &PolyStruct, i: Option<&PolyImpl>) -> String {
    // crate struct template
    let ident = s.ident.export_name.to_pascal_case();
    let doc = formatdoc! {"
        public struct {ident}
        {{
            polygen-inner
        }}"
    };

    // render out inner items
    let lib_name = lib_name.as_ref();
    let mut inner = utils::render_each(s.fields.iter().enumerate(), "\n", render_struct_field);
    if let Some(r#impl) = i {
        inner += &format!(
            "\n\n{}",
            utils::render_each(r#impl.functions.iter(), "\n\n", |implfn| {
                render_impl_function(lib_name, implfn)
            })
        );
    }

    // replace
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

fn render_impl_function(lib_name: impl AsRef<str>, implfn: &ImplFn) -> String {
    let lib_name = lib_name.as_ref();
    let name = implfn.name.to_pascal_case();
    let entry_point = implfn.export_name;
    let out_type = convert_polytype(implfn.output.as_ref());
    let static_keyword = match implfn.inputs.get(0) {
        Some(input) if input.name == "self" => "",
        _ => " static",
    };

    let mut out = formatdoc! {"
        [DllImport(\"{lib_name}\", CallingConvention = CallingConvention.Cdecl)]
        private static {out_type} {entry_point}(polygen-params);
        public{static_keyword} {out_type} {name}(polygen-self-params) {{
            return {entry_point}(polygen-transfer);
        }}"
    };

    let params = utils::render_each(implfn.inputs.iter(), ", ", render_function_input);
    let transfer = utils::render_each(implfn.inputs.iter(), ", ", |f| match f.name {
        "self" => format!("this"),
        name => name.to_string(),
    });
    let self_params = utils::render_each(
        implfn.inputs.iter().filter(|f| f.name != "self"),
        ", ",
        render_function_input,
    );
    out = out.replace("polygen-params", &params);
    out = out.replace("polygen-transfer", &transfer);
    out = out.replace("polygen-self-params", &self_params);
    out
}
