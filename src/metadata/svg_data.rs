//! Stable `data-rb-*` attribute keys used in generated SVG.

/// Generic discriminator key for structure elements.
pub const DATA_RB_STRUCTURE: &str = "data-rb-structure";

/// Band id / name.
pub const DATA_RB_BAND: &str = "data-rb-band";

/// Lane id (layout lane id).
pub const DATA_RB_LANE_ID: &str = "data-rb-lane-id";

/// Axis discriminator (e.g. asc/desc/mc/ic).
pub const DATA_RB_AXIS: &str = "data-rb-axis";

/// Ecliptic longitude in degrees (stringified).
pub const DATA_RB_DEG: &str = "data-rb-deg";

/// Sign index (0..11).
pub const DATA_RB_SIGN_INDEX: &str = "data-rb-sign-index";

/// Dataset id (e.g. "natal", "transit").
pub const DATA_RB_DATASET: &str = "data-rb-dataset";

/// House set id (e.g. "natal").
pub const DATA_RB_HOUSE_SET: &str = "data-rb-house-set";

/// House number (1..12).
pub const DATA_RB_HOUSE: &str = "data-rb-house";

/// Structure kinds as emitted by the SVG spec renderer.
///
/// These are string values stored under [`DATA_RB_STRUCTURE`].
///
/// Note: keep these stable; downstream tooling may depend on them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureKind {
    SignDivider,
    HouseSpoke,
    HouseAxis,
    BandBoundary,
    LaneSeparator,
}

impl StructureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            StructureKind::SignDivider => "sign-divider",
            StructureKind::HouseSpoke => "house-spoke",
            StructureKind::HouseAxis => "house-axis",
            StructureKind::BandBoundary => "band-boundary",
            StructureKind::LaneSeparator => "lane-separator",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "sign-divider" => StructureKind::SignDivider,
            "house-spoke" => StructureKind::HouseSpoke,
            "house-axis" => StructureKind::HouseAxis,
            "band-boundary" => StructureKind::BandBoundary,
            "lane-separator" => StructureKind::LaneSeparator,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structure_kind_round_trip() {
        for k in [
            StructureKind::SignDivider,
            StructureKind::HouseSpoke,
            StructureKind::HouseAxis,
            StructureKind::BandBoundary,
            StructureKind::LaneSeparator,
        ] {
            let s = k.as_str();
            assert_eq!(StructureKind::parse(s), Some(k));
        }
    }
}
