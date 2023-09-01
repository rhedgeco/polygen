use polygen::polygen;

#[polygen]
pub struct TestStruct(u32, u64);

#[polygen]
pub struct TestStruct2 {
    _test: &'static TestStruct,
}

pub struct TestStruct3 {
    _test: u64,
}

#[polygen]
pub fn test(_thing: TestStruct, thing2: TestStruct2) -> TestStruct2 {
    thing2
}
