//! Backend-agnostic geometry helpers.
//!
//! These helpers are intentionally free of any Cairo/SVG rendering details.

/// Normalize degrees into the range [0, 360).
pub fn normalize_deg(mut deg: f64) -> f64 {
    deg %= 360.0;
    if deg < 0.0 {
        deg += 360.0;
    }
    deg
}

/// Convert a polar coordinate (radius + degrees) to cartesian coordinates.
///
/// Convention:
/// - 0° points to the right
/// - angles increase counter-clockwise
/// - the Y axis is flipped because SVG/Cairo surfaces have Y increasing downward
pub fn polar_to_xy(cx: f64, cy: f64, r: f64, lon_deg: f64) -> (f64, f64) {
    let rad = lon_deg.to_radians();
    (cx + r * rad.cos(), cy - r * rad.sin())
}
