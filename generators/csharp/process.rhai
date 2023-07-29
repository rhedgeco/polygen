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
    struct.vis = parse_vis(struct.vis);
    struct.name = as_pascal_case(struct.name);

    for i in 0..struct.fields.len() {
        let name = as_camel_case(struct.fields[i].name);
        if is_keyword(name) {
            throw `cannot use field name '${name}' because it is also a csharp keyword`;
        }

        struct.fields[i].name = name;
        struct.fields[i].vis = parse_vis(struct.fields[i].vis);
        struct.fields[i].type.name = parse_type(struct.fields[i].type.name);
    }

    return struct;
}

fn function(func) {
    func.vis = parse_vis(func.vis);
    func.original_name = func.name;
    func.name = as_pascal_case(func.name);

    if func.output != () {
        func.output.name = parse_type(func.output.name);
    }

    for (input, i) in func.inputs {
        let name = as_camel_case(input.name);
        if is_keyword(name) {
            throw `cannot use parameter name '${name}' because it is also a csharp keyword`;
        }

        func.inputs[i].name = name;
        func.inputs[i].type.name = parse_type(input.type.name);
    }

    return func;
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
