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
pub struct MultiStruct {
    item: isize,
    normal_item: NormalStruct,
    tuple_item: TupleStruct,
    nested_item: nested::NestedStruct,
}

#[polygen]
#[repr(transparent)]
#[allow(dead_code)]
struct TransparentStruct {
    pub value: MultiStruct,
}

#[polygen]
extern "C" fn _cool_function(value: i8, _normal_struct: NormalStruct) -> TupleStruct {
    TupleStruct(value, 2, 3)
}

mod nested {
    use super::*;

    #[polygen]
    #[repr(C)]
    pub struct NestedStruct {
        field1: i64,
    }
}
