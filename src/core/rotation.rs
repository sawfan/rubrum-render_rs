use rubrum::{Angle, Chart, House, Occupant};

use crate::core::geometry::normalize_deg;

/// Compute the chart rotation (in degrees) for a wheel render.
///
/// Convention:
/// - ASC is placed at 180° (left side).
/// - If ASC is unavailable, we render unrotated.
///
/// This logic is shared between backends.
pub fn chart_rotation_deg(chart: &Chart) -> f64 {
    // Prefer explicit house cusp data when available.
    let asc = chart.house_cusp(House::First).or_else(|| {
        chart
            .placements_of(Occupant::Angle(Angle::Ascendant))
            .into_iter()
            .find_map(|p| p.coordinate.sign_degree())
    });

    // If there is no Ascendant data, keep the wheel unrotated.
    let Some(asc) = asc else {
        return 0.0;
    };

    // Place Ascendant at 180° (left side).
    normalize_deg(180.0 - asc.degrees)
}
