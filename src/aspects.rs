use serde::{Deserialize, Serialize};

use crate::options::RgbaColor;

/// Rendering configuration for aspect lines.
///
/// This is intentionally separate from the aspect computation rules (orbs, enabled kinds, etc).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AspectsStyle {
    /// If true, render aspect lines.
    pub enabled: bool,

    /// Default stroke style for aspect lines.
    ///
    /// Individual aspect kinds can override this via `kind_styles`.
    pub stroke: AspectStrokeStyle,

    /// Optional per-aspect-kind styling.
    ///
    /// This allows (for example):
    /// - hard aspects (square/opposition) in red + dashed
    /// - soft aspects (trine/sextile) in blue
    /// - conjunctions in a neutral color
    pub kind_styles: Vec<AspectKindStyle>,

    /// Optional label rendering configuration.
    ///
    /// Labels are rendered as raw SVG near the midpoint of each aspect line.
    pub labels: AspectLabelsStyle,

    /// Inner radius factor for where aspect lines should be drawn.
    ///
    /// 1.0 = use the aspect-anchor ring radius, 0.0 = use the chart center.
    ///
    /// Values in 0.0..=1.0.
    pub radius_factor: Option<f64>,

    /// If true, draw only aspects where both endpoints appear in the placement glyph list.
    ///
    /// (This avoids drawing lines to points that aren't being rendered as placements.)
    #[serde(default = "default_true")]
    pub require_endpoints_present: bool,

    /// If true, emit stable CSS classes + data attributes on injected SVG nodes.
    ///
    /// This enables downstream styling via CSS (in addition to any TOML configuration).
    #[serde(default = "default_true")]
    pub emit_svg_classes: bool,
}

fn default_true() -> bool {
    true
}

impl Default for AspectsStyle {
    fn default() -> Self {
        Self {
            enabled: false,
            stroke: AspectStrokeStyle::default(),
            kind_styles: Vec::new(),
            labels: AspectLabelsStyle::default(),
            radius_factor: Some(0.0),
            require_endpoints_present: true,
            emit_svg_classes: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StrokeLineCap {
    #[default]
    Butt,
    Round,
    Square,
}

/// Default stroke style for aspect lines.
///
/// Note: this is used as the *global* default. Per-kind overrides use
/// [`AspectStrokeStyleOverride`] so unspecified fields do not override the global defaults.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AspectStrokeStyle {
    pub color: Option<RgbaColor>,
    pub width: Option<f64>,
    pub dash: Option<Vec<f64>>,
    pub alpha: Option<f64>,

    /// SVG `stroke-linecap`.
    pub linecap: Option<StrokeLineCap>,
}

impl Default for AspectStrokeStyle {
    fn default() -> Self {
        Self {
            color: None,
            width: Some(1.0),
            dash: None,
            alpha: Some(0.6),
            linecap: None,
        }
    }
}

/// Per-kind stroke overrides.
///
/// All fields are optional and only apply when set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AspectKindGroup {
    Hard,
    Soft,
    Neutral,
    Minor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedAspectStrokeStyle {
    pub color: RgbaColor,
    pub width: f64,
    pub dash: Option<Vec<f64>>,
    pub linecap: Option<StrokeLineCap>,
}

pub fn aspect_kind_group(kind: &rubrum::DegreeAspectKind) -> AspectKindGroup {
    use rubrum::DegreeAspectKind;

    match kind {
        DegreeAspectKind::Square
        | DegreeAspectKind::Opposition
        | DegreeAspectKind::SemiSquare
        | DegreeAspectKind::Sesquiquadrate => AspectKindGroup::Hard,
        DegreeAspectKind::Trine | DegreeAspectKind::Sextile => AspectKindGroup::Soft,
        DegreeAspectKind::Conjunction => AspectKindGroup::Neutral,
        _ => AspectKindGroup::Minor,
    }
}

pub fn resolve_aspect_stroke_style(
    aspects: &AspectsStyle,
    kind: &rubrum::DegreeAspectKind,
    fallback_color: RgbaColor,
    fallback_width: f64,
) -> ResolvedAspectStrokeStyle {
    let kind_override = aspects.kind_styles.iter().find(|s| s.kind == *kind);

    let color = kind_override
        .and_then(|s| s.stroke.color)
        .or(aspects.stroke.color)
        .unwrap_or(fallback_color);

    let alpha = kind_override
        .and_then(|s| s.stroke.alpha)
        .or(aspects.stroke.alpha)
        .unwrap_or(1.0)
        .clamp(0.0, 1.0);

    let mut color = color;
    color.a *= alpha;

    let width = kind_override
        .and_then(|s| s.stroke.width)
        .or(aspects.stroke.width)
        .unwrap_or(fallback_width)
        .max(0.1);

    let dash = kind_override
        .and_then(|s| s.stroke.dash.clone())
        .or_else(|| aspects.stroke.dash.clone());

    let linecap = kind_override
        .and_then(|s| s.stroke.linecap)
        .or(aspects.stroke.linecap);

    ResolvedAspectStrokeStyle {
        color,
        width,
        dash,
        linecap,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AspectStrokeStyleOverride {
    pub color: Option<RgbaColor>,
    pub width: Option<f64>,
    pub dash: Option<Vec<f64>>,
    pub alpha: Option<f64>,
    pub linecap: Option<StrokeLineCap>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AspectKindStyle {
    pub kind: rubrum::DegreeAspectKind,

    /// Optional stroke overrides for this aspect kind.
    pub stroke: AspectStrokeStyleOverride,

    /// Optional label overrides for this aspect kind.
    pub labels: Option<AspectLabelsStyleOverride>,
}

impl Default for AspectKindStyle {
    fn default() -> Self {
        Self {
            kind: rubrum::DegreeAspectKind::Conjunction,
            stroke: AspectStrokeStyleOverride::default(),
            labels: None,
        }
    }
}

/// Global label configuration for aspect lines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AspectLabelsStyle {
    /// If true, render aspect kind symbols at line midpoints.
    pub enabled: bool,

    /// Text style.
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub text_color: Option<RgbaColor>,

    /// Optional background box behind the symbol.
    pub box_enabled: bool,
    pub box_fill: Option<RgbaColor>,
    pub box_stroke: Option<RgbaColor>,
    pub box_stroke_width: Option<f64>,
    pub box_padding: Option<f64>,
    pub box_corner_radius: Option<f64>,
}

impl Default for AspectLabelsStyle {
    fn default() -> Self {
        Self {
            enabled: false,
            font_family: None,
            font_size: Some(10.0),
            text_color: None,
            box_enabled: false,
            box_fill: None,
            box_stroke: None,
            box_stroke_width: Some(1.0),
            box_padding: Some(3.0),
            box_corner_radius: Some(2.0),
        }
    }
}

/// Per-kind label overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AspectLabelsStyleOverride {
    pub enabled: Option<bool>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub text_color: Option<RgbaColor>,

    pub box_enabled: Option<bool>,
    pub box_fill: Option<RgbaColor>,
    pub box_stroke: Option<RgbaColor>,
    pub box_stroke_width: Option<f64>,
    pub box_padding: Option<f64>,
    pub box_corner_radius: Option<f64>,
}
