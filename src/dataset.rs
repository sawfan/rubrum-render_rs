use serde::{Deserialize, Serialize};

/// A named dataset of placements.
///
/// Strict responsibility:
/// - Placements only
/// - No styling
/// - No geometry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DatasetData {
    pub id: String,

    #[serde(default)]
    pub bodies: Vec<rubrum::PlacementMotion>,
}

/// A named set of house cusps.
///
/// Strict responsibility:
/// - Cusps only
/// - No styling
/// - No geometry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HouseSetData {
    pub id: String,

    #[serde(default)]
    pub house_cusps: Vec<crate::chart_data::HouseCuspData>,
}
