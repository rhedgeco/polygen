use polygen::polygen;

#[polygen]
#[repr(C)]
pub struct NormalStruct {
    item: u32,
    another_item: bool,
}

#[polygen]
#[no_mangle]
pub extern "C" fn create_struct() -> NormalStruct {
    NormalStruct {
        item: 42,
        another_item: true,
    }
}

#[polygen]
#[no_mangle]
pub extern "C" fn get_item(normal_struct: &NormalStruct) -> u32 {
    normal_struct.item
}
