import "csharp/render" as render;
import "csharp/process" as process;

fn process(item) {
    // create struct array if it doesnt exist
    if this.structs == () { this.structs = [] };

    // process item according to its type
    if item.contains("struct") { this.structs += process::struct(item.struct); }
    else { throw `'${item.keys()[0]}' items are not supported`; }
}

fn render() {
    // render all structs
    let structs = render::each(this.structs, "\n\n", |struct| render::struct(struct));

    // render a static class to hold the items
    let static_class = render::static_class(PACKAGE_NAME, structs);

    // render the default imports
    let imports = render::default_imports();

    // combine the imports and class for the final render
    return imports + "\n\n" + static_class + "\n";
}
