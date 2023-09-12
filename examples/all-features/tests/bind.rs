use std::fs;

use all_features::{change_item, create_opaque, get_u32};
use polygen::PolyBag;
use polygen_csharp::PolygenCSharp;

#[test]
fn bind() {
    let csharp = PolygenCSharp {
        lib_name: format!("all_features"),
        namespace: format!("AllFeatures"),
        bag: PolyBag::new("Native")
            .register_function::<get_u32>()
            .register_function::<create_opaque>()
            .register_function::<change_item>(),
    };

    fs::create_dir_all("./target/polygen").unwrap();
    fs::write("./target/polygen/AllFeatures.cs", csharp.generate()).unwrap();
}
