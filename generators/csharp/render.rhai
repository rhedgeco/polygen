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

fn struct(struct, inner) {
    let tmp = docformat(`
        [StructLayout(LayoutKind.Sequential)]
        ${struct.vis} partial struct ${struct.name}
        {
        polygen-inner
        }
    `);

    return replace(tmp, "polygen-inner", indent(4, inner));
}

fn struct_field(field) {
    docformat(`
        ${field.vis} ${field.type} ${field.name};
    `)
}

fn struct_function(lib_name, implfn) {
    let inputs = utils::render_each(implfn.function.inputs, ", ", function_input);
    let pass_inputs = utils::render_each(implfn.function.inputs, ", ", |input| input.name);
    let _static = if implfn.has_self { "" } else { " static" };
    
    let tmp = docformat(`
    polygen-extern
    ${implfn.vis}${_static} ${implfn.function.output} ${implfn.alias}(${inputs}) => ${implfn.function.name}(${pass_inputs});
    `);
    
    return replace(tmp, "polygen-extern", function(lib_name, implfn.function))
}

fn function(lib_name, func) {
    let inputs = utils::render_each(func.inputs, ", ", function_input);

    return docformat(`
        [DllImport(${lib_name}, EntryPoint = "${func.original_name}", CallingConvention = CallingConvention.Cdecl)]
        ${func.vis} static extern ${func.output} ${func.name}(${inputs});
    `);
}

private fn function_input(input) {
    return `${input.type} ${input.name}`;
}
