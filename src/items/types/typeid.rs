use std::any::TypeId;

use crate::{
    __private::ExportedPolyStruct,
    items::{PolyField, PolyStruct},
};

// assert that TypeId is 64 bits
// it seems like this may change in the future
// so this is necessary to fail compilation if the size changes
const _: fn(TypeId) = |id| unsafe {
    std::mem::transmute::<TypeId, u64>(id);
};

unsafe impl ExportedPolyStruct for TypeId {
    type ExportedType = TypeId;

    const STRUCT: PolyStruct = PolyStruct {
        module: "::polygen",
        name: stringify!(TypeId),
        fields: &[PolyField {
            name: "id",
            ty_name: stringify!(u64),
            ty: &<u64 as ExportedPolyStruct>::STRUCT,
        }],
        generics: &[],
    };
}
