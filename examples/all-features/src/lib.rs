use polygen::{items::types::Opaque, polygen};

#[polygen]
pub struct TestStruct(u32, u64);

#[polygen]
pub struct TestStruct2 {
    _test: TestStruct,
}

#[polygen]
pub struct TestStruct3 {
    _test: u64,
}

#[polygen]
pub fn test(_thing: Opaque<TestStruct3>, _thing2: sub_module::TestStruct) -> TestStruct2 {
    todo!()
}

pub mod sub_module {
    use super::*;

    #[polygen]
    pub struct TestStruct(u32, u64);

    #[polygen]
    pub fn test(_thing: TestStruct, thing2: TestStruct2) -> TestStruct2 {
        thing2
    }
}
