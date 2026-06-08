//! Shared chart-rendering types and planning code.
//!
//! This crate is the backend-agnostic core used by both:
//! - `rubrum_cairo` (native Cairo renderer)
//! - `rubrum_svg` (pure-SVG renderer)
//!
//! It contains spec/data types (Theme/Layout/ChartData), render planning, and small helpers
//! (geometry, glyph IDs, etc.).

pub mod aspects;
pub mod chart_data;
pub mod core;

/// Embedded TOML configs used as demo defaults by the editor/viewer and for examples.
///
/// These are embedded so WASM builds don't rely on filesystem paths, and so multiple frontends
/// (Cairo, SVG, editor/viewer) can share the same starting point.
pub mod embedded_configs;

pub mod dataset;
pub mod error;
pub mod glyph_paint;
pub mod glyphs;
pub mod labels;
pub mod layout;
pub mod style;
pub mod svg;

pub mod metadata;
pub mod options;
pub mod theme;
pub mod thickness;

pub use aspects::*;
pub use chart_data::{ChartData, DatasetMetadata, PlacementMetadata};
pub use core::render_plan::{RenderPlan, plan_chart_spec};
pub use dataset::{DatasetData, HouseSetData};
pub use error::ChartRenderError;
pub use glyph_paint::{
    GlyphPaint, GlyphTheme, OccupantGlyphTheme, SignGlyphTheme, occupant_type_key,
    resolve_occupant_glyph_paint, resolve_sign_glyph_paint, sign_element,
};
pub use glyphs::*;
pub use layout::Layout;
pub use options::{CairoOccupantGlyphMode, ChartCairoOptions, RgbaColor};
pub use theme::Theme;

// Re-export rubrum types that are used throughout the rendering pipeline.
pub use rubrum::{Angle, Body, ChartPoint, House, Occupant, Placement, Sign};

// Additional rubrum types used by our public API/tests.
pub use rubrum::{
    AspectRules, Coordinate, EndpointFilter, EndpointKey, Motion, PlacementMotion, SignDegree,
};
