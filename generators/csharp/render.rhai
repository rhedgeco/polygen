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
        public static class ${as_pascal_case(name)}
        {
        polygen-inner
        }
    `);

    return replace(tmp, "polygen-inner", indent(4, inner));
}

fn struct(struct, inner) {
    let tmp = docformat(`
        [StructLayout(LayoutKind.Sequential)]
        ${struct.vis} struct ${struct.name}
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
    let unsafe_keyword = if implfn.function.is_unsafe { " unsafe" } else { "" };
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
        let pass_input = "";
        if input.type.starts_with("in ") { pass_input = "in " };
        if input.type.starts_with("ref ") { pass_input = "ref " };
        if input.type.ends_with("*") { unsafe_keyword = " unsafe" };
        pass_input += if input.name == "__polygen_impl_self" {
            "this"
        } else {
            input.name
        };

        pass_input
    });

    // create the member function
    let output = implfn.function.output;
    if output == () { output = "void"; }
    else if output.ends_with("*") { unsafe_keyword = " unsafe"; }
    let member_fn = `${implfn.vis}${static_keyword}${unsafe_keyword} ${output} ${implfn.alias}(${inputs}) => ${implfn.function.name}(${pass_inputs});`;
    
    return extern_fn + "\n" + member_fn;
}

fn function(lib_name, func) {
    let inputs = utils::render_each(func.inputs, ", ", function_input);
    let unsafe_keyword = if func.is_unsafe { " unsafe" } else { "" };
    let output = func.output;
    if output == () { output = "void"; }
    else if output.ends_with("*") { unsafe_keyword = " unsafe"; }

    return docformat(`
        [DllImport("${lib_name}", EntryPoint = "${func.original_name}", CallingConvention = CallingConvention.Cdecl)]
        ${func.vis} static${unsafe_keyword} extern ${output} ${func.name}(${inputs});
    `);
}

private fn function_input(input) {
    return `${input.type} ${as_camel_case(input.name)}`;
}
