//! Shared metadata schema for renderers and interactive inspectors.
//!
//! The pure-SVG backend emits stable `data-rb-*` attributes to make the output DOM inspectable.
//! Frontends (viewer/editor) should prefer these constants to avoid schema drift.

pub mod svg_data;
