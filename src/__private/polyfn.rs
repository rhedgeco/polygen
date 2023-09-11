use crate::items::PolyFn;

pub unsafe trait ExportedPolyFn: Sized + 'static {
    const FUNCTION: PolyFn;
}
