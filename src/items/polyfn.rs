use super::{PolyField, PolyType};

#[derive(Debug, Clone, Copy)]
pub struct PolyFn {
    pub ident: &'static str,
    pub module: &'static str,
    pub export_ident: &'static str,
    pub inputs: &'static [PolyField],
    pub output: Option<PolyType>,
}
