use polygen::polygen;

#[polygen]
#[repr(C)]
pub struct NormalStruct {
    item: u32,
    another_item: bool,
}

#[polygen]
#[repr(C)]
pub struct TestStruct {
    floater: *mut f64,
    another_float: f32,
}

#[polygen]
#[repr(C)]
pub struct AnotherStruct {
    test_struct: TestStruct,
    another_float: *const u8,
}

#[polygen]
#[no_mangle]
pub extern "C" fn create_struct() -> NormalStruct {
    NormalStruct {
        item: 42,
        another_item: true,
    }
}

#[polygen]
#[no_mangle]
pub extern "C" fn get_normal_item(normal_struct: NormalStruct) -> u32 {
    normal_struct.item
}

#[polygen]
#[no_mangle]
pub extern "C" fn get_test_float(test_struct: TestStruct) -> *mut f64 {
    test_struct.floater
}

// #[polygen]
impl NormalStruct {
    pub fn new() -> Self {
        Self::new_with(42)
    }

    pub fn new_with(item: u32) -> NormalStruct {
        NormalStruct {
            item,
            another_item: true,
        }
    }

    pub fn get_item(&self) -> u32 {
        self.item
    }

    pub fn get_item_ptr(&mut self) -> *mut u32 {
        (&mut self.item) as *mut u32
    }

    pub fn replace_item(&mut self, new_item: u32) -> u32 {
        std::mem::replace(&mut self.item, new_item)
    }
}
