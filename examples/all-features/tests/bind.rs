use std::{fs, path::PathBuf};

use all_features::{
    change_item, create_opaque, create_ptr, execute, get_u32, pointer_test, sub_module, TestStruct,
};
use polygen::PolyBag;
use polygen_csharp::CSharpRenderer;

static OUTPUT_DIR: &str = "target/polygen";

#[test]
fn bind() {
    // remove all current rendered templates
    let out_path = PathBuf::from(OUTPUT_DIR);
    if out_path.exists() {
        fs::remove_dir_all(out_path).unwrap();
    }

    // create the PolyBag
    let bag = PolyBag::new("Native")
        .register_impl::<TestStruct>()
        .register_function::<pointer_test>()
        .register_function::<execute>()
        .register_function::<get_u32>()
        .register_function::<create_opaque>()
        .register_function::<create_ptr>()
        .register_function::<change_item>()
        .register_function::<sub_module::sub_module_function>();

    // render the csharp data to a file
    fs::create_dir_all(OUTPUT_DIR).unwrap();
    fs::write(
        PathBuf::from(OUTPUT_DIR).join("AllFeatures.cs"),
        CSharpRenderer {
            lib_name: "all_features".to_string(),
            namespace: "AllFeatures".to_string(),
        }
        .render(&bag),
    )
    .unwrap();
}
