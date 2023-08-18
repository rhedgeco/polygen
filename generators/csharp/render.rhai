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
    // create extern function
    let extern_fn = function(lib_name, implfn.function);
    let static_keyword = " static";

    // filter the inputs for self parameters
    let filtered_inputs = [];
    for input in implfn.function.inputs {
        if input.name != "__polygen_impl_self" {
            filtered_inputs += input;
        } else {
            static_keyword = "";
        }
    }

    // render all function inputs
    let inputs = utils::render_each(filtered_inputs, ", ", function_input);
    let pass_inputs = utils::render_each(implfn.function.inputs, ", ", |input| {
        if input.name == "__polygen_impl_self" {
            "this"
        } else {
            input.name
        }
    });

    // create the member function
    let member_fn = `${implfn.vis}${static_keyword} ${implfn.function.output} ${implfn.alias}(${inputs}) => ${implfn.function.name}(${pass_inputs});`;
    
    return extern_fn + "\n" + member_fn;
}

fn function(lib_name, func) {
    let inputs = utils::render_each(func.inputs, ", ", function_input);

    return docformat(`
        [DllImport(${lib_name}, EntryPoint = "${func.original_name}", CallingConvention = CallingConvention.Cdecl)]
        ${func.vis} static extern ${func.output} ${func.name}(${inputs});
    `);
}

private fn function_input(input) {
    return `${input.type} ${as_camel_case(input.name)}`;
}
