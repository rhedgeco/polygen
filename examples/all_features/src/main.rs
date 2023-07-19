use polygen::polygen;

#[polygen]
#[repr(C)]
struct Test {
    pub test: u32,
}

#[polygen]
#[repr(C)]
pub struct Test2 {
    test: u32,
    pub testy: bool,
    woah: usize,
}

#[polygen]
#[repr(C)]
pub struct Test3 {
    test: u32,
    pub test2: Test2,
}

fn main() {
    let _ = Test { test: 5 };
    let test2 = Test2 {
        test: 5,
        testy: true,
        woah: 2,
    };
    let _ = Test3 { test: 5, test2 };
}
