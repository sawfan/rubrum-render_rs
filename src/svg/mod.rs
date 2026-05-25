//! Minimal SVG string helpers.
//!
//! NOTE: These helpers exist to support pure-SVG rendering and metadata emission.
//! They intentionally do **not** depend on any SVG DOM crate.

mod attrs;
mod text;

pub use attrs::*;
pub use text::*;
