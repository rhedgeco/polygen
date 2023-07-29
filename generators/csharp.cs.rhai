import "utils/utils" as utils;
import "csharp/render" as render;
import "csharp/process" as process;

fn process(item) {
    if this.structs == () { this.structs = #{}; }
    if this.functions == () { this.functions = #{}; }

    let key = item.keys()[0];
    let value = item.values()[0];

    if key == "Struct" {
        if this.structs.contains(value.name) {
            throw `Cannot export multiple structs with name '${value.name}'`;
        }

        this.structs[value.name] = process::struct(value);
    } else if key == "Fn" { 
        if this.functions.contains(value.name) {
            throw `Cannot export multiple functions with name '${value.name}'`;
        }

        this.functions[value.name] = process::function(value);
    } else { throw "This item is unsupported by c-sharp"; }
}

fn render() {
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
