use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::options::RgbaColor;
use rubrum::{ElementKind, Occupant, Sign};

/// Paint applied to an SVG glyph instance.
///
/// `color` is the preferred monochrome control and works with tintable sprites that use
/// `currentColor`. `fill`/`stroke` allow two-channel/outlined sprite variants to expose separate
/// fill and stroke colors. Optional opacity/width fields are emitted as SVG presentation attrs by
/// SVG backends when supported by the referenced glyph asset.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GlyphPaint {
    pub color: Option<RgbaColor>,
    pub fill: Option<RgbaColor>,
    pub stroke: Option<RgbaColor>,
    pub fill_opacity: Option<f64>,
    pub stroke_opacity: Option<f64>,
    pub stroke_width: Option<f64>,
}

impl GlyphPaint {
    pub fn monochrome(color: RgbaColor) -> Self {
        Self {
            color: Some(color),
            fill: None,
            stroke: None,
            fill_opacity: None,
            stroke_opacity: None,
            stroke_width: None,
        }
    }

    /// Overlay `self` on top of `base`, keeping `base` fields when `self` omits them.
    pub fn overlay(self, base: Self) -> Self {
        Self {
            color: self.color.or(base.color),
            fill: self.fill.or(base.fill),
            stroke: self.stroke.or(base.stroke),
            fill_opacity: self.fill_opacity.or(base.fill_opacity),
            stroke_opacity: self.stroke_opacity.or(base.stroke_opacity),
            stroke_width: self.stroke_width.or(base.stroke_width),
        }
    }

    /// Ensure there is at least a monochrome fallback color.
    pub fn with_fallback_color(self, fallback: RgbaColor) -> Self {
        Self {
            color: self.color.or(Some(fallback)),
            ..self
        }
    }
}

/// Theme-level glyph coloring controls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GlyphTheme {
    /// Global default for all glyphs.
    pub default: Option<GlyphPaint>,

    /// Sign glyph paint rules (outer wheel signs and placement sign glyph labels).
    pub signs: SignGlyphTheme,

    /// Occupant glyph paint rules (bodies, chart points, angles, lots).
    pub occupants: OccupantGlyphTheme,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SignGlyphTheme {
    pub default: Option<GlyphPaint>,

    /// Per-sign overrides keyed by canonical sign key (`aries`, `scorpio`, ...).
    pub by_sign: BTreeMap<String, GlyphPaint>,

    /// Element scheme keyed by canonical element key (`fire`, `earth`, `air`, `water`).
    ///
    /// This is intentionally just a map: users can make earth brown, green, or anything else.
    pub by_element: BTreeMap<String, GlyphPaint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OccupantGlyphTheme {
    pub default: Option<GlyphPaint>,

    /// Per-occupant overrides keyed by `Occupant::canonical_key()` (`sun`, `ascendant`, ...).
    pub by_occupant: BTreeMap<String, GlyphPaint>,

    /// Type-level defaults keyed by `body`, `chart_point`, `angle`, `lot`, or `empty`.
    pub by_type: BTreeMap<String, GlyphPaint>,
}

pub fn sign_element(sign: Sign) -> ElementKind {
    match sign {
        Sign::Aries | Sign::Leo | Sign::Sagittarius => ElementKind::Fire,
        Sign::Taurus | Sign::Virgo | Sign::Capricorn => ElementKind::Earth,
        Sign::Gemini | Sign::Libra | Sign::Aquarius => ElementKind::Air,
        Sign::Cancer | Sign::Scorpio | Sign::Pisces => ElementKind::Water,
    }
}

pub fn occupant_type_key(occupant: Occupant) -> &'static str {
    match occupant {
        Occupant::Empty => "empty",
        Occupant::Body(_) => "body",
        Occupant::ChartPoint(_) => "chart_point",
        Occupant::Angle(_) => "angle",
        Occupant::Lot(_) => "lot",
    }
}

