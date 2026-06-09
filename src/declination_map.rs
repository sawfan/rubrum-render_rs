use serde::{Deserialize, Serialize};

/// Layout/configuration for rectangular declination-map chart projections.
///
/// This is intentionally separate from the circular wheel layout: longitude is mapped to the
/// horizontal axis and apparent equatorial declination to the vertical axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeclinationMapLayout {
    pub width: f64,
    pub height: f64,
    pub margin_left: f64,
    pub margin_top: f64,
    pub margin_right: f64,
    pub margin_bottom: f64,
    pub min_declination_deg: f64,
    pub max_declination_deg: f64,
    pub ecliptic_obliquity_deg: f64,
    pub show_equator: bool,
    pub show_tropics: bool,
    pub show_out_of_bounds_bands: bool,
    pub show_ecliptic_curve: bool,
    pub show_sign_blocks: bool,
    pub show_angle_guides: bool,
    pub show_degree_labels: bool,
}

impl Default for DeclinationMapLayout {
    fn default() -> Self {
        Self {
            width: 1200.0,
            height: 560.0,
            margin_left: 72.0,
            margin_top: 34.0,
            margin_right: 48.0,
            margin_bottom: 96.0,
            min_declination_deg: -30.0,
            max_declination_deg: 30.0,
            ecliptic_obliquity_deg: 23.439_291_1,
            show_equator: true,
            show_tropics: true,
            show_out_of_bounds_bands: true,
            show_ecliptic_curve: true,
            show_sign_blocks: true,
            show_angle_guides: true,
            show_degree_labels: true,
        }
    }
}

impl DeclinationMapLayout {
    pub fn plot_width(&self) -> f64 {
        (self.width - self.margin_left - self.margin_right).max(1.0)
    }

    pub fn plot_height(&self) -> f64 {
        (self.height - self.margin_top - self.margin_bottom).max(1.0)
    }
}
