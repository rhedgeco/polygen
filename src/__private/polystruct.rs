use crate::items::PolyType;

pub unsafe trait ExportedPolyStruct: Sized + 'static {
    type ExportedType: From<Self> + Into<Self>;
    const STRUCT: PolyType;
}
