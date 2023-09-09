use heck::ToPascalCase;
use indent::indent_by;
use indoc::formatdoc;
use polygen::items::{PolyField, PolyFn, PolyImpl, PolyStruct};

use crate::{generator::polytype::convert_typename, utils};

use super::polyfn::render_function_input;

pub fn render_struct(lib_name: impl AsRef<str>, s: &PolyStruct, i: Option<&PolyImpl>) -> String {
    // crate struct template
    let ident = s.name.to_pascal_case();
    let generics = utils::render_each(s.generics.iter(), ", ", |s| s.to_string());
    let doc = formatdoc! {"
        public struct {ident}<{generics}>
        {{
            polygen-inner
        }}"
    };

    // render out inner items
    let lib_name = lib_name.as_ref();
    let mut inner = utils::render_each(s.fields.iter().enumerate(), "\n", |f| {
        render_struct_field(s.generics, f)
    });
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

fn render_impl_function(lib_name: impl AsRef<str>, implfn: &PolyFn) -> String {
    let lib_name = lib_name.as_ref();
    let name = implfn.name.to_pascal_case();
    let entry_point = implfn.export_name;
    let out_type = convert_typename(implfn.params.output.as_ref());
    let transfer = utils::render_each(implfn.params.inputs.iter(), ", ", |f| f.name.into());
    let params = utils::render_each(implfn.params.inputs.iter(), ", ", render_function_input);

    let function = match implfn.params.inputs.first() {
        Some(f) if f.name == "self" => {
            let self_ty = convert_typename(Some(&f.ty));
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
