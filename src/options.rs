use serde::{Deserialize, Serialize};

fn default_glyph_pack_dir() -> String {
    // Default to an actual pack directory (contains manifest.toml + SVG assets).
    "assets/black".to_owned()
}

fn default_glyph_sets_config_path() -> String {
    "config/glyph_sets.toml".to_owned()
}

fn default_width() -> i32 {
    900
}

fn default_height() -> i32 {
    900
}

fn default_margin() -> i32 {
    40
}

fn default_stroke_width() -> f64 {
    2.0
}

fn default_sign_font_size() -> f64 {
    22.0
}

fn default_label_font_size() -> f64 {
    18.0
}

fn default_font_family() -> String {
    // Font availability varies by OS.
    "DejaVu Sans".to_owned()
}

fn default_occupant_symbol_size() -> f64 {
    20.0
}

fn default_alpha() -> f64 {
    1.0
}

fn default_placement_leader_stroke() -> RgbaColor {
    // A subtle medium gray leader line.
    RgbaColor {
        r: 0.35,
        g: 0.35,
        b: 0.35,
        a: 1.0,
    }
}

fn default_placement_leader_width() -> f64 {
    1.0
}

fn default_placement_min_separation_px() -> Option<f64> {
    // None => derive from label/glyph size.
    None
}

/// RGBA color with components in the range 0.0–1.0.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RgbaColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,

    #[serde(default = "default_alpha")]
    pub a: f64,
}

impl Default for RgbaColor {
    fn default() -> Self {
        Self {
            // Preserve existing behavior: white background.
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

/// How to render occupant glyphs (bodies, angles, lots, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CairoOccupantGlyphMode {
    /// Render occupant glyphs as text.
    #[default]
    Text,

    /// Render `Occupant::Body` using embedded SVG symbols when available; fall back to text.
    PreferSvgFallbackText,
}

/// Selector for choosing a glyph set based on a light/dark mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlyphSetModeSelector {
    /// Which mode is active.
    pub mode: GlyphSetMode,

    /// Glyph set to use when `mode = "Light"`.
    pub light: Option<String>,

    /// Glyph set to use when `mode = "Dark"`.
    pub dark: Option<String>,
}

impl Default for GlyphSetModeSelector {
    fn default() -> Self {
        Self {
            mode: GlyphSetMode::Light,
            light: None,
            dark: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GlyphSetMode {
    #[default]
    Light,
    Dark,
}

/// Settings controlling Cairo-based chart rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChartCairoOptions {
    /// Output width in pixels.
    pub width: i32,

    /// Output height in pixels.
    pub height: i32,

    /// Outer wheel margin in pixels.
    pub margin: i32,

    /// Stroke width in pixels.
    pub stroke_width: f64,

    /// Font size for rim sign labels.
    pub sign_font_size: f64,

    /// Font size for occupant labels.
    pub label_font_size: f64,

    /// Font family for text.
    ///
    /// Note: Cairo expects a single font family name (not a CSS fallback stack).
    pub font_family: String,

    /// Background fill color.
    ///
    /// Set `a = 0.0` for a transparent background.
    #[serde(default)]
    pub background: RgbaColor,

    /// How occupant glyphs should be rendered.
    #[serde(default)]
    pub occupant_glyph_mode: CairoOccupantGlyphMode,

    /// Target size (in px) for SVG-symbol occupant glyphs.
    ///
    /// This is only used when `occupant_glyph_mode` renders SVG symbols.
    #[serde(default)]
    pub occupant_symbol_size: f64,

    /// If true, attempt to reduce placement overlap by stacking close placements and drawing
    /// leader lines back to the exact degree.
    #[serde(default)]
    pub placement_collision_avoidance: bool,

    /// Minimum separation (in px) used to detect “too close” placements.
    ///
    /// If `None`, this is derived from `label_font_size` / `occupant_symbol_size`.
    #[serde(default = "default_placement_min_separation_px")]
    pub placement_min_separation_px: Option<f64>,

    /// Stroke color used for placement leader lines (only when collision avoidance is enabled).
    #[serde(default = "default_placement_leader_stroke")]
    pub placement_leader_stroke: RgbaColor,

    /// Stroke width used for placement leader lines (only when collision avoidance is enabled).
    #[serde(default = "default_placement_leader_width")]
    pub placement_leader_width: f64,

    /// Directory containing an SVG glyph pack (e.g. "assets/black").
    #[serde(default = "default_glyph_pack_dir")]
    pub glyph_pack_dir: String,

    /// Path to glyph set configuration TOML.
    #[serde(default = "default_glyph_sets_config_path")]
    pub glyph_sets_config_path: String,

    /// Optional glyph set name to use when resolving SVG glyph assets.
    #[serde(default)]
    pub glyph_set: Option<String>,

    /// Optional mode-based glyph set selector (Light/Dark).
    #[serde(default)]
    pub glyph_set_mode_selector: Option<GlyphSetModeSelector>,
}

impl Default for ChartCairoOptions {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            margin: default_margin(),
            stroke_width: default_stroke_width(),
            sign_font_size: default_sign_font_size(),
            label_font_size: default_label_font_size(),
            font_family: default_font_family(),
            background: RgbaColor::default(),
            occupant_glyph_mode: CairoOccupantGlyphMode::Text,
            occupant_symbol_size: default_occupant_symbol_size(),
            placement_collision_avoidance: false,
            placement_min_separation_px: default_placement_min_separation_px(),
            placement_leader_stroke: default_placement_leader_stroke(),
            placement_leader_width: default_placement_leader_width(),
            glyph_pack_dir: default_glyph_pack_dir(),
            glyph_sets_config_path: default_glyph_sets_config_path(),
            glyph_set: None,
            glyph_set_mode_selector: None,
        }
    }
}

impl ChartCairoOptions {
    /// Resolve the effective glyph set name to use for SVG glyph lookup.
    ///
    /// Priority:
    /// 1) `glyph_set_mode_selector` (when configured) chooses between `light` and `dark`.
    /// 2) `glyph_set` explicit override.
    /// 3) `None` (fall back to pack-dir behavior).
    pub fn effective_glyph_set_name(&self) -> Option<String> {
        if let Some(sel) = self.glyph_set_mode_selector.as_ref() {
            let selected = match sel.mode {
                GlyphSetMode::Light => sel.light.as_ref(),
                GlyphSetMode::Dark => sel.dark.as_ref(),
            };

            if let Some(name) = selected {
                return Some(name.clone());
            }
        }

        self.glyph_set.clone()
    }
}
