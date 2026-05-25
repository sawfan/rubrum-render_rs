use serde::{Deserialize, Serialize};

use crate::chart_data::ChartData;
use crate::core::rotation::chart_rotation_deg;
use crate::error::ChartRenderError;
use crate::layout::Layout;
use crate::options::RgbaColor;
use crate::theme::Theme;

/// Backend-agnostic description of what to draw.
///
/// This is the shared output of the geometry/layout planning stage.
///
/// NOTE: This is intentionally minimal right now (scaffold). We will extend it incrementally to
/// include rings, ticks, glyph placement positions, aspect routing, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderPlan {
    /// Canvas size in pixels.
    pub canvas: CanvasSpec,

    /// Background fill (if any).
    pub background: Option<FillSpec>,

    /// Default foreground/text color for plan-driven renderers.
    ///
    /// This is intended for placeholders and debug output (e.g. the SVG scaffold).
    pub foreground: Option<RgbaColor>,

    /// Computed padding applied around the chart content to prevent glyph clipping.
    pub canvas_pad: i32,

    /// Chart center in output coordinates.
    pub center: Point,

    /// Chart rotation in degrees.
    pub rotation_deg: f64,

    /// Outer radius of the wheel (before band thickness subdivisions).
    pub base_r_outer: f64,

    pub debug_label: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CanvasSpec {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FillSpec {
    pub color: RgbaColor,
}

impl RenderPlan {
    /// Placeholder plan used until the planner is fully implemented.
    ///
    /// This intentionally mirrors the current Cairo renderer defaults so we can migrate logic from
    /// the renderer into the planner without changing outputs.
    pub fn scaffold(
        theme: &Theme,
        layout: &Layout,
        data: &ChartData,
    ) -> Result<Self, ChartRenderError> {
        let cairo = &theme.cairo;
        let bg = theme.effective_cairo_background();

        // Allocate extra room so wide glyph metrics don't get clipped by viewers/rasterizers.
        //
        // Keep this padding symmetric so the wheel stays centered.
        //
        // When `theme.svg.fill_canvas` is enabled, we intentionally render into a tight
        // `{width} x {height}` square (with no extra padding) so the wheel can be composed with
        // other SVG elements without unexpected whitespace.
        let fill_canvas = theme.svg.fill_canvas;

        let pad_font = cairo.sign_font_size.max(cairo.label_font_size);
        let canvas_pad = if fill_canvas {
            0
        } else {
            ((pad_font * 4.0) as i32).max(cairo.margin)
        };

        // For a tight SVG composition canvas, make the output square so the wheel can touch the
        // edges on all sides.
        let side = cairo.width.min(cairo.height);

        let canvas_width = if fill_canvas { side } else { cairo.width } + (canvas_pad * 2);

        let canvas_height = if fill_canvas { side } else { cairo.height } + (canvas_pad * 2);

        // Match Cairo's current center computation.
        let (cx, cy) = if fill_canvas {
            let c = (side as f64) / 2.0;
            (c, c)
        } else {
            (
                (canvas_pad + (cairo.width / 2)) as f64,
                (canvas_height / 2) as f64,
            )
        };

        // Rotation currently comes from the legacy `rubrum::Chart` adapter.
        let legacy_chart = data.to_legacy_chart();
        let rotation_deg = chart_rotation_deg(&legacy_chart);

        // Compute the outer drawable radius.
        //
        // For `fill_canvas`, the goal is: the *outer edge* of the outer boundary stroke should be
        // as close to the viewBox edge as possible, while still being fully contained.
        //
        // This means subtracting:
        // - `theme.svg.fill_canvas_margin_px` (optional user-controlled safety margin)
        // - half the *actual* outer boundary stroke width (if any)
        let min_side = (cairo.width.min(cairo.height)) as f64;

        let outer_boundary_width = layout
            .bands
            .first()
            .and_then(|b| b.boundary.as_ref())
            .and_then(|b| b.width)
            .unwrap_or(cairo.stroke_width);

        let base_r_outer = if fill_canvas {
            (min_side / 2.0)
                - theme.svg.fill_canvas_margin_px.max(0.0)
                - (outer_boundary_width.max(0.0) / 2.0)
        } else {
            (min_side / 2.0) - (cairo.margin as f64)
        };

        if base_r_outer <= 0.0 {
            return Err(ChartRenderError::InvalidSpec(
                "Output size/margin leaves no drawable radius".to_owned(),
            ));
        }

        // Avoid unused-parameter warnings while this is scaffolded.
        let _ = layout;

        Ok(Self {
            canvas: CanvasSpec {
                width: canvas_width as f64,
                height: canvas_height as f64,
            },
            background: Some(FillSpec { color: bg }),
            foreground: Some(theme.effective_text_color()),
            canvas_pad,
            center: Point { x: cx, y: cy },
            rotation_deg,
            base_r_outer,
            debug_label: Some("render plan scaffold".to_string()),
        })
    }
}

/// Plan a chart render from spec inputs.
///
/// In the future this will:
/// - validate theme/layout/data
/// - compute radii + band geometry
/// - compute tick positions
/// - compute collision-avoided placement angles and label positions
/// - compute aspect line routing and label midpoints
pub fn plan_chart_spec(
    theme: &Theme,
    layout: &Layout,
    data: &ChartData,
) -> Result<RenderPlan, ChartRenderError> {
    RenderPlan::scaffold(theme, layout, data)
}
