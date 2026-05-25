# AGENTS

This document is for humans and automated agents working on the **rubrum_render** crate.

The goal is to capture the crate’s purpose, key modules, feature flags, and verification commands so future work (including AI-assisted changes) can be done safely.

---

## 1. Project summary

- **Name:** `rubrum_render`
- **Type:** Rust library crate
- **Domain:** Spec-driven astrology chart rendering (backend-agnostic)
- **Purpose:** Provide the shared render/spec core used by both:
  - `rubrum_cairo` (native Cairo renderer)
  - `rubrum_svg` (pure-SVG renderer)

This crate owns the *spec types* (`Theme`, `Layout`, `ChartData`) and planning/helpers that are intended to remain stable across backends.

---

## 2. Features

From `Cargo.toml`:

- `default = []`
- `cairo` — enables Cairo-specific conveniences (e.g. error conversions) by depending on `cairo-rs`.

Notes:

- The pure-SVG backend (`rubrum_svg`) depends on `rubrum_render` **without** enabling `cairo`.
- `rubrum_cairo` enables `rubrum_render/cairo` via its own `cairo` feature.

---

## 3. Key modules / organization

- `aspects` — aspect render models/helpers used by backends.
- `chart_data` — `ChartData` (datasets, placements, optional computed structures).
- `core/`
  - `render_plan` — spec planning entrypoints (`plan_chart_spec`) and the render plan types.
  - `geometry` — shared geometry helpers.
  - `rotation` — chart rotation helpers.
- `dataset` — `DatasetData`, `HouseSetData`, dataset-related helpers.
- `error` — backend-agnostic render errors (`ChartRenderError`).
- `glyphs` — stable symbol-id helpers for glyph sprites/packs.
- `layout` — `Layout` and band/lane spec types.
- `options` — `ChartCairoOptions`, `CairoOccupantGlyphMode`, and other shared options.
- `style` — generic style primitives.
- `theme` — `Theme` + color roles, base colors, mode selectors.
- `thickness` — shared thickness scaling helpers/types.

Public re-exports are intentionally broad to make downstream usage convenient.

---

## 4. Verification

```sh
RUSTFLAGS='-Dwarnings' cargo check -q
RUSTFLAGS='-Dwarnings' cargo test  -q
```

---

## 5. Recent agent operations

- 2026-04-04: Added a repository-level `README.md` documenting crate purpose, features, key modules, embedded demo configs, and verification commands.
- 2026-05-06: Added glyph sprite/pack lot support (`rb-lot-<canonical_key>`). Implemented `lot_svg_symbol_id` in `src/glyphs.rs` and enabled `[lots]` parsing in the glyph pack sprite builder; updated the solid pack manifest to include `fortune` -> `points/part_of_fortune.svg`.


