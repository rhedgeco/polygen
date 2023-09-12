use polygen::{items::types::OpaquePtr, polygen};

#[polygen]
pub struct TestStruct {
    x0: u32,
    x1: u64,
}

#[polygen]
pub struct TestStruct2 {
    item: TestStruct,
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
pub fn change_item(item: *mut TestStruct2, val: u64) {
    unsafe { (*item).item.x1 = val }
}
