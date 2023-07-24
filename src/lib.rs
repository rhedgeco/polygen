#[doc(hidden)]
pub use polygen_private as __private;

// re-export macro
#[allow(unused_imports)]
#[macro_use]
extern crate polygen_proc;
pub use polygen_proc::polygen;
