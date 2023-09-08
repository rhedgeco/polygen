use std::fs;

use all_features::test;
use polygen::PolyBag;
use polygen_csharp::PolygenCSharp;

#[test]
fn bind() {
    let csharp = PolygenCSharp {
        lib_name: format!("all_features"),
        namespace: format!("AllFeatures"),
        bag: PolyBag::new("Native").register_function::<test>(),
    };

    fs::create_dir_all("./target/polygen").unwrap();
    fs::write("./target/polygen/AllFeatures.cs", csharp.generate()).unwrap();
}
