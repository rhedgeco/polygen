use super::{PolyField, PolyType};

pub struct PolyFn {
    pub ident: &'static str,
    pub export_ident: &'static str,
    pub inputs: &'static [PolyField],
    pub output: Option<PolyType>,
}
