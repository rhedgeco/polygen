use polygen::polygen;

#[polygen]
#[repr(C, align(4096))]
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

fn main() {}
