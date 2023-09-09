use polygen::{items::types::OpaquePtr, polygen};

#[polygen]
pub struct Nested<Wowzers> {
    item: u32,
    gen: Wowzers,
}

#[polygen]
pub fn convert(nested: Nested<u64>, other: u32, _thing: TestStruct2) -> Nested<u32> {
    Nested {
        item: nested.item,
        gen: other,
    }
}

#[polygen]
pub struct TestStruct {
    x0: u32,
    x1: u64,
}

#[polygen]
pub struct TestStruct2 {
    _test: TestStruct,
}

pub struct TestStruct3 {
    _test: u64,
}

#[polygen]
pub fn test(_thing: OpaquePtr<TestStruct3>, _thing2: sub_module::TestStruct) -> TestStruct2 {
    todo!()
}

pub mod sub_module {
    use super::*;

    #[polygen]
    pub struct TestStruct {
        x0: u32,
        x1: u64,
    }

    #[polygen]
    pub fn test(_thing: TestStruct, thing2: TestStruct2) -> TestStruct2 {
        thing2
    }
}
