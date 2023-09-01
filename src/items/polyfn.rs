use super::{PolyField, PolyType};

pub struct PolyFn {
    pub ident: &'static str,
    pub inputs: &'static [PolyField],
    pub output: Option<PolyType>,
}
