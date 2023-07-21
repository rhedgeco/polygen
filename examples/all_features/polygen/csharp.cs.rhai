import "csharp/render" as render;
import "csharp/process" as process;

fn process(item) {
    if item.struct != () { return process::struct(item.struct); }
    else { throw `'${item.keys()[0]}' items are not supported`; }
}

fn render(items) {
    // render all structs
    let structs = render::each(items.structs, "\n\n", |struct| render::struct(struct));

    // render a static class to hold the items
    let static_class = render::static_class(PACKAGE_NAME, structs);

    // render the default imports
    let imports = render::default_imports();

    // combine the imports and class for the final render
    return imports + "\n\n" + static_class + "\n";
}
