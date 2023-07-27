use polygen::polygen;

#[polygen]
pub struct NormalStruct {
    item: u32,
    another_item: bool,
}

#[polygen]
pub fn create_struct() -> NormalStruct {
    NormalStruct {
        item: 42,
        another_item: true,
    }
}

#[polygen]
pub fn get_item(normal_struct: NormalStruct) -> u32 {
    normal_struct.item
}