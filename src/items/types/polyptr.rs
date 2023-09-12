use crate::{
    __private::ExportedPolyStruct,
    items::{PolyGeneric, PolyStruct, StructField},
};

unsafe impl<T: ExportedPolyStruct> ExportedPolyStruct for *mut T {
    type ExportedType = *mut T;
    const STRUCT: PolyStruct = PolyStruct {
        module: "::polygen",
        name: "PolyPtr",
        fields: &[StructField {
            name: "ptr",
            ty: crate::items::FieldType::Typed(&<usize as ExportedPolyStruct>::STRUCT),
        }],
        generics: &[PolyGeneric {
            ident: "T",
            ty: &<T as ExportedPolyStruct>::STRUCT,
        }],
    };
}
