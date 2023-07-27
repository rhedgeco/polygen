import "csharp/render" as render;
import "csharp/process" as process;

fn process(item) {
    // create struct array if it doesnt exist
    if this.structs == () { this.structs = #{}; }
    if this.functions == () { this.functions = #{}; }

    // process item according to its type
    if item.contains("struct") {
        let struct = item.struct;
        let name = struct.ident;

        // check for overlapping struct names
        if this.structs.contains(name) {
            throw `Cannot export more than one struct with name '${name}'`;
        }

        this.structs[name] = process::struct(struct);
    } else if item.contains("fn") {
        let func = item["fn"];
        let name = func.ident;

        // check for overlapping function names
        if this.functions.contains(name) {
            throw `Cannot export more than one function with name '${name}'`;
        }

        this.functions[name] = process::function(func);
    } else { 
        throw `'${item.keys()[0]}' items are not supported`;
    }
}

fn render() {
    // render constant for library name
    let lib_const = render::constant("NativeLib", "string", `"${replace(PACKAGE_NAME, "-", "_")}"`);

    // render all structs
    let structs = render::each(this.structs.values(), "\n\n", |struct| render::struct(struct));

    // render all functions
    let functions = render::each(this.functions.values(), "\n\n", |func| render::function("NativeLib", func));

    // combine all items
    let items = lib_const + "\n\n" + structs + "\n\n" + functions;

    // render a static class to hold the items
    let static_class = render::static_class(PACKAGE_NAME, items);

    // render the default imports
    let imports = render::default_imports();

    // combine the imports and class for the final render
    return imports + "\n\n" + static_class + "\n";
}
