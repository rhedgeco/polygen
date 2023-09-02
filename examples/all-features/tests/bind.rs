use all_features::test;
use polygen::PolyBag;
use polygen_csharp::PolygenCSharp;

#[test]
fn bind() {
    let data = PolygenCSharp {
        lib_name: format!("all_features"),
        namespace: format!("AllFeatures"),
        bag: PolyBag::new("Native").register_function::<test>(),
    }
    .generate();

    panic!("\n{data}\n");
}
