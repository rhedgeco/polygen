import "utils/utils" as utils;

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

fn struct(struct) {
    let tmp = docformat(`
        [StructLayout(LayoutKind.Sequential)]
        ${struct.vis} struct ${struct.name}
        {
        polygen-inner
        }
    `);
    
    let fields = utils::render_each(struct.fields, "\n", struct_field);
    return replace(tmp, "polygen-inner", indent(4, fields));
}

private fn struct_field(field) {
    docformat(`
        ${field.vis} ${field.type.name} ${field.name};
    `)
}

fn function(lib_name, func) {
    let inputs = utils::render_each(func.inputs, ", ", function_input);

    return docformat(`
        [DllImport(${lib_name}, EntryPoint = "${func.original_name}", CallingConvention = CallingConvention.Cdecl)]
        ${func.vis} static extern ${func.output.name} ${func.name}(${inputs});
    `);
}

private fn function_input(input) {
    return `${input.type.name} ${input.name}`;
}
