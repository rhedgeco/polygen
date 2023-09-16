use polygen::{
    items::types::{OpaquePtr, PolyBox},
    polygen,
};

#[polygen]
pub struct TestStruct {
    x0: u32,
    x1: u64,
}

#[polygen]
impl TestStruct {
    pub fn new() -> Self {
        Self { x0: 42, x1: 42 }
    }

    pub fn new_with(val: u32) -> Self {
        Self {
            x0: val,
            x1: val.into(),
        }
    }

    pub fn read(&self) -> u32 {
        self.x0
    }

    pub fn modify(&mut self, val: u32) {
        self.x0 = val;
        self.x1 = val.into();
    }

    pub fn duplicate(&self) -> Self {
        Self {
            x0: self.x0,
            x1: self.x1,
        }
    }

    pub fn convert(mut self, val: u32) -> Self {
        self.x0 = val;
        self.x1 = val.into();
        self
    }
}

#[polygen]
pub struct TestStruct2 {
    nested: sub_module::TestStruct2,
}

#[polygen]
pub fn pointer_test(_input: *mut TestStruct) -> *mut *const TestStruct2 {
    todo!()
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
pub fn create_ptr(val: u64) -> PolyBox<sub_module::TestStruct2> {
    PolyBox::new(sub_module::TestStruct2 {
        item: TestStruct { x0: 42, x1: val },
    })
}

#[polygen]
pub fn change_item(mut item: PolyBox<sub_module::TestStruct2>, val: u64) {
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
