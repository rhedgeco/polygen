// marker trait for if a type has been exported by polygen
#[allow(non_camel_case_types)]
pub unsafe trait exported_by_polygen {}

macro_rules! impl_export {
    ($($t:ty),*) => {
        $(
            unsafe impl $crate::exported_by_polygen for $t {}
        )*
    };
}

impl_export!(f32, f64, char, bool);
impl_export!(u8, u16, u32, u64, u128, usize);
impl_export!(i8, i16, i32, i64, i128, isize);
