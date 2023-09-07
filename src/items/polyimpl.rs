use super::PolyFn;

#[derive(Debug, Clone, Copy)]
pub struct PolyImpl {
    pub functions: &'static [PolyFn],
}
