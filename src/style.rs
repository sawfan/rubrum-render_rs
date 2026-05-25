use crate::options::RgbaColor;
use serde::{Deserialize, Serialize};

mod lane_style;

pub use lane_style::resolve_lane_style;

/// Style template applied to lanes.
///
/// Note: templates do not include any dataset binding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LaneTemplate {
    #[serde(default)]
    pub fill: Option<RgbaColor>,

    #[serde(default)]
    pub stroke: Option<RgbaColor>,

    #[serde(default)]
    pub stroke_width: Option<f64>,

    /// Optional text font family override for lane-rendered text.
    #[serde(default)]
    pub font_family: Option<String>,

    /// Optional text font size override for lane-rendered text.
    #[serde(default)]
    pub font_size: Option<f64>,
}
