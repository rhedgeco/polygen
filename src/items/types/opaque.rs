use std::any::TypeId;

use thiserror::Error;

use crate::{
    __private::ExportedPolyStruct,
    items::{PolyField, PolyIdent, PolyStruct},
};

#[derive(Debug, Error)]
#[error("PolyBox points to a null reference")]
pub struct NullError;

pub struct Opaque<T: 'static> {
    item: *mut T,
}

impl<T: 'static> Opaque<T> {
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
pub struct OpaqueUntyped {
    id: u64,
    item: usize,
}

#[doc(hidden)]
impl<T: 'static> From<Opaque<T>> for OpaqueUntyped {
    fn from(value: Opaque<T>) -> Self {
        Self {
            id: unsafe { std::mem::transmute(TypeId::of::<T>()) },
            item: value.item as usize,
        }
    }
}

#[doc(hidden)]
impl<T: 'static> Into<Opaque<T>> for OpaqueUntyped {
    fn into(self) -> Opaque<T> {
        let mut item = 0;
        let id: TypeId = unsafe { std::mem::transmute(self.id) };
        if id == TypeId::of::<T>() {
            item = self.item
        }

        Opaque {
            item: item as *mut T,
        }
    }
}

unsafe impl<T: 'static> ExportedPolyStruct for Opaque<T> {
    type ExportedType = OpaqueUntyped;
    const STRUCT: PolyStruct = PolyStruct {
        ident: PolyIdent {
            module: "root::polygen",
            name: "Opaque",
            export_name: "Opaque",
        },
        fields: &[
            PolyField {
                name: "id",
                ty: <u64 as ExportedPolyStruct>::STRUCT,
            },
            PolyField {
                name: "item",
                ty: <usize as ExportedPolyStruct>::STRUCT,
            },
        ],
    };
}
