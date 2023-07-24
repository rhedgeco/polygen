fn default_imports() {
    docformat(`
        using System;
        using System.Runtime.InteropServices;
    `)
}

fn namespace(name, inner) {
    let tmp = docformat(`
        namespace ${as_pascal_case(name)}
        {
        polygen-inner
        }
    `);

    return replace(tmp, "polygen-inner", indent(4, inner));
}

fn static_class(name, inner) {
    let tmp = docformat(`
        public static partial class ${as_pascal_case(name)}
        {
        polygen-inner
        }
    `);

    return replace(tmp, "polygen-inner", indent(4, inner));
}

fn item(item) {
    if item.type == "struct" { return struct(item); }
    else { return `// Item ${item.type} is unimplemented.` }
}


fn struct(struct) {
    let tmp = docformat(`
        [StructLayout(LayoutKind.Sequential)]
        ${struct.vis} struct ${struct.name}
        {
        polygen-inner
        }
    `);
    
    let fields = each(struct.fields, "\n", struct_field);
    return replace(tmp, "polygen-inner", indent(4, fields));
}

fn struct_field(field) {
    docformat(`
        ${field.vis} ${field.type} ${field.name};
    `)
}

fn each(items, seperator, function) {
    if items.len() == 0 { return ""; }
    let output = "";
    output += function.call(items[0]);
    for i in 1..items.len() {
        let item = items[i];
        output += seperator + function.call(item);
    }
    return output;
}
