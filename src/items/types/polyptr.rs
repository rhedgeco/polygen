use std::ops::{Deref, DerefMut};

use crate::{
    __private::ExportedPolyStruct,
    items::{PolyGeneric, PolyStruct, PolyType, StructField},
};

#[repr(C)]
pub struct PolyPtr<T: ExportedPolyStruct> {
    ptr: *mut T,
}

impl<T: ExportedPolyStruct> PolyPtr<T> {
    pub fn new(item: T) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(item)),
        }
    }
}

impl<T: ExportedPolyStruct> Deref for PolyPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T: ExportedPolyStruct> DerefMut for PolyPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

unsafe impl<T: ExportedPolyStruct> ExportedPolyStruct for PolyPtr<T> {
    type ExportedType = PolyPtr<T>;
    const STRUCT: PolyType = PolyType::Struct(PolyStruct {
        module: "::polygen",
        name: stringify!(PolyPtr),
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
