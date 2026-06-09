use crate::glyph_paint::GlyphTheme;
use crate::options::{ChartCairoOptions, RgbaColor};
use crate::style::LaneTemplate;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Base palette for charts.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BaseColors {
    pub background: RgbaColor,
    pub foreground: RgbaColor,
    pub muted: RgbaColor,
}

/// Additional theme-level color roles.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ColorRoles {
    pub text: Option<RgbaColor>,
    pub structure: Option<RgbaColor>,
    pub ticks: Option<RgbaColor>,
}

/// Pure-SVG backend theme options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SvgThemeOptions {
    /// Optional external SVG sprite sheet URL.
    pub glyph_sprite_url: Option<String>,

    /// If true, render outer-wheel sign labels using sprite `<use>` glyphs when
    /// `glyph_sprite_url` is set.
    pub use_sprite_sign_labels: bool,

    /// If true, render the chart so the SVG canvas is a tight square around the wheel.
    ///
    /// Concretely, the SVG backend will:
    /// - ignore the render-plan padding (`canvas_pad`) that is normally used to prevent glyph
    ///   clipping in other contexts
    /// - render into a tight *square* canvas using `min(theme.cairo.width, theme.cairo.height)`
    ///   with the wheel centered at `(side/2, side/2)`
    ///
    /// Notes:
    /// - Use `fill_canvas_margin_px` to keep a small safety margin between the outer boundary and
    ///   the viewBox edge. Set it to `0` for the closest possible fit.
    /// - Outward-facing ticks or labels may be clipped if they extend beyond the outer boundary.
    pub fill_canvas: bool,

    /// Extra margin (in px) to keep between the wheel and the viewBox edge when `fill_canvas` is
    /// enabled.
    ///
    /// This is SVG-backend-specific and intentionally does **not** reuse `theme.cairo.margin`.
    pub fill_canvas_margin_px: f64,

    /// Optional styling overrides for the aspect grid ("aspect table") render target.
    ///
    /// When omitted, the SVG backend should derive sensible defaults from the theme's
    /// base palette (light/dark) and color roles.
    pub aspect_grid: Option<AspectGridTheme>,

    /// Optional styling overrides for the declination-map render target.
    ///
    /// Like the aspect grid, this rectangular chart projection inherits from the main chart
    /// theme by default while allowing dedicated export/theme tuning.
    pub declination_map: Option<DeclinationMapTheme>,
}

/// Color palette for declination-map rendering.
///
/// These roles intentionally mirror the visual primitives of a rectangular coordinate plot while
/// remaining compatible with the broader chart theme.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DeclinationMapTheme {
    /// Overall SVG canvas background.
    pub canvas_bg: Option<RgbaColor>,

    /// Inner plot background.
    pub plot_bg: Option<RgbaColor>,

    /// Grid line color.
    pub grid_line: Option<RgbaColor>,

    /// Celestial equator line color.
    pub equator: Option<RgbaColor>,

    /// Ecliptic/Solar declination reference curve color.
    pub ecliptic: Option<RgbaColor>,

    /// Tropic/out-of-bounds boundary line color.
    pub tropic: Option<RgbaColor>,

    /// Text/axis/sign-label color.
    pub text: Option<RgbaColor>,

    /// Shaded out-of-bounds band color.
    pub out_of_bounds_band: Option<RgbaColor>,
}

/// Color palette for aspect-grid rendering.
///
/// These are intentionally limited to "background + lines + text" so downstream apps can:
/// - inherit from the chart theme by default
/// - override just the aspect grid without affecting the chart
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AspectGridTheme {
    /// Overall SVG canvas background.
    pub canvas_bg: Option<RgbaColor>,

    /// Default cell background.
    pub cell_bg: Option<RgbaColor>,

    /// Grid line (stroke) color.
    pub grid_line: Option<RgbaColor>,

    /// Text color used for labels.
    pub text: Option<RgbaColor>,
}

