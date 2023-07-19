fn item(item) {
    if item.struct != () { return struct(item.struct); }
    throw "item is unsupported";
}

fn struct(struct) {
    let s = #{};
    s.type = "struct";
    s.vis = parse_vis(struct.vis);
    s.name = as_pascal_case(struct.ident);
    s.fields = [];

    for field in struct.fields.named {
        let f = #{};
        f.vis = parse_vis(field.vis);
        f.name = as_camel_case(field.ident);
        f.type = parse_type(field.ty.path.segments[-1].ident);
        s.fields += f;
    }

    return s;
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
    
    return type;
}
