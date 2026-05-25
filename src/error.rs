use std::fmt;

/// Errors that can occur when rendering a chart.
#[derive(Debug)]
pub enum ChartRenderError {
    Io(std::io::Error),

    /// Indicates an invalid Theme/Layout/ChartData spec (e.g. layout validation failed).
    InvalidSpec(String),
}

impl fmt::Display for ChartRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::InvalidSpec(e) => write!(f, "Invalid spec: {e}"),
        }
    }
}

impl std::error::Error for ChartRenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::InvalidSpec(_) => None,
        }
    }
}

impl From<std::io::Error> for ChartRenderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// Backwards-compatible alias.
///
/// Historically the outer crate exposed `ChartCairoRenderError`.
/// Keep this alias in the shared core crate so downstream code can remain stable.
pub type ChartCairoRenderError = ChartRenderError;

// // Cairo backend error conversion.
// //
// // `rubrum_render` is backend-agnostic, but `rubrum_cairo` re-uses `ChartRenderError` as its
// // error type alias (`ChartCairoRenderError`). In order for Cairo code to use `?` ergonomically,
// // we provide a `From<cairo::Error>` conversion behind a feature gate.
// #[cfg(feature = "cairo")]
// impl From<cairo::Error> for ChartRenderError {
//     fn from(value: cairo::Error) -> Self {
//         // Cairo errors don't carry much structure; keep a readable message.
//         Self::InvalidSpec(format!("Cairo error: {value}"))
//     }
// }
