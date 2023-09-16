use crate::{__private::ExportedPolyStruct, items::PolyType};

unsafe impl<T: ExportedPolyStruct> ExportedPolyStruct for *mut T {
    type ExportedType = *mut T;
    const STRUCT: PolyType = PolyType::Pointer(&T::STRUCT);
}

unsafe impl<T: ExportedPolyStruct> ExportedPolyStruct for *const T {
    type ExportedType = *const T;
    const STRUCT: PolyType = PolyType::Pointer(&T::STRUCT);
}
