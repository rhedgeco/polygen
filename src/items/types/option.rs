use std::mem::MaybeUninit;

use crate::{
    __private::ExportedPolyStruct,
    items::{FieldType, PolyGeneric, PolyStruct, PolyType, StructField},
};

#[repr(C)]
pub struct PolyOption<T: ExportedPolyStruct> {
    valid: bool,
    data: MaybeUninit<T>,
}

impl<T: ExportedPolyStruct> From<Option<T>> for PolyOption<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(item) => Self {
                valid: true,
                data: MaybeUninit::new(item),
            },
            None => Self {
                valid: false,
                data: MaybeUninit::uninit(),
            },
        }
    }
}

unsafe impl<T: ExportedPolyStruct> ExportedPolyStruct for PolyOption<T> {
    type ExportedType = Self;

    const STRUCT: PolyType = PolyType::Struct(PolyStruct {
        module: "::polygen",
        name: "PolyOption",
        fields: &[StructField {
            name: "valid",
            ty: FieldType::Generic("T"),
        }],
        generics: &[PolyGeneric {
            ident: "T",
            ty: &T::STRUCT,
        }],
    });
}
