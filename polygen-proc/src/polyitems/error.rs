use std::fmt::{Debug, Display};

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

pub type PolyResult<T> = Result<T, PolyError>;

enum ErrorType {
    Simple(anyhow::Error),
    Spanned(Span, anyhow::Error),
}

pub struct PolyError {
    errors: Vec<ErrorType>,
}

impl ToTokens for PolyError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for error in &self.errors {
            match error {
                ErrorType::Simple(error) => {
                    let message = format!("{error}");
                    tokens.extend(quote!( compile_error!(#message); ));
                }
                ErrorType::Spanned(span, error) => {
                    let span = span.clone();
                    let message = format!("{error}");
                    tokens.extend(quote_spanned!( span => compile_error!(#message); ));
                }
            }
        }
    }
}

impl PolyError {
    /// Builds a new empty `PolyError`
    pub const fn empty() -> Self {
        Self { errors: Vec::new() }
    }

    /// Builds a new `PolyError` with a simple error containing `message`
    pub fn simple<M>(message: M) -> Self
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Self::simple_error(anyhow::Error::msg(message))
    }

    /// Builds a new `PolyError` containing `error`
    pub fn simple_error(error: impl Into<anyhow::Error>) -> Self {
        Self {
            errors: vec![ErrorType::Simple(error.into())],
        }
    }

    /// Builds a new `PolyError` containing and error `message` over `span`
    pub fn spanned<M>(span: &impl Spanned, message: M) -> Self
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Self::spanned_error(span, anyhow::Error::msg(message))
    }

    /// Builds a new `PolyError` containing `error` over a `span`
    pub fn spanned_error(span: &impl Spanned, error: impl Into<anyhow::Error>) -> Self {
        Self {
            errors: vec![ErrorType::Spanned(span.span(), error.into())],
        }
    }

    /// Pushes a new error with `message` into this error
    #[allow(dead_code)]
    pub fn push_simple<M>(&mut self, message: M)
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        self.merge(Self::simple(message))
    }

    /// Pushes a new error with `message` over `span` into this error
    pub fn push_spanned<M>(&mut self, span: &impl Spanned, message: M)
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        self.merge(Self::spanned(span, message))
    }

    /// Merges and consumes `other` into self
    pub fn merge(&mut self, mut other: Self) {
        self.errors.append(&mut other.errors);
    }

    /// Returns `Ok(PolyError)` if it is empty.
    /// Returns `Err(PolyError)` if it contains any errors.
    pub fn fork(self) -> Result<Self, Self> {
        if self.errors.len() == 0 {
            Ok(self)
        } else {
            Err(self)
        }
    }
}
