# rubrum_render

Backend-agnostic, spec-driven astrology chart rendering core.

This crate contains the shared **spec types** (`Theme`, `Layout`, `ChartData`), **planning** (`RenderPlan`), and small utilities that are intended to stay stable across rendering backends.

It is used by:

- `rubrum_cairo` — native Cairo renderer
- `rubrum_svg` — pure-SVG renderer

## Status / scope

- The planner currently returns a **scaffold** `RenderPlan` (canvas + basic geometry) as the planning pipeline is migrated out of renderers.
- The crate already contains a number of stable spec/data primitives and SVG emission helpers.

## Features

From `Cargo.toml`:

- `default = []` — no backend features enabled by default
- `cairo` — enables Cairo-specific conveniences by depending on `cairo-rs`

## What’s in this crate

High-level modules:

- `theme` — `Theme`, base palettes, color roles, light/dark selectors; backend options (`Theme::cairo`, `Theme::svg`)
- `layout` — spec types describing bands/lanes, ticks, houses/signs, and glyph lanes
- `chart_data` — `ChartData` (datasets, house sets, placements/cusps) with legacy adapters
- `core` — shared geometry and planning
  - `core::render_plan` — `RenderPlan` and `plan_chart_spec`
  - `core::geometry` / `core::radial` — geometry helpers
  - `core::rotation` — chart rotation helpers
- `aspects` — aspect style/spec helpers shared by backends
- `svg` — backend-agnostic SVG string emission helpers (`<circle>`, `<line>`, `<use>`, hit targets, etc.)
- `style` — reusable style primitives/templates (lane templates, etc.)
- `options` — shared render options and primitives (`ChartCairoOptions`, `RgbaColor`, glyph-set selection)
- `glyphs` — stable glyph/symbol ID helpers
- `dataset` — dataset and house-set containers used by `ChartData`
- `labels` — label templating helpers
- `metadata` — SVG metadata helpers
- `embedded_configs` — embedded TOML demo defaults (themes + example chart/layout specs)

## Public API highlights

Re-exports in `lib.rs` are intentionally broad to make downstream usage convenient.

Common entrypoints:

- `plan_chart_spec(theme, layout, data) -> Result<RenderPlan, ChartRenderError>`
- `Theme`, `Layout`, `ChartData`

## Embedded demo configs

The `config/` directory contains TOML configs that are embedded at compile time via `embedded_configs`:

- `config/theme_dark.toml`
- `config/theme_light.toml`
- `config/chart_spec_natal_layout_only.toml`
- `config/chart_spec_natal_data.toml`
- `config/chart_spec_natal_aspects.toml`

These are intended as versioned defaults for examples/editors/viewers (including WASM builds), not as a general runtime configuration mechanism.

## Verification

```sh
RUSTFLAGS='-Dwarnings' cargo check -q
RUSTFLAGS='-Dwarnings' cargo test  -q
```

