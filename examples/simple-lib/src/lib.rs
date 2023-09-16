use polygen::{items::types::PolyBox, polygen};

#[polygen]
pub struct MyStruct {
    item: u32,
    another_item: u64,
}

#[polygen]
pub fn set_item(mut boxed: PolyBox<MyStruct>, item: u32) {
    boxed.item = item;
}
