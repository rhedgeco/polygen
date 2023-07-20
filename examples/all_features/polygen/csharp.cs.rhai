import "csharp/render" as render;
import "csharp/process" as process;

fn build(item) {
    // throw an error if the item being built is not a module
    if !item.contains("mod") { throw "only modules are supported"; }

    // process the module for use in rendering
    let mod = process::mod(item.mod);
    
    // render all structs in the module
    let structs = render::each(mod.structs, "\n\n", |struct| render::struct(struct));

    // render a static class to hold all the items in it
    let class = render::static_class(mod.name, structs);

    // render a namespace with the package name
    let namespace = render::namespace(PACKAGE_NAME, class);

    // render the default imports for the top of the file
    let imports = render::default_imports();

    // merge and return the imports and namespace
    return imports + "\n\n" + namespace;
    // return replace(item.to_string(), "#{", "{");
}
