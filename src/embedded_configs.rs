//! Embedded, shared TOML configs.
//!
//! Motivation:
//! - WASM frontends (editor/viewer) cannot read repo-relative files at runtime.
//! - Native examples want a single source of truth for “default/demo” configs.
//!
//! These configs are *not* intended as a general runtime configuration mechanism.
//! They are a convenient, versioned set of defaults that all frontends can seed from.

/// Dark theme default used by examples and the editor.
pub const THEME_DARK_TOML: &str = include_str!("../config/theme_dark.toml");

/// Light theme default used by examples and the viewer.
pub const THEME_LIGHT_TOML: &str = include_str!("../config/theme_light.toml");

/// Spec-driven natal layout default used by examples and the editor.
pub const CHART_SPEC_NATAL_LAYOUT_ONLY_TOML: &str =
    include_str!("../config/chart_spec_natal_layout_only.toml");

/// Spec-driven natal chart data default used by examples and the editor.
pub const CHART_SPEC_NATAL_DATA_TOML: &str = include_str!("../config/chart_spec_natal_data.toml");

/// Aspect rules default used by examples and the editor.
pub const CHART_SPEC_NATAL_ASPECTS_TOML: &str =
    include_str!("../config/chart_spec_natal_aspects.toml");

/// Spec-driven transit layout default used by examples.
pub const CHART_SPEC_TRANSIT_LAYOUT_ONLY_TOML: &str =
    include_str!("../config/chart_spec_transit_layout_only.toml");

/// Spec-driven transit chart data default used by examples.
pub const CHART_SPEC_TRANSIT_DATA_TOML: &str =
    include_str!("../config/chart_spec_transit_data.toml");

/// Transit aspect rules default used by examples.
pub const CHART_SPEC_TRANSIT_ASPECTS_TOML: &str =
    include_str!("../config/chart_spec_transit_aspects.toml");
