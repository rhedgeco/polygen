import "csharp/render" as render;
import "csharp/process" as process;

fn process(item) {
    return process::item(item);
}

fn render(name, items) {
    // render every item sequentially with two return characters inbetween
    let items = render::each(items, "\n\n", |item| render::item(item));

    // render a static class with all the items inside of it
    let class = render::static_class(name, items);

    // render a namespace and put the static class inside it
    let namespace = render::namespace("RustNative", class);

    // render the default imports for the top of the file
    let imports = render::default_imports();

    // merge and return the imports and namespace
    return imports + "\n\n" + namespace;
}
