use serde::{Deserialize, Serialize};

use crate::style::LaneTemplate;
use crate::thickness::ThicknessSpec;

/// Layout determines band order, template usage, overrides, and dataset binding.
///
/// Strict responsibility:
/// - Band order
/// - Template usage
/// - Overrides
/// - Dataset binding
/// - No placements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Layout {
    #[serde(default)]
    pub bands: Vec<BandSpec>,
}

impl Layout {
    /// Validate strict responsibility rules.
    ///
    /// - Templates must NOT bind datasets (enforced by type system).
    /// - Dataset-driven glyph lanes must declare a dataset binding.
    /// - Structural glyph lanes must NOT declare a dataset binding.
    pub fn validate(&self) -> Result<(), String> {
        for band in &self.bands {
            for lane in &band.lanes {
                let Some(glyphs) = &lane.glyphs else {
                    if lane.dataset.is_some() {
                        return Err(format!(
                            "Lane in band '{}' binds dataset but has no glyphs spec",
                            band.id
                        ));
                    }
                    if lane.house_set.is_some() {
                        return Err(format!(
                            "Lane in band '{}' binds house_set but has no glyphs spec",
                            band.id
                        ));
                    }
                    continue;
                };

                match glyphs.mode {
                    GlyphLaneMode::Bodies => {
                        if lane.dataset.is_none() {
                            return Err(format!(
                                "Lane in band '{}' has Bodies glyphs but no dataset binding",
                                band.id
                            ));
                        }
                        if lane.house_set.is_some() {
                            return Err(format!(
                                "Lane in band '{}' has Bodies glyphs but also binds house_set",
                                band.id
                            ));
                        }
                    }
                    GlyphLaneMode::Aspects => {
                        if lane.dataset.is_none() {
                            return Err(format!(
                                "Lane in band '{}' has Aspects glyphs but no dataset binding",
                                band.id
                            ));
                        }
                        if lane.house_set.is_some() {
                            return Err(format!(
                                "Lane in band '{}' has Aspects glyphs but also binds house_set",
                                band.id
                            ));
                        }
                    }
                    GlyphLaneMode::HouseNumbers => {
                        if lane.dataset.is_some() {
                            return Err(format!(
                                "Lane in band '{}' has HouseNumbers glyphs but binds a dataset",
                                band.id
                            ));
                        }
                        // `house_set` is optional (defaults to "natal").
                    }
                    GlyphLaneMode::CrossAspects => {
                        if lane.dataset.is_none() {
                            return Err(format!(
                                "Lane in band '{}' has CrossAspects glyphs but no dataset binding",
                                band.id
                            ));
                        }
                        if glyphs.other_dataset.is_none() {
                            return Err(format!(
                                "Lane in band '{}' has CrossAspects glyphs but no other_dataset",
                                band.id
                            ));
                        }
                        if lane.house_set.is_some() {
                            return Err(format!(
                                "Lane in band '{}' has CrossAspects glyphs but also binds house_set",
                                band.id
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BandSpec {
    pub id: String,

    /// Band thickness spec.
    ///
    /// A bare number is interpreted as an absolute thickness in px.
    pub thickness: ThicknessSpec,

    /// Subdivided radial tracks inside this band.
    #[serde(default)]
    pub lanes: Vec<LaneSpec>,

    /// Optional band fill (structural; no dataset binding).
    #[serde(default)]
    pub fill: Option<crate::options::RgbaColor>,

    /// Optional band boundary stroke.
    #[serde(default)]
    pub boundary: Option<StrokeSpec>,

    /// Optional tick spec (ticks on the inner boundary: towards the chart center).
    ///
    /// This is the recommended field for "classic" charts where there is only one tick system.
    #[serde(default, alias = "ticks")]
    pub ticks_inner: Option<TicksSpec>,

    /// Optional tick spec on the outer boundary (away from the chart center).
    ///
    /// This allows rendering ticks on both the inner and outer boundaries of the same band.
    ///
    /// Backward compatibility:
    /// - The old `ticks` key deserializes into `ticks_inner`.
    /// - The old `ticks2` key deserializes into `ticks_outer`.
    #[serde(default, alias = "ticks2")]
    pub ticks_outer: Option<TicksSpec>,

    /// Optional house divisions rendering (cusp spokes + house numbers) within this band.
    #[serde(default)]
    pub houses: Option<HousesSpec>,

    /// Optional sign wheel rendering within this band.
    #[serde(default)]
    pub signs: Option<SignsSpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LaneSpec {
    /// Optional identifier for diagnostics.
    #[serde(default)]
    pub id: Option<String>,

    /// Optional template name (declared in Theme).
    #[serde(default)]
    pub template: Option<String>,

    /// Optional dataset binding.
    ///
    /// Dataset binding is only allowed on dataset-driven glyph/text lanes.
    #[serde(default)]
    pub dataset: Option<String>,

    /// Optional house cusp set binding.
    ///
    /// This is used by structural glyph lanes such as `GlyphLaneMode::HouseNumbers`.
    ///
    /// Defaults to "natal" (with `ChartData.house_cusps` as the legacy fallback).
    #[serde(default)]
    pub house_set: Option<String>,

    /// Glyph/text rendering spec for this lane.
    #[serde(default)]
    pub glyphs: Option<GlyphLaneSpec>,

    /// Optional per-lane endpoint filter.
    ///
    /// When present, this filter is applied to the dataset placements before any rendering occurs.
    /// This can be used to selectively show only certain bodies/angles/points without having to
    /// pre-filter the input dataset.
    #[serde(default)]
    pub endpoint_filter: Option<rubrum::EndpointFilter>,

    /// Optional inline overrides applied after template.
    #[serde(default)]
    pub overrides: LaneTemplate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrokeSpec {
    /// Stroke color.
    ///
    /// When omitted, renderers should fall back to theme defaults.
    #[serde(default)]
    pub color: Option<crate::options::RgbaColor>,

    #[serde(default)]
    pub width: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TicksSpec {
    /// If true, draw 1° tick marks.
    #[serde(default)]
    pub enabled: bool,

    /// Which boundary to anchor ticks on.
    #[serde(default)]
    pub anchor: Option<TickAnchor>,

    /// Which side(s) of the anchor boundary the tick segments should extend.
    #[serde(default)]
    pub direction: Option<TickDirection>,

    #[serde(default)]
    pub stroke: Option<crate::options::RgbaColor>,

    /// Optional major-tick length (in px) on the inward side of the anchor boundary.
    #[serde(default)]
    pub length_in: Option<f64>,

    /// Optional major-tick length (in px) on the outward side of the anchor boundary.
    #[serde(default)]
    pub length_out: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HousesSpec {
    /// If true, draw house cusp spokes and (optionally) house numbers.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// If true, render house numbers in the houses phase.
    ///
    /// When using `GlyphLaneMode::HouseNumbers`, set this to `false` to avoid double-rendering.
    #[serde(default = "default_true")]
    pub numbers: bool,

    /// Optional house set id to select which cusps to render.
    #[serde(default)]
    pub house_set: Option<String>,

    /// Optional placement options for house numbers.
    #[serde(default)]
    pub number_placement: HouseNumberPlacementSpec,

    /// If true, draw cusp spokes all the way to the center of the wheel.
    #[serde(default)]
    pub spoke_to_center: bool,

    /// Optional major-axis lines (Asc/Desc and MC/IC).
    #[serde(default)]
    pub axes: Option<HouseAxesSpec>,

    /// Optional spoke stroke spec.
    #[serde(default)]
    pub spoke: Option<StrokeSpec>,

    /// Legacy: spoke stroke color.
    #[serde(default)]
    pub spoke_stroke: Option<crate::options::RgbaColor>,

    /// Legacy: spoke line width in px.
    #[serde(default)]
    pub spoke_width: Option<f64>,

    /// Optional house number color.
    #[serde(default)]
    pub number_color: Option<crate::options::RgbaColor>,

    /// Optional base font size for house numbers.
    #[serde(default)]
    pub number_font_size: Option<f64>,
}

impl Default for HousesSpec {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            numbers: default_true(),
            house_set: None,
            number_placement: HouseNumberPlacementSpec::default(),
            spoke_to_center: false,
            axes: None,
            spoke: None,
            spoke_stroke: None,
            spoke_width: None,
            number_color: None,
            number_font_size: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HouseNumberPlacementSpec {
    /// Where to place the house number relative to the house span.
    ///
    /// - `Midpoint` (default): place at the midpoint between this cusp and the next cusp.
    /// - `CuspStart`: place near the cusp spoke itself (the "first degree" of the house).
    #[serde(default)]
    pub mode: HouseNumberPlacementMode,

    /// Which corner of the band to pin the label to when using `CuspStart`.
    #[serde(default)]
    pub corner: HouseNumberCorner,

    /// Radial padding (px) from the chosen corner boundary.
    ///
    /// For `corner = Outer`, this is subtracted from `r_outer`.
    /// For `corner = Inner`, this is added to `r_inner`.
    #[serde(default = "default_house_number_radial_padding")]
    pub radial_padding: f64,

    /// Angular offset in degrees applied to the base cusp angle.
    ///
    /// This is useful to push the label slightly into the "corner" of a band (so it doesn't sit
    /// directly on top of the cusp spoke).
    #[serde(default)]
    pub angle_offset_deg: f64,
}

fn default_house_number_radial_padding() -> f64 {
    6.0
}

impl Default for HouseNumberPlacementSpec {
    fn default() -> Self {
        Self {
            mode: HouseNumberPlacementMode::Midpoint,
            corner: HouseNumberCorner::Outer,
            radial_padding: default_house_number_radial_padding(),
            angle_offset_deg: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum HouseNumberPlacementMode {
    #[default]
    Midpoint,
    CuspStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum HouseNumberCorner {
    Inner,
    #[default]
    Outer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HouseAxesSpec {
    /// If true, draw configured axes.
    #[serde(default)]
    pub enabled: bool,

    /// If true, draw the Ascendant/Descendant axis.
    #[serde(default = "default_true")]
    pub asc_desc: bool,

    /// If true, draw the MC/IC axis.
    #[serde(default = "default_true")]
    pub mc_ic: bool,

    /// If true, draw axes all the way to the center of the wheel.
    #[serde(default)]
    pub to_center: bool,

    /// Optional axis stroke spec.
    #[serde(default)]
    pub stroke: Option<StrokeSpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SignsSpec {
    /// If true, draw sign dividers and/or labels in this band.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Draw 30° divider segments spanning the band's thickness.
    #[serde(default = "default_true")]
    pub dividers: bool,

    /// Draw sign labels at the midpoints between dividers.
    #[serde(default = "default_true")]
    pub labels: bool,

    /// Optional divider stroke color.
    #[serde(default)]
    pub divider_stroke: Option<crate::options::RgbaColor>,

    /// Optional divider line width in px.
    #[serde(default)]
    pub divider_width: Option<f64>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TickAnchor {
    Inner,
    Outer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TickDirection {
    Inward,
    Outward,
    Both,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeclinationRadialPlacementSpec {
    /// If true, vary body glyph radius inside this lane from apparent equatorial declination.
    #[serde(default)]
    pub enabled: bool,

    /// Theoretical declination used for normalization. Values beyond this are clamped.
    /// A 30° default keeps ordinary planets visible without overreacting to outliers.
    #[serde(default)]
    pub max_declination_deg: Option<f64>,

    /// Fraction of usable half-lane travel to use. `1.0` may touch lane padding;
    /// lower values are calmer. Default is intentionally subtle.
    #[serde(default)]
    pub strength: Option<f64>,

    /// Soft compression curve. Larger values push high declinations toward the lane edge
    /// while retaining visible separation near the center.
    #[serde(default)]
    pub curve: Option<f64>,

    /// Padding from lane boundaries in px before mapping declination.
    #[serde(default)]
    pub padding_px: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GlyphLaneSpec {
    #[serde(default)]
    pub mode: GlyphLaneMode,

    /// Secondary dataset binding used by glyph modes that compare two datasets.
    ///
    /// For example, `GlyphLaneMode::CrossAspects` uses:
    /// - `LaneSpec.dataset` as the "from" dataset
    /// - `GlyphLaneSpec.other_dataset` as the "to" dataset
    #[serde(default)]
    pub other_dataset: Option<String>,

    #[serde(default)]
    pub radial_bias: Option<f64>,

    /// Optional declination-aware radial placement within the glyph lane.
    #[serde(default)]
    pub declination_radial: Option<DeclinationRadialPlacementSpec>,

    #[serde(default)]
    pub collision_avoidance: Option<CollisionAvoidanceSpec>,

    /// Optional tick marks drawn at each placement.
    #[serde(default)]
    pub placement_ticks: Option<PlacementTicksSpec>,

    /// Optional tick marks drawn at each placement *boundary*.
    ///
    /// Boundaries come from dataset-driven `placement_boundary_ticks` (e.g. aspects anchors).
    #[serde(default)]
    pub placement_boundary_ticks: Option<PlacementBoundaryTicksSpec>,

    /// Optional per-placement label segments.
    #[serde(default)]
    pub placement_labels: Option<PlacementLabelsSpec>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum GlyphLaneMode {
    #[default]
    Bodies,
    HouseNumbers,

    /// Aspect lines drawn between endpoints in one dataset.
    Aspects,

    /// Aspect lines drawn between endpoints in two datasets (e.g. transits → natal).
    CrossAspects,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CollisionAvoidanceSpec {
    /// If true, enable collision avoidance behavior.
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementTicksSpec {
    #[serde(default)]
    pub enabled: bool,

    /// Which lane boundary to anchor the tick on.
    ///
    /// Defaults to `Outer` so existing charts keep the original "connector tick from the outer
    /// lane wall" behavior.
    #[serde(default)]
    pub anchor: Option<TickAnchor>,

    /// Which direction the tick should extend.
    ///
    /// Defaults to `Inward` for backward compatibility.
    #[serde(default)]
    pub direction: Option<TickDirection>,

    /// Stroke color for the connector tick.
    #[serde(default)]
    pub stroke: Option<crate::options::RgbaColor>,

    /// Stroke width in px.
    #[serde(default)]
    pub width: Option<f64>,

    /// Connector length in px.
    #[serde(default)]
    pub length: Option<f64>,

    /// How far to inset the connector's *start* from the chosen anchor boundary.
    #[serde(default)]
    pub offset: Option<f64>,

    /// Gap between the connector's end and the placement glyph/label anchor.
    #[serde(default)]
    pub end_gap: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementBoundaryTicksSpec {
    #[serde(default)]
    pub enabled: bool,

    /// Which band boundary to draw ticks around.
    ///
    /// - Inner: draw around the band’s inner boundary (`r_inner`).
    /// - Outer: draw around the band’s outer boundary (`r_outer`).
    ///
    /// Defaults to `Outer` for backward compatibility.
    #[serde(default)]
    pub anchor: Option<TickAnchor>,

    #[serde(default)]
    pub direction: Option<TickDirection>,

    #[serde(default)]
    pub stroke: Option<crate::options::RgbaColor>,

    #[serde(default)]
    pub width: Option<f64>,

    #[serde(default)]
    pub length_in: Option<f64>,

    #[serde(default)]
    pub length_out: Option<f64>,

    #[serde(default)]
    pub offset_in: Option<f64>,

    #[serde(default)]
    pub offset_out: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementLabelsSpec {
    #[serde(default)]
    pub enabled: bool,

    /// Which side of the placement glyph to render the label segments.
    ///
    /// - Inner: render inward (towards chart center). This is the historical behavior.
    /// - Outer: render outward (away from chart center). Useful for outer rings (e.g. transits).
    #[serde(default)]
    pub side: Option<PlacementLabelSide>,

    /// Global text style overrides.
    #[serde(default)]
    pub text: Option<TextStyleSpec>,

    /// Back-compat: global font size override.
    #[serde(default)]
    pub font_size: Option<f64>,

    /// Back-compat: global text color override.
    #[serde(default)]
    pub color: Option<crate::options::RgbaColor>,

    /// Radial offset inward from the glyph anchor.
    #[serde(default)]
    pub offset_in: Option<f64>,

    /// Per-segment radial step inward.
    #[serde(default)]
    pub step_in: Option<f64>,

    /// If true, attempt to avoid segment overlap by pushing segments inward.
    #[serde(default)]
    pub collision_avoidance: Option<bool>,

    /// Optional per-segment explicit offsets (in px) inward from the glyph anchor.
    #[serde(default)]
    pub segment_offsets_in: Option<Vec<f64>>,

    /// Default styling for sign glyph segments.
    #[serde(default)]
    pub sign_glyph: Option<PlacementSignGlyphSpec>,

    /// Template segments to render for each placement.
    ///
    /// Supports tokens: {deg}, {min}, {sec}, {sign}, {dms}.
    #[serde(default)]
    pub segments: Vec<PlacementLabelSegmentInput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TextStyleSpec {
    #[serde(default)]
    pub font_family: Option<String>,

    #[serde(default)]
    pub font_size: Option<f64>,

    #[serde(default)]
    pub color: Option<crate::options::RgbaColor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlacementLabelSegmentInput {
    Text(String),
    Spec(PlacementLabelSegmentSpec),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementLabelSegmentSpec {
    pub text: String,

    #[serde(default)]
    pub font_family: Option<String>,

    #[serde(default)]
    pub font_size: Option<f64>,

    #[serde(default)]
    pub color: Option<crate::options::RgbaColor>,

    /// Optional per-segment offset inward from the glyph anchor.
    #[serde(default)]
    pub offset_in: Option<f64>,

    /// Optional sign glyph configuration (used when `text == "{sign_glyph}"`).
    #[serde(default)]
    pub sign_glyph: Option<PlacementSignGlyphSpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PlacementSignGlyphSpec {
    #[serde(default)]
    pub size: Option<f64>,

    #[serde(default)]
    pub color: Option<crate::options::RgbaColor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementLabelSide {
    Inner,
    Outer,
}
