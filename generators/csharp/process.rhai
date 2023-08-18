private fn is_keyword(string) {
    const KEYWORDS = [
        "abstract", "as", "base", "bool", "break",
        "byte", "case", "catch", "char", "checked",
        "class", "const", "continue", "decimal", "default",
        "delegate", "do", "double", "else", "enum",
        "event", "explicit", "extern", "false", "finally",
        "fixed", "float", "for", "foreach", "goto",
        "if", "implicit", "in", "int", "interface",
        "internal", "is", "lock", "long", "namespace",
        "new", "null", "object", "operator", "out",
        "override", "params", "private", "protected", "public",
        "readonly", "ref", "return", "sbyte", "sealed",
        "short", "sizeof", "stackalloc", "static",
        "string", "struct", "switch", "this", "throw",
        "true", "try", "typeof", "uint", "ulong",
        "unchecked", "unsafe", "ushort", "using",
        "virtual", "void", "volatile", "while"
    ];

    if KEYWORDS.contains(string) {
        return true;
    }

    return false;
}

fn struct(struct) {
    let new_struct = #{};
    new_struct.vis = parse_vis(struct.vis);
    new_struct.name = as_pascal_case(struct.name);

    // check if struct is repr(C)
    let is_repr = false;
    for attr in struct.attrs {
        if attr == "repr(C)" { is_repr = true; }
    }

    if !is_repr {
        throw "struct must have attribute '#[repr(C)]'";
    }

    new_struct.fields = [];
    for field in struct.fields {
        let new_field = #{};
        let name = as_camel_case(field.name);
        if is_keyword(name) {
            throw `cannot use field name '${name}' because it is also a csharp keyword`;
        }

        new_field.name = name;
        new_field.vis = parse_vis(field.vis);
        
        let key = field.type.keys()[0];
        let value = field.type.values()[0];
        switch key {
            "named" => new_field.type = parse_type(value),
            "mutptr" => {
                let key = value.keys()[0];
                let value = value.values()[0];
                switch key {
                    "named" => new_field.type = `${parse_type(value)}*`,
                    _ => throw `structs dont support nested pointers. field: '${name}'`,
                }
            },
            "constptr" => {
                let key = value.keys()[0];
                let value = value.values()[0];
                switch key {
                    "named" => new_field.type = `${parse_type(value)}*`,
                    _ => throw `structs dont support nested pointers. field: '${name}'`,
                }
            },
            _ => throw `structs dont support reference types. field: '${name}'`,
        }

        new_struct.fields += new_field;
    }

    return new_struct;
}

fn function(func) {
    let new_func = #{};
    new_func.vis = parse_vis(func.vis);
    new_func.original_name = func.name;
    new_func.name = as_pascal_case(func.name);

    // check if function has extern "C"
    if func.abi != "\"C\"" {
        throw "function abi must be extern \"C\"";
    }

    // check if function has no_mangle
    let no_mangle = false;
    for attr in func.attrs {
        if attr == "no_mangle" { no_mangle = true; }
    }

    if !no_mangle {
        throw "function must have attribute '#[no_mangle]'";
    }

    if func.output != () {
        let key = func.output.keys()[0];
        let value = func.output.values()[0];
        switch key {
            "named" => new_func.output = parse_type(value),
            "mutptr" => {
                let key = value.keys()[0];
                let value = value.values()[0];
                switch key {
                    "named" => new_func.output = `${parse_type(value)}*`,
                    _ => throw `functions dont support nested pointer outputs.`,
                }
            },
            "constptr" => {
                let key = value.keys()[0];
                let value = value.values()[0];
                switch key {
                    "named" => new_func.output = `${parse_type(value)}*`,
                    _ => throw `functions dont support nested pointer outputs.`,
                }
            },
            _ => throw `functions dont support reference outputs.`,
        }
    }

    new_func.inputs = [];
    for input in func.inputs {
        let new_input = #{};
        let name = as_camel_case(input.name);
        if is_keyword(name) {
            throw `cannot use parameter name '${name}' because it is also a csharp keyword`;
        }

        new_input.name = name;
        let key = input.type.keys()[0];
        let value = input.type.values()[0];
        switch key {
            "named" => new_input.type = parse_type(value),
            "ref" => {
                let key = value.keys()[0];
                let value = value.values()[0];
                switch key {
                    "named" => new_input.type = `in ${parse_type(value)}`,
                    _ => throw `functions dont support nested reference inputs. field: '${name}'`,
                }
            }
            "refmut" => {
                let key = value.keys()[0];
                let value = value.values()[0];
                switch key {
                    "named" => new_input.type = `ref ${parse_type(value)}`,
                    _ => throw `functions dont support nested reference inputs. field: '${name}'`,
                }
            }
            "mutptr" => {
                let key = value.keys()[0];
                let value = value.values()[0];
                switch key {
                    "named" => new_input.type = `${parse_type(value)}*`,
                    _ => throw `functions dont support nested pointer inputs. field: '${name}'`,
                }
            },
            "constptr" => {
                let key = value.keys()[0];
                let value = value.values()[0];
                switch key {
                    "named" => new_input.type = `${parse_type(value)}*`,
                    _ => throw `functions dont support nested pointer inputs. field: '${name}'`,
                }
            },
            _ => throw `functions dont support reference inputs.`,
        }

        new_func.inputs += new_input;
    }

    return new_func;
}

fn parse_vis(vis) {
    if vis { "public" } else { "private" }
}

fn parse_type(type) {
    if type == "u8" { return "byte"; }
    if type == "u16" { return "ushort"; }
    if type == "u32" { return "uint"; }
    if type == "u64" { return "ulong"; }
    if type == "u128" { throw "u128 is not supported"; }
    if type == "usize" { return "nuint"; }
    if type == "i8" { return "sbyte"; }
    if type == "i16" { return "short"; }
    if type == "i32" { return "int"; }
    if type == "i64" { return "long"; }
    if type == "i128" { throw "i128 is not supported"; }
    if type == "isize" { return "nint"; }
    if type == "f32" { return "float"; }
    if type == "f64" { return "double"; }
    if type == "char" { throw "char is not supported"; }
    if type == "bool" { return "bool"; }
    
    return type;
}

fn impl(impl) {
    let new_impl = #{};
    new_impl.name = as_pascal_case(impl.name);
    
    new_impl.functions = [];
    for implfn in impl.functions {
        let new_implfn = #{};
        new_implfn.has_self = implfn.has_self;
        new_implfn.vis = parse_vis(implfn.vis);
        new_implfn.alias = as_pascal_case(implfn.alias);
        new_implfn.function = function(implfn.function);
        new_impl.functions += new_implfn;
    }

    return new_impl;
}
