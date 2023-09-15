use std::ops::{Deref, DerefMut};

use crate::{
    __private::ExportedPolyStruct,
    items::{PolyGeneric, PolyStruct, PolyType, StructField},
};

#[repr(C)]
pub struct PolyBox<T: ExportedPolyStruct> {
    ptr: *mut T,
}

impl<T: ExportedPolyStruct> PolyBox<T> {
    pub fn new(item: T) -> Self {
        Box::new(item).into()
    }
}

impl<T: ExportedPolyStruct> Deref for PolyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T: ExportedPolyStruct> DerefMut for PolyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T: ExportedPolyStruct> From<Box<T>> for PolyBox<T> {
    fn from(value: Box<T>) -> Self {
        Self {
            ptr: Box::into_raw(value),
        }
    }
}

unsafe impl<T: ExportedPolyStruct> ExportedPolyStruct for PolyBox<T> {
    type ExportedType = PolyBox<T>;
    const STRUCT: PolyType = PolyType::Struct(PolyStruct {
        module: "::polygen",
        name: "PolyBox",
        fields: &[StructField {
            name: "ptr",
            ty: crate::items::FieldType::Typed(&<usize as ExportedPolyStruct>::STRUCT),
        }],
        generics: &[PolyGeneric {
            ident: "T",
            ty: &<T as ExportedPolyStruct>::STRUCT,
        }],
    });
}
