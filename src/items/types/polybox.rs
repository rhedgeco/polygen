use std::any::TypeId;

use thiserror::Error;

use crate::{
    __private::ExportedPolyStruct,
    items::{PolyField, PolyStruct},
};

#[derive(Debug, Error)]
#[error("PolyBox points to a null reference")]
pub struct NullError;

pub struct PolyBox<T: 'static> {
    item: *mut T,
}

impl<T: 'static> PolyBox<T> {
    pub fn new(item: T) -> Self {
        Self {
            item: Box::into_raw(Box::new(item)),
        }
    }

    pub fn into_item(self) -> Result<T, NullError> {
        if self.item.is_null() {
            return Err(NullError);
        }

        Ok(unsafe { *Box::from_raw(self.item) })
    }

    pub fn as_ref(&self) -> Result<&T, NullError> {
        if self.item.is_null() {
            return Err(NullError);
        }

        Ok(unsafe { &*self.item })
    }

    pub fn as_mut(&mut self) -> Result<&mut T, NullError> {
        if self.item.is_null() {
            return Err(NullError);
        }

        Ok(unsafe { &mut *self.item })
    }
}

#[repr(C)]
#[doc(hidden)]
pub struct PolyBoxUntyped {
    id: u64,
    item: usize,
}

#[doc(hidden)]
impl<T: 'static> From<PolyBox<T>> for PolyBoxUntyped {
    fn from(value: PolyBox<T>) -> Self {
        Self {
            id: unsafe { std::mem::transmute(TypeId::of::<T>()) },
            item: value.item as usize,
        }
    }
}

#[doc(hidden)]
impl<T: 'static> Into<PolyBox<T>> for PolyBoxUntyped {
    fn into(self) -> PolyBox<T> {
        let mut item = 0;
        let id: TypeId = unsafe { std::mem::transmute(self.id) };
        if id == TypeId::of::<T>() {
            item = self.item
        }

        PolyBox {
            item: item as *mut T,
        }
    }
}

unsafe impl<T: 'static> ExportedPolyStruct for PolyBox<T> {
    type ExportedType = PolyBoxUntyped;
    const STRUCT: PolyStruct = PolyStruct {
        module: "::polygen",
        name: "PolyBox",
        fields: &[
            PolyField {
                name: "id",
                ty: &<u64 as ExportedPolyStruct>::STRUCT,
            },
            PolyField {
                name: "item",
                ty: &<usize as ExportedPolyStruct>::STRUCT,
            },
        ],
    };
}
