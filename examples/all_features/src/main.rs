use polygen::polygen;

#[polygen]
#[repr(C)]
#[allow(dead_code)]
struct NormalStruct {
    pub item: u32,
    another_item: bool,
    pub(crate) third_item: i64,
}

#[polygen]
#[repr(C)]
#[allow(dead_code)]
struct TupleStruct(i8, pub i16, usize);

#[polygen]
#[repr(C)]
#[allow(dead_code)]
struct MultiStruct {
    item: isize,
    normal_item: NormalStruct,
    tuple_item: TupleStruct,
    nested_item: nested::NestedStruct,
}

mod nested {
    use super::*;

    #[polygen]
    #[repr(C)]
    pub struct NestedStruct {
        field1: i64,
    }
}

fn main() {}
