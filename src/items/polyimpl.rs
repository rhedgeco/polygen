use serde::Serialize;

use super::FnParams;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PolyImpl {
    pub functions: &'static [ImplFn],
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ImplFn {
    pub name: &'static str,
    pub export_name: &'static str,
    pub params: FnParams,
}
