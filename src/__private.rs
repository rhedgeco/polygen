// marker trait for if a type has been exported by polygen
#[allow(non_camel_case_types)]
pub unsafe trait exported_by_polygen {}

// create macro to implement many types
macro_rules! impl_export {
    ($($t:ty),*) => {
        $(
            unsafe impl $crate::__private::exported_by_polygen for $t {}
        )*
    };
}

// implement for all primitives
impl_export!(f32, f64, char, bool);
impl_export!(u8, u16, u32, u64, u128, usize);
impl_export!(i8, i16, i32, i64, i128, isize);

// implement for all reference and pointer exports
unsafe impl<T: exported_by_polygen> exported_by_polygen for &T {}
unsafe impl<T: exported_by_polygen> exported_by_polygen for &mut T {}
unsafe impl<T: exported_by_polygen> exported_by_polygen for *mut T {}
unsafe impl<T: exported_by_polygen> exported_by_polygen for *const T {}
