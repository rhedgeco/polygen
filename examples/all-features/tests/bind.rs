use std::fs;

use all_features::{change_item, create_opaque, create_ptr, execute, get_u32, sub_module};
use polygen::PolyBag;
use polygen_csharp::PolygenCSharp;

#[test]
fn bind() {
    let csharp = PolygenCSharp {
        lib_name: format!("all_features"),
        namespace: format!("AllFeatures"),
        bag: PolyBag::new("Native")
            .register_function::<execute>()
            .register_function::<get_u32>()
            .register_function::<create_opaque>()
            .register_function::<create_ptr>()
            .register_function::<change_item>()
            .register_function::<sub_module::sub_module_function>(),
    };

    fs::create_dir_all("./target/polygen").unwrap();
    fs::write("./target/polygen/AllFeatures.cs", csharp.generate()).unwrap();
}
