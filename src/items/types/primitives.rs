// create macro to implement polystruct over primitives
macro_rules! impl_item {
    ($($item:ty),+ $(,)?) => {
        $(
            unsafe impl $crate::__private::ExportedPolyStruct for $item {
                type ExportedType = $item;
                const STRUCT: $crate::items::PolyType = $crate::items::PolyType::Primitive(
                    stringify!($item)
                );
            }
        )+
    };
}

// export all FFI safe primitives
impl_item! {
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
    f32, f64,
    bool,
}
