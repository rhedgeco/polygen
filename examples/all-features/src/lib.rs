use polygen::{
    items::types::{OpaquePtr, PolyPtr},
    polygen,
};
use sub_module::TestStruct2;

#[polygen]
pub struct TestStruct {
    x0: u32,
    x1: u64,
}

#[polygen]
pub fn execute(item: TestStruct2) {
    drop(item)
}

#[polygen]
pub fn get_u32(item: TestStruct) -> u32 {
    item.x0
}

#[polygen]
pub fn create_opaque(item: u32) -> OpaquePtr {
    OpaquePtr::new(TestStruct { x0: item, x1: 42 })
}

#[polygen]
pub fn create_ptr(val: u64) -> PolyPtr<TestStruct2> {
    PolyPtr::new(TestStruct2 {
        item: TestStruct { x0: 42, x1: val },
    })
}

#[polygen]
pub fn change_item(mut item: PolyPtr<TestStruct2>, val: u64) {
    item.item.x1 = val
}

pub mod sub_module {
    use polygen::polygen;

    use crate::TestStruct;

    #[polygen]
    pub struct TestStruct2 {
        pub(crate) item: TestStruct,
    }

    #[polygen]
    pub fn sub_module_function(item: TestStruct) -> u32 {
        item.x0
    }
}
