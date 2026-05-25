use crate::error::ChartRenderError;
use crate::layout::Layout;
use crate::thickness::ThicknessSpec;

/// Resolved radial geometry for a single band.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BandGeometry {
    pub r_inner: f64,
    pub r_outer: f64,
    pub thickness_px: f64,

    /// The width of the boundary stroke that is shared with the previous band's inner boundary.
    ///
    /// This is useful for ticks/connectors that want to align relative to that boundary.
    pub outer_shared_boundary_width: f64,
}

/// Resolve per-band thickness in px given a `base_r_outer`.
///
/// Layout thickness specs support:
/// - absolute px thicknesses
/// - fractional weights (share remaining radius)
///
/// `ThicknessSpec::Fit` is currently rejected because it requires a layout pass.
pub fn resolve_band_thicknesses(
    layout: &Layout,
    base_r_outer: f64,
) -> Result<Vec<f64>, ChartRenderError> {
    if base_r_outer <= 0.0 {
        return Err(ChartRenderError::InvalidSpec(
            "Output size/margin leaves no drawable radius".to_owned(),
        ));
    }

    let mut abs_total = 0.0;
    let mut frac_total = 0.0;

    // First pass: validate + collect absolute thicknesses and fractional weights.
    for band in &layout.bands {
        match band.thickness {
            ThicknessSpec::Px(px) => {
                if px <= 0.0 {
                    return Err(ChartRenderError::InvalidSpec(format!(
                        "Band '{}' has non-positive thickness",
                        band.id
                    )));
                }
                abs_total += px;
            }
            ThicknessSpec::Abs { abs } => {
                if abs <= 0.0 {
                    return Err(ChartRenderError::InvalidSpec(format!(
                        "Band '{}' has non-positive thickness",
                        band.id
                    )));
                }
                abs_total += abs;
            }
            ThicknessSpec::Frac { frac } => {
                if frac <= 0.0 {
                    return Err(ChartRenderError::InvalidSpec(format!(
                        "Band '{}' has non-positive fractional thickness weight",
                        band.id
                    )));
                }
                frac_total += frac;
            }
            ThicknessSpec::Fit { fit } => {
                if fit {
                    return Err(ChartRenderError::InvalidSpec(format!(
                        "Band '{}' uses fit thickness, which is not yet supported",
                        band.id
                    )));
                }
                return Err(ChartRenderError::InvalidSpec(format!(
                    "Band '{}' has invalid thickness spec",
                    band.id
                )));
            }
        }
    }

    let remaining = base_r_outer - abs_total;
    if remaining < 0.0 {
        return Err(ChartRenderError::InvalidSpec(format!(
            "Total absolute band thickness ({abs_total}) exceeds available radius ({base_r_outer})"
        )));
    }

    if frac_total > 0.0 && remaining <= 0.0 {
        return Err(ChartRenderError::InvalidSpec(
            "Fractional bands but no remaining radius".to_owned(),
        ));
    }

    // Second pass: resolve final thicknesses per band in-order.
    let mut out: Vec<f64> = Vec::with_capacity(layout.bands.len());
    for band in &layout.bands {
        let px = match band.thickness {
            ThicknessSpec::Px(px) => px,
            ThicknessSpec::Abs { abs } => abs,
            ThicknessSpec::Frac { frac } => {
                if frac_total > 0.0 {
                    remaining * (frac / frac_total)
                } else {
                    0.0
                }
            }
            ThicknessSpec::Fit { .. } => 0.0,
        };
        out.push(px);
    }

    Ok(out)
}

/// Plan per-band radii from the resolved thicknesses.
///
/// This is useful for renderers that want to do multiple passes (structure → aspects → glyphs)
/// while re-using the same computed radii.
pub fn plan_band_geometries(
    layout: &Layout,
    base_r_outer: f64,
    band_thicknesses_px: &[f64],
    default_boundary_width: f64,
) -> Result<Vec<BandGeometry>, ChartRenderError> {
    if layout.bands.len() != band_thicknesses_px.len() {
        return Err(ChartRenderError::InvalidSpec(format!(
            "Band thicknesses length mismatch: bands={} thicknesses={}",
            layout.bands.len(),
            band_thicknesses_px.len()
        )));
    }

    let mut out: Vec<BandGeometry> = Vec::with_capacity(layout.bands.len());

    let mut r_outer = base_r_outer;
    let mut prev_inner_boundary_width = 0.0;

    for (band, band_thickness_px) in layout.bands.iter().zip(band_thicknesses_px.iter().copied()) {
        if band_thickness_px <= 0.0 {
            return Err(ChartRenderError::InvalidSpec(format!(
                "Band '{}' has non-positive thickness",
                band.id
            )));
        }

        let r_inner = (r_outer - band_thickness_px).max(0.0);

        out.push(BandGeometry {
            r_inner,
            r_outer,
            thickness_px: band_thickness_px,
            outer_shared_boundary_width: prev_inner_boundary_width,
        });

        prev_inner_boundary_width = band
            .boundary
            .as_ref()
            .map(|b| b.width.unwrap_or(default_boundary_width))
            .unwrap_or(0.0);

        r_outer = r_inner;
    }

    Ok(out)
}

/// Compute a lane's inner/outer radii given the band's outer radius and the lane thickness.
pub fn lane_radii(r_outer: f64, lane_thickness: f64, lane_idx: usize) -> (f64, f64) {
    let lane_r_outer = r_outer - lane_thickness * (lane_idx as f64);
    let lane_r_inner = (lane_r_outer - lane_thickness).max(0.0);
    (lane_r_inner, lane_r_outer)
}
