use crate::items::PolyFn;

pub unsafe trait ExportedPolyFn {
    const FUNCTION: PolyFn;
}
