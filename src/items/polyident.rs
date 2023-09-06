use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
pub struct PolyIdent {
    pub module: &'static str,
    pub name: &'static str,
    pub export_name: &'static str,
}

impl Eq for PolyIdent {}
impl PartialEq for PolyIdent {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module && self.name == other.name
    }
}

impl Hash for PolyIdent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.module.hash(state);
        self.name.hash(state);
    }
}
