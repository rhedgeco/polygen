use polygen::polygen;

#[polygen]
mod bindings {
    #[repr(C)]
    #[allow(dead_code)]
    struct NormalStruct {
        pub item: u32,
        another_item: bool,
        pub(crate) third_item: i64,
    }

    #[repr(C)]
    #[allow(dead_code)]
    struct TupleStruct(i8, pub i16, usize);

    #[repr(C)]
    #[allow(dead_code)]
    struct MultiStruct {
        normal_item: NormalStruct,
        tuple_item: TupleStruct,
    }
}

fn main() {}
