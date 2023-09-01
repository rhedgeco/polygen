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
pub fn test(_thing: TestStruct) -> TestStruct2 {
    todo!()
}

#[polygen]
pub fn test2(_: TestStruct, _another: TestStruct3) {
    todo!()
}

#[polygen]
pub fn test3(_thing: TestStruct) -> TestStruct3 {
    todo!()
}
