use std::{fs, path::PathBuf};

use polygen::PolyBag;
use polygen_csharp::CSharpRenderer;
use simple_lib::{set_item, MyStruct};

static OUTPUT_DIR: &str = "target/polygen";

#[test]
fn bind() {
    // clear output folder
    let out_path = PathBuf::from(OUTPUT_DIR);
    if out_path.exists() {
        fs::remove_dir_all(out_path).unwrap();
    }

    // create the PolyBag
    let bag = PolyBag::new("Native")
        .register_impl::<MyStruct>()
        .register_function::<set_item>();

    // render the csharp data to a file
    fs::create_dir_all(OUTPUT_DIR).unwrap();
    fs::write(
        PathBuf::from(OUTPUT_DIR).join("AllFeatures.cs"),
        CSharpRenderer {
            lib_name: "simple_lib".to_string(),
            namespace: "SimpleLib".to_string(),
        }
        .render(&bag),
    )
    .unwrap();
}
