import "utils/utils" as utils;
import "csharp/render" as render;
import "csharp/process" as process;

fn struct(struct) {
    if this.structs == () { this.structs = #{}; }
    if this.structs.contains(struct.name) {
        throw `Cannot export multiple structs with the name '${struct.name}'`;
    }

    this.structs[struct.name] = process::struct(struct);
}

fn function(function) {
    if this.functions == () { this.functions = #{}; }
    if this.functions.contains(function.name) {
        throw `Cannot export multiple functions with the name '${function.name}'`;
    }

    this.functions[function.name] = process::function(function);
}

fn render() {
    // ensure all items exist
    if this.structs == () { this.structs = #{}; }
    if this.functions == () { this.functions = #{}; }

    // create library template
    let library_items = docformat(`
        public const string NativeLib = "${replace(PACKAGE_NAME, "-", "_")}";

        poly-structs

        poly-functions
    `);

    // replace placeholder text with rendered items
    let structs = utils::render_each(this.structs.values(), "\n\n", |struct| render::struct(struct));
    let library_items = replace(library_items, "poly-structs", structs);
    let functions = utils::render_each(this.functions.values(), "\n\n", |func| render::function("NativeLib", func));
    let library_items = replace(library_items, "poly-functions", functions);

    // render final output template
    let output = docformat(`
        using System;
        using System.Runtime.InteropServices;

        poly-static-class
    `);

    // replace placeholder text with rendered static class
    let static_class = render::static_class(PACKAGE_NAME, library_items);
    let output = replace(output, "poly-static-class", static_class);

    // return rendered file with a newline
    return output + "\n";
}
