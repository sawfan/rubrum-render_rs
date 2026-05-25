use serde::{Deserialize, Serialize};

/// Band thickness specification.
///
/// - `{ abs = px }`: fixed thickness in px
/// - `{ frac = f }`: weight of remaining radius (allocated proportionally)
/// - `{ fit = true }`: computed from lane content (not always supported)
/// - `n`: shorthand for `{ abs = n }`
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThicknessSpec {
    /// Shorthand: a bare number is an absolute thickness in px.
    Px(f64),

    Abs {
        abs: f64,
    },
    Frac {
        frac: f64,
    },
    Fit {
        fit: bool,
    },
}

impl ThicknessSpec {
    pub fn as_abs_px(&self) -> Option<f64> {
        match *self {
            Self::Px(px) => Some(px),
            Self::Abs { abs } => Some(abs),
            _ => None,
        }
    }
}
