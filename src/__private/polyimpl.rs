use crate::items::PolyImpl;

use super::ExportedPolyType;

pub unsafe trait ExportedPolyImpl: ExportedPolyType {
    const IMPL: PolyImpl;
}
