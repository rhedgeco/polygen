use crate::items::PolyType;

pub unsafe trait ExportedPolyType: Sized {
    type ExportedType: From<Self> + Into<Self>;
    const TYPE: PolyType;
}

unsafe impl<'a, T: ExportedPolyType> ExportedPolyType for &'a T {
    type ExportedType = &'a T;
    const TYPE: PolyType = PolyType::Ref(&T::TYPE);
}

unsafe impl<'a, T: ExportedPolyType> ExportedPolyType for &'a mut T {
    type ExportedType = &'a mut T;
    const TYPE: PolyType = PolyType::RefMut(&T::TYPE);
}

unsafe impl<T: ExportedPolyType> ExportedPolyType for *mut T {
    type ExportedType = *mut T;
    const TYPE: PolyType = PolyType::PtrMut(&T::TYPE);
}

unsafe impl<T: ExportedPolyType> ExportedPolyType for *const T {
    type ExportedType = *const T;
    const TYPE: PolyType = PolyType::PtrConst(&T::TYPE);
}

// create macro to implement polystruct over primitives
macro_rules! impl_item {
    ($( $item:ty),+ $(,)?) => {
        $(
            unsafe impl $crate::__private::ExportedPolyType for $item {
                type ExportedType = $item;
                const TYPE: $crate::items::PolyType = $crate::items::PolyType::Struct(
                    $crate::items::PolyStruct {
                        ident: stringify!($item),
                        module: "std",
                        fields: &[],
                    }
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