impl Default for SvgThemeOptions {
    fn default() -> Self {
        // Default to sprite sign labels when available.
        //
        // Unicode zodiac symbols vary wildly across platforms/fonts (and often fall back to
        // inconsistent glyphs). Using the sprite sheet keeps sign labels visually consistent with
        // the rest of the glyph system.
        Self {
            glyph_sprite_url: None,
            use_sprite_sign_labels: true,
            fill_canvas: false,
            fill_canvas_margin_px: 0.0,
            aspect_grid: None,
            declination_map: None,
        }
    }
}

impl Default for BaseColors {
    fn default() -> Self {
        Self {
            background: RgbaColor {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            foreground: RgbaColor {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            muted: RgbaColor {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            },
        }
    }
}

/// Light/dark mode for theme selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    #[default]
    Light,
    Dark,
}

/// Select between a light and dark base palette.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorModeSelector {
    pub mode: ColorMode,
    pub light: Option<BaseColors>,
    pub dark: Option<BaseColors>,

    /// If true, apply the selected base background color to `theme.cairo.background` at render
    /// time.
    #[serde(default = "default_true")]
    pub apply_background_to_cairo: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ColorModeSelector {
    fn default() -> Self {
        Self {
            mode: ColorMode::Light,
            light: None,
            dark: None,
            apply_background_to_cairo: default_true(),
        }
    }
}

fn default_dark_base_colors() -> BaseColors {
    BaseColors {
        background: RgbaColor {
            r: 0.08,
            g: 0.08,
            b: 0.09,
            a: 1.0,
        },
        foreground: RgbaColor {
            r: 0.95,
            g: 0.95,
            b: 0.95,
            a: 1.0,
        },
        muted: RgbaColor {
            r: 0.65,
            g: 0.65,
            b: 0.65,
            a: 1.0,
        },
    }
}

/// Visual defaults and reusable templates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Theme {
    #[serde(default)]
    pub cairo: ChartCairoOptions,

    /// Pure-SVG backend configuration.
    #[serde(default)]
    pub svg: SvgThemeOptions,

    /// Optional theme-level light/dark selector for base chart colors.
    #[serde(default)]
    pub color_mode: Option<ColorModeSelector>,

    /// Optional additional color-role defaults.
    #[serde(default)]
    pub colors: ColorRoles,

    /// Glyph paint/color customization rules shared by SVG renderers.
    #[serde(default)]
    pub glyphs: GlyphTheme,

    /// Rendering configuration for aspect lines.
    #[serde(default)]
    pub aspects: crate::aspects::AspectsStyle,

    /// Reusable lane templates.
    #[serde(default)]
    pub templates: BTreeMap<String, LaneTemplate>,

    /// Per-dataset color mapping.
    #[serde(default)]
    pub dataset_colors: BTreeMap<String, RgbaColor>,
}

impl Theme {
    pub fn effective_text_color(&self) -> RgbaColor {
        let base = self.effective_base_colors();
        self.colors.text.unwrap_or(base.foreground)
    }

    pub fn effective_structure_color(&self) -> RgbaColor {
        let base = self.effective_base_colors();
        self.colors.structure.unwrap_or(base.muted)
    }

    pub fn effective_ticks_color(&self) -> RgbaColor {
        let base = self.effective_base_colors();
        self.colors.ticks.unwrap_or(base.muted)
    }

    pub fn effective_base_colors(&self) -> BaseColors {
        let Some(sel) = self.color_mode.as_ref() else {
            return BaseColors::default();
        };

        match sel.mode {
            ColorMode::Light => sel.light.unwrap_or_default(),
            ColorMode::Dark => sel.dark.unwrap_or_else(default_dark_base_colors),
        }
    }

    pub fn effective_cairo_background(&self) -> RgbaColor {
        if let Some(sel) = self.color_mode.as_ref()
            && sel.apply_background_to_cairo
        {
            return self.effective_base_colors().background;
        }

        self.cairo.background
    }
}
