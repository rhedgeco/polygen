use proc_macro2::TokenStream;

use super::PolyResult;

pub type BuildResult<T> = PolyResult<PolyBuild<T>>;

pub struct PolyBuild<T> {
    pub polyitem: T,
    pub assertions: TokenStream,
}

impl<T> PolyBuild<T> {
    pub fn build(item: T, assertions: TokenStream) -> BuildResult<T> {
        Ok(Self {
            polyitem: item,
            assertions,
        })
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> BuildResult<U> {
        PolyBuild::build(f(self.polyitem), self.assertions)
    }
}
