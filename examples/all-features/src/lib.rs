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
pub fn test(_thing: &TestStruct, _thing2: sub_module::TestStruct) -> &TestStruct2 {
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

#[polygen]
impl TestStruct {
    pub fn new() -> Self {
        TestStruct(42, 42)
    }

    pub fn new_with(item: &&u32) -> TestStruct {
        Self(**item, 42)
    }

    pub fn get_item(&self) -> u32 {
        self.0
    }

    pub fn set_item(&mut self, item: u32) {
        self.0 = item
    }

    pub fn builder(&mut self) -> &mut Self {
        self
    }

    pub fn also_builder(self) -> Self {
        self
    }
}
