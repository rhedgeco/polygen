use crate::{
    polytype::{render_typename, render_typename_data},
    utils,
};

use heck::{ToLowerCamelCase, ToPascalCase};
use indent::indent_by;
use indoc::formatdoc;
use polygen::{
    items::{FieldType, FnInput, ImplFn, PolyFn, PolyImpl, PolyStruct, PolyType, StructField},
    PolyBag, PolyMod,
};

pub struct CSharpRenderer {
    pub lib_name: String,
    pub namespace: String,
}

impl CSharpRenderer {
    pub fn render(&self, bag: &PolyBag) -> String {
        let namespace = &self.namespace;
        let module = indent_by(4, self.render_module(bag.root_module()));

        formatdoc! {"
            using System;
            using System.Runtime.InteropServices;
            
            namespace {namespace}
            {{
                {module}
            }}
            "
        }
    }

    fn render_struct(&self, s: &PolyStruct, i: Option<&PolyImpl>) -> String {
        let name = s.name.to_pascal_case();

        let fields = indent_by(
            8,
            utils::join(s.fields.iter(), "\n", |f| self.render_struct_field(f)),
        );

        let functions = match i {
            None => format!(""),
            Some(i) => {
                let mut functions = format!("");
                for f in i.functions {
                    functions += "\n\n";
                    functions += &self.render_struct_function(f);
                }
                indent_by(4, functions)
            }
        };

        formatdoc! {"
            public class {name}
            {{
                internal Data _data;
                public readonly ref Data data = ref _data;

                internal {name}(Data newData)
                {{
                    _data = newData;
                }}

                [StructLayout(LayoutKind.Sequential)]
                public struct Data
                {{
                    {fields}
                }}{functions}
            }}"
        }
    }

    fn render_function(&self, f: &PolyFn) -> String {
        let lib_name = &self.lib_name;
        let export_name = f.export_name;
        let name = f.name.to_pascal_case();
        let out_type = render_typename(f.params.output.as_ref());
        let out_data = render_typename_data(f.params.output.as_ref());

        let export_params = utils::join(f.params.inputs.iter(), ", ", |i| {
            let name = i.name.to_lower_camel_case();
            let ty = render_typename_data(Some(i.ty));
            format!("{ty} {name}")
        });

        let func_params = utils::join(f.params.inputs.iter(), ", ", |i| {
            let name = i.name.to_lower_camel_case();
            let ty = render_typename(Some(i.ty));
            format!("{ty} {name}")
        });

        let convert_params = utils::join(f.params.inputs.iter(), ", ", |i| match i.ty {
            PolyType::Struct(_) => format!("{}._data", i.name.to_lower_camel_case()),
            _ => i.name.to_lower_camel_case(),
        });

        let convert_call = match f.params.output {
            Some(PolyType::Struct(_)) => format!("new {out_type}({export_name}({convert_params}))"),
            _ => format!("{export_name}({convert_params})"),
        };

        formatdoc! {"
            [DllImport(\"{lib_name}\", CallingConvention = CallingConvention.Cdecl)]
            private static extern {out_data} {export_name}({export_params});
            public static {out_type} {name}({func_params}) => {convert_call}"
        }
    }

    fn render_module(&self, m: &PolyMod) -> String {
        let name = m.name().to_pascal_case();
        let items = indent_by(4, self.render_module_items(m));

        formatdoc! {"
            public static class {name}
            {{
                {items}
            }}"
        }
    }

    fn render_module_items(&self, m: &PolyMod) -> String {
        let mut output = String::new();
        let structs = utils::join(m.structs(), "\n\n", |(s, i)| self.render_struct(s, i));
        let functions = utils::join(m.functions(), "\n\n", |f| self.render_function(f));
        let modules = utils::join(m.modules(), "\n\n", |m| self.render_module(m));

        output += &structs;
        if output.len() > 0 && functions.len() > 0 {
            output += "\n\n";
        }

        output += &functions;
        if output.len() > 0 && modules.len() > 0 {
            output += "\n\n";
        }

        output += &modules;
        output
    }

    fn render_struct_field(&self, f: &StructField) -> String {
        let name = f.name.to_lower_camel_case();
        let ty = match f.ty {
            FieldType::Generic(g) => g.to_string(),
            FieldType::Typed(t) => render_typename(Some(t)),
        };

        format!("internal {ty} {name};")
    }

    fn render_struct_function(&self, f: &ImplFn) -> String {
        let lib_name = &self.lib_name;
        let export_name = f.export_name;
        let name = f.name.to_pascal_case();
        let out_type = render_typename(f.params.output.as_ref());
        let out_data = render_typename_data(f.params.output.as_ref());
        let self_input = f.params.inputs.iter().find(|i| i.name == "self");
        let static_keyword = match self_input.is_some() {
            false => " static",
            true => "",
        };

        let export_params = utils::join(f.params.inputs.iter(), ", ", |i| {
            let name = i.name.to_lower_camel_case();
            let ty = render_typename_data(Some(i.ty));
            format!("{ty} {name}")
        });

        let func_params = utils::join(
            f.params.inputs.iter().filter(|f| f.name != "self"),
            ", ",
            |i| {
                let name = i.name.to_lower_camel_case();
                let ty = render_typename(Some(i.ty));
                format!("{ty} {name}")
            },
        );

        let convert_params = utils::join(f.params.inputs.iter(), ", ", |i| match i.ty {
            PolyType::Pointer(_) if i.name == "self" => format!("__polygen_self_ptr"),
            PolyType::Struct(_) if i.name == "self" => format!("this._data"),
            _ => i.name.to_lower_camel_case(),
        });

        let convert_call = match f.params.output {
            Some(PolyType::Struct(_)) => format!("new {out_type}({export_name}({convert_params}))"),
            _ => format!("{export_name}({convert_params})"),
        };

        let conversion = match self_input {
            Some(FnInput {
                name: _,
                ty: ty @ PolyType::Pointer(_),
            }) => {
                let ty = render_typename_data(Some(ty));
                formatdoc! {"
                    
                    {{
                        fixed ({ty} __polygen_self_ptr = &this._data)
                        {{
                            return {convert_call};
                        }}
                    }}"
                }
            }
            _ => format!("=> {convert_call}"),
        };

        formatdoc! {"
            [DllImport(\"{lib_name}\", CallingConvention = CallingConvention.Cdecl)]
            private static extern {out_data} {export_name}({export_params});
            private{static_keyword} {out_type} {name}({func_params}) {conversion}"
        }
    }
}
