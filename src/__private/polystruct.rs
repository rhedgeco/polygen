use crate::items::PolyStruct;

pub unsafe trait ExportedPolyStruct: Sized + 'static {
    type ExportedType: From<Self> + Into<Self>;
    const STRUCT: PolyStruct;
}
