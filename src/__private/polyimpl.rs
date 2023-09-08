use crate::items::PolyImpl;

use super::ExportedPolyStruct;

pub unsafe trait ExportedPolyImpl: ExportedPolyStruct {
    const IMPL: PolyImpl;
}
