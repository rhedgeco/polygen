use super::{PolyField, PolyType};

#[derive(Debug, Clone, Copy)]
pub struct PolyImpl {
    pub functions: &'static [ImplFn],
}

#[derive(Debug, Clone, Copy)]
pub struct ImplFn {
    pub name: &'static str,
    pub export_name: &'static str,
    pub inputs: &'static [PolyField],
    pub output: Option<PolyType>,
}
