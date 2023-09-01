use crate::items::PolyFn;

pub trait ExportedPolyFn {
    const FUNCTION: PolyFn;
}
