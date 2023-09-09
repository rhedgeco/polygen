use std::any::TypeId;

use thiserror::Error;

use crate::{
    __private::ExportedPolyStruct,
    items::{PolyField, PolyStruct},
};

#[derive(Debug, Error)]
#[error("Pointer is invalid")]
pub struct InvalidPtr;

#[repr(C)]
pub struct OpaquePtr<T: 'static> {
    id: TypeId,
    ptr: *mut T,
}

impl<T: 'static> OpaquePtr<T> {
    pub fn new(item: T) -> Self {
        Self {
            id: TypeId::of::<T>(),
            ptr: Box::into_raw(Box::new(item)),
        }
    }

    pub fn as_ref(&self) -> Result<&T, InvalidPtr> {
        self.validate_pointer()?;
        Ok(unsafe { &*self.ptr })
    }

    pub fn as_mut(&mut self) -> Result<&mut T, InvalidPtr> {
        self.validate_pointer()?;
        Ok(unsafe { &mut *self.ptr })
    }

    pub fn into_inner(self) -> Result<T, InvalidPtr> {
        self.validate_pointer()?;
        Ok(unsafe { *Box::from_raw(self.ptr) })
    }

    fn validate_pointer(&self) -> Result<(), InvalidPtr> {
        if self.id != TypeId::of::<T>() {
            return Err(InvalidPtr);
        }

        Ok(())
    }
}

unsafe impl<T: 'static> ExportedPolyStruct for OpaquePtr<T> {
    type ExportedType = OpaquePtr<T>;

    const STRUCT: PolyStruct = PolyStruct {
        module: "::polygen",
        name: stringify!(OpaquePtr),
        fields: &[
            PolyField {
                name: "id",
                ty_name: stringify!(TypeId),
                ty: &<TypeId as ExportedPolyStruct>::STRUCT,
            },
            PolyField {
                name: "ptr",
                ty_name: stringify!(usize),
                ty: &<usize as ExportedPolyStruct>::STRUCT,
            },
        ],
        generics: &[],
    };
}
