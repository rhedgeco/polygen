use heck::ToPascalCase;
use indent::indent_by;
use indoc::formatdoc;
use polygen::items::{PolyField, PolyFn, PolyImpl, PolyStruct};

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

fn render_impl_function(lib_name: impl AsRef<str>, implfn: &PolyFn) -> String {
    let lib_name = lib_name.as_ref();
    let name = implfn.ident.name.to_pascal_case();
    let entry_point = implfn.ident.export_name;
    let out_type = convert_polytype(implfn.params.output.as_ref());
    let transfer = utils::render_each(implfn.params.inputs.iter(), ", ", |f| f.name.into());
    let params = utils::render_each(implfn.params.inputs.iter(), ", ", render_function_input);

    let function = match implfn.params.inputs.first() {
        Some(f) if f.name == "self" && f.ty.nesting_depth() > 1 => {
            let self_ty = convert_polytype(Some(&f.ty));
            let self_params = utils::render_each(
                implfn.params.inputs.iter().filter(|f| f.name != "self"),
                ", ",
                render_function_input,
            );

            formatdoc! {"
                public {out_type} {name}({self_params})
                {{
                    fixed ({self_ty} self = &this)
                    {{
                        return {entry_point}({transfer});
                    }}
                }}"
            }
        }
        _ => formatdoc! {"
            public static {out_type} {name}({params})
            {{
                return {entry_point}({transfer});
            }}"
        },
    };

    let external = formatdoc! {"
        [DllImport(\"{lib_name}\", CallingConvention = CallingConvention.Cdecl)]
        private static {out_type} {entry_point}({params});"
    };

    format!("{external}\n{function}")
}
