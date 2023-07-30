use polygen::polygen;

#[polygen]
pub struct NormalStruct {
    item: u32,
    another_item: bool,
}

#[polygen]
pub struct TestStruct {
    floater: *mut f64,
    another_float: f32,
}

#[polygen]
pub struct AnotherStruct {
    test_struct: TestStruct,
    another_float: *const u8,
}

#[polygen]
pub fn create_struct() -> NormalStruct {
    NormalStruct {
        item: 42,
        another_item: true,
    }
}

#[polygen]
pub fn get_normal_item(normal_struct: &NormalStruct) -> u32 {
    normal_struct.item
}

#[polygen]
pub fn get_test_float(test_struct: &TestStruct) -> *mut f64 {
    test_struct.floater
}
