use std::any::TypeId;

use thiserror::Error;

use crate::{
    __private::ExportedPolyStruct,
    items::{FieldType, PolyStruct, PolyType, StructField},
};

#[derive(Debug, Error)]
#[error("Pointer is invalid")]
pub struct InvalidPtr;

#[repr(C)]
pub struct OpaquePtr {
    id: TypeId,
    ptr: usize,
}

impl OpaquePtr {
    pub fn new<T: 'static>(item: T) -> Self {
        Self {
            id: TypeId::of::<T>(),
            ptr: Box::into_raw(Box::new(item)) as usize,
        }
    }

    pub fn as_ref<T: 'static>(&self) -> Result<&T, InvalidPtr> {
        self.validate_pointer::<T>()?;
        Ok(unsafe { &*(self.ptr as *const T) })
    }

    pub fn as_mut<T: 'static>(&mut self) -> Result<&mut T, InvalidPtr> {
        self.validate_pointer::<T>()?;
        Ok(unsafe { &mut *(self.ptr as *mut T) })
    }

    pub fn into_inner<T: 'static>(self) -> Result<T, InvalidPtr> {
        self.validate_pointer::<T>()?;
        Ok(unsafe { *Box::from_raw(self.ptr as *mut T) })
    }

    fn validate_pointer<T: 'static>(&self) -> Result<(), InvalidPtr> {
        if self.id != TypeId::of::<T>() {
            return Err(InvalidPtr);
        }

        Ok(())
    }
}

unsafe impl ExportedPolyStruct for OpaquePtr {
    type ExportedType = OpaquePtr;
    const STRUCT: PolyType = PolyType::Struct(PolyStruct {
        module: "::polygen",
        name: stringify!(OpaquePtr),
        fields: &[
            StructField {
                visible: false,
                name: "id",
                ty: FieldType::Typed(&<TypeId as ExportedPolyStruct>::STRUCT),
            },
            StructField {
                visible: false,
                name: "ptr",
                ty: FieldType::Typed(&<usize as ExportedPolyStruct>::STRUCT),
            },
        ],
        generics: &[],
    });
}
