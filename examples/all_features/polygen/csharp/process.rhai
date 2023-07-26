fn struct(struct) {
    // fail on unit structs
    if struct.fields.contains("unit") {
        throw "unit structs are not supported";
    }

    // TODO: allow for align and packed. csharp seems to have the tools to make this happen
    // reject strust if it specifies alignment or packed
    // anything else will be caught by the compiler for being ffi unsafe
    for attr in struct.attrs {
        for token in attr.meta?.list?.tokens {
            if token.ident == "align" { throw "structs must not have an 'align' specified"; }
            if token.ident == "packed" { throw "structs must not have 'packed' specified"; }
        }
    }
    
    let s = #{};
    s.vis = parse_vis(struct.vis);
    s.name = as_pascal_case(struct.ident);

    s.fields = [];
    if struct.fields.contains("named") {
        for field in struct.fields.named {
            let f = #{};
            f.vis = parse_vis(field.vis);
            f.name = as_camel_case(field.ident);
            f.type = parse_type(field.ty.path.segments[-1].ident);
            s.fields += f;
        }
    }

    if struct.fields.contains("unnamed") {
        for (field, i) in struct.fields.unnamed {
            let f = #{};
            f.vis = parse_vis(field.vis);
            f.name = "x" + i;
            f.type = parse_type(field.ty.path.segments[-1].ident);
            s.fields += f;
        }
    }

    return s;
}

fn function(func) {
    let f = #{};
    f.vis = parse_vis(func.vis);
    f.original_name = func.ident;
    f.name = as_pascal_case(func.ident);

    f.inputs = [];
    for (input, index) in func.inputs {
        if !input.contains("typed") { continue; }
        let i = #{};

        let ident = input.typed.pat.ident;
        if ident != () {
            i.name = as_camel_case(ident.ident);
        } else {
            i.name = `_`;
        }

        i.type = parse_type(input.typed.ty.path.segments[-1].ident);
        f.inputs += i;
    }

    if func.contains("output") {
        f.output = parse_type(func.output.path.segments[-1].ident);
    } else {
        f.output = "void";
    }

    return f;
}

fn parse_vis(vis) {
    if vis == "pub" { "public" } else { "private" }
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
