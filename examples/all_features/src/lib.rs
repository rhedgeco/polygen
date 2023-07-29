use polygen::polygen;

#[polygen]
pub struct NormalStruct {
    pub item: u32,
    another_item: bool,
    pub(crate) third_item: i64,
}

#[polygen]
pub struct TupleStruct(i8, pub i16, usize);

#[polygen]
pub struct MultiStruct {
    item: isize,
    normal_item: NormalStruct,
    tuple_item: TupleStruct,
    nested_item: nested::NestedStruct,
}

#[polygen]
pub extern "C" fn _cool_function(value: i8, _normal_struct: NormalStruct) -> TupleStruct {
    TupleStruct(value, 2, 3)
}

mod nested {
    use super::*;

    #[polygen]
    pub struct NestedStruct {
        field1: i64,
    }
}
