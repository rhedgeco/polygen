use crate::{
    __private::ExportedPolyStruct,
    items::{PolyGeneric, PolyStruct},
};

use super::OpaquePtr;

#[repr(C)]
pub struct PolyPtr<T: ExportedPolyStruct>(OpaquePtr<T>);

unsafe impl<T: ExportedPolyStruct> ExportedPolyStruct for PolyPtr<T> {
    type ExportedType = PolyPtr<T>;
    const STRUCT: PolyStruct = PolyStruct {
        module: "::polygen",
        name: stringify!(PolyPtr),
        // Since the opaque pointer is the only field,
        // we can just copy its fields reference into this one.
        // This means that there is no nesting on the generated code
        // because I cant figure out how to do that effectively yet lol
        fields: <OpaquePtr<T> as ExportedPolyStruct>::STRUCT.fields,
        generics: &[PolyGeneric {
            ident: "T",
            ty: &<T as ExportedPolyStruct>::STRUCT,
        }],
    };
}