/// Resolve sign glyph paint using precedence:
/// per-sign > element scheme > sign default > global default > fallback.
pub fn resolve_sign_glyph_paint(
    theme: &crate::theme::Theme,
    sign: Sign,
    fallback: RgbaColor,
) -> GlyphPaint {
    let mut paint = GlyphPaint::default();

    if let Some(global) = theme.glyphs.default {
        paint = global.overlay(paint);
    }
    if let Some(default) = theme.glyphs.signs.default {
        paint = default.overlay(paint);
    }

    let element_key = sign_element(sign).canonical_key();
    if let Some(element) = theme.glyphs.signs.by_element.get(element_key).copied() {
        paint = element.overlay(paint);
    }

    if let Some(sign_paint) = theme
        .glyphs
        .signs
        .by_sign
        .get(sign.canonical_key())
        .copied()
    {
        paint = sign_paint.overlay(paint);
    }

    paint.with_fallback_color(fallback)
}

/// Resolve occupant glyph paint using precedence:
/// per-occupant > type default > occupant default > global default > dataset/fallback color.
pub fn resolve_occupant_glyph_paint(
    theme: &crate::theme::Theme,
    occupant: Occupant,
    fallback: RgbaColor,
) -> GlyphPaint {
    let mut paint = GlyphPaint::default();

    if let Some(global) = theme.glyphs.default {
        paint = global.overlay(paint);
    }
    if let Some(default) = theme.glyphs.occupants.default {
        paint = default.overlay(paint);
    }

    if let Some(type_paint) = theme
        .glyphs
        .occupants
        .by_type
        .get(occupant_type_key(occupant))
        .copied()
    {
        paint = type_paint.overlay(paint);
    }

    if let Some(occupant_paint) = theme
        .glyphs
        .occupants
        .by_occupant
        .get(occupant.canonical_key())
        .copied()
    {
        paint = occupant_paint.overlay(paint);
    }

    paint.with_fallback_color(fallback)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use rubrum::{Body, Occupant, Sign};

    fn color(r: f64, g: f64, b: f64) -> RgbaColor {
        RgbaColor { r, g, b, a: 1.0 }
    }

    #[test]
    fn sign_element_mapping_is_astrological_default() {
        assert_eq!(sign_element(Sign::Aries), ElementKind::Fire);
        assert_eq!(sign_element(Sign::Taurus), ElementKind::Earth);
        assert_eq!(sign_element(Sign::Gemini), ElementKind::Air);
        assert_eq!(sign_element(Sign::Scorpio), ElementKind::Water);
    }

    #[test]
    fn by_sign_overrides_element_scheme() {
        let fallback = color(1.0, 1.0, 1.0);
        let water = color(0.0, 0.0, 1.0);
        let scorpio = color(0.5, 0.1, 0.7);
        let mut theme = Theme::default();
        theme
            .glyphs
            .signs
            .by_element
            .insert("water".to_owned(), GlyphPaint::monochrome(water));
        theme
            .glyphs
            .signs
            .by_sign
            .insert("scorpio".to_owned(), GlyphPaint::monochrome(scorpio));

        let resolved = resolve_sign_glyph_paint(&theme, Sign::Scorpio, fallback);
        assert_eq!(resolved.color, Some(scorpio));
    }

    #[test]
    fn occupant_override_beats_type_default() {
        let fallback = color(1.0, 1.0, 1.0);
        let body = color(0.2, 0.2, 0.2);
        let sun = color(1.0, 0.8, 0.0);
        let mut theme = Theme::default();
        theme
            .glyphs
            .occupants
            .by_type
            .insert("body".to_owned(), GlyphPaint::monochrome(body));
        theme
            .glyphs
            .occupants
            .by_occupant
            .insert("sun".to_owned(), GlyphPaint::monochrome(sun));

        let resolved = resolve_occupant_glyph_paint(&theme, Occupant::Body(Body::Sun), fallback);
        assert_eq!(resolved.color, Some(sun));
    }
}
