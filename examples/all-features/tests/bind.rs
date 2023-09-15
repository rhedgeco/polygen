use std::{fs, path::PathBuf};

use all_features::{change_item, create_opaque, create_ptr, execute, get_u32, sub_module};
use polygen::PolyBag;
use polygen_tera::PolyTera;

static INPUT_GLOB: &str = "templates/**/*";
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
        .register_function::<execute>()
        .register_function::<get_u32>()
        .register_function::<create_opaque>()
        .register_function::<create_ptr>()
        .register_function::<change_item>()
        .register_function::<sub_module::sub_module_function>();

    // render the bag with PolyTera
    PolyTera::new(INPUT_GLOB)
        .unwrap()
        .render(OUTPUT_DIR, bag)
        .unwrap();
}
