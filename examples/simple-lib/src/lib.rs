use polygen::{items::types::PolyBox, polygen};

#[polygen]
pub struct MyStruct {
    item: u32,
    another_item: u64,
}

#[polygen]
impl MyStruct {
    pub fn new_with(item: u32) -> Self {
        Self {
            item,
            another_item: 42,
        }
    }
}

#[polygen]
pub fn create_boxed(item: MyStruct) -> PolyBox<MyStruct> {
    PolyBox::new(item)
}

#[polygen]
pub fn set_item(mut boxed: PolyBox<MyStruct>, item: u32) {
    boxed.item = item;
}
