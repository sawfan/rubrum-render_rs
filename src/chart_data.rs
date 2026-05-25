use serde::{Deserialize, Serialize};

use crate::dataset::{DatasetData, HouseSetData};

/// Chart data for rendering.
///
/// Strict responsibility:
/// - Longitudes
/// - Retrograde flags (when applicable)
/// - Cusps
/// - No styling
/// - No geometry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChartData {
    /// Dataset: natal bodies.
    ///
    /// Kept for backwards compatibility with earlier TOML examples.
    ///
    /// Prefer `datasets` for new specs.
    #[serde(default)]
    pub natal_bodies: Vec<rubrum::PlacementMotion>,

    /// All datasets available for dataset-bound lanes.
    ///
    /// This carries placements only (no styling / geometry).
    #[serde(default)]
    pub datasets: Vec<DatasetData>,

    /// All house cusp sets available for house divisions.
    ///
    /// This carries cusps only (no styling / geometry).
    #[serde(default)]
    pub house_sets: Vec<HouseSetData>,

    /// House cusps expressed as absolute zodiac longitudes.
    ///
    /// Kept for backwards compatibility with earlier TOML examples.
    ///
    /// Prefer `house_sets` for new specs.
    ///
    /// Motion does not apply to cusps.
    #[serde(default)]
    pub house_cusps: Vec<HouseCuspData>,
}

impl ChartData {
    /// Adapter into the legacy `rubrum::Chart` used by the current Cairo renderer.
    pub fn to_legacy_chart(&self) -> rubrum::Chart {
        // The Cairo renderers still adapt `ChartData` into a legacy `rubrum::Chart` in a few
        // places (notably to reuse Ascendant-based rotation logic). When specs provide data via
        // `datasets` (lane bindings), we need the "natal" dataset to be visible here.
        let bodies = self
            .dataset_bodies("natal")
            .unwrap_or(self.natal_bodies.as_slice());

        let placements = bodies
            .iter()
            .copied()
            .map(|pm| pm.placement)
            .collect::<Vec<_>>();

        let house_cusps_src = self
            .house_set_cusps("natal")
            .unwrap_or(self.house_cusps.as_slice());

        let house_cusps = house_cusps_src
            .iter()
            .copied()
            .map(|c| rubrum::HouseSignDegree::new(c.house, c.sign_degree))
            .collect::<Vec<_>>();

        rubrum::Chart {
            info: None,
            placements,
            house_cusps,
        }
    }

    /// Resolve a dataset-bound lane to the bodies it should render.
    ///
    /// Backwards compatibility:
    /// - If `dataset_id == "natal"` and no explicit dataset exists, fall back to `natal_bodies`.
    pub fn dataset_bodies(&self, dataset_id: &str) -> Option<&[rubrum::PlacementMotion]> {
        if let Some(ds) = self.datasets.iter().find(|ds| ds.id == dataset_id) {
            return Some(ds.bodies.as_slice());
        }

        if dataset_id == "natal" {
            return Some(self.natal_bodies.as_slice());
        }

        None
    }

    /// Resolve a house set id to the cusps it should use.
    ///
    /// Backwards compatibility:
    /// - If `house_set_id == "natal"` and no explicit house set exists, fall back to `house_cusps`.
    pub fn house_set_cusps(&self, house_set_id: &str) -> Option<&[HouseCuspData]> {
        if let Some(set) = self.house_sets.iter().find(|set| set.id == house_set_id) {
            return Some(set.house_cusps.as_slice());
        }

        if house_set_id == "natal" {
            return Some(self.house_cusps.as_slice());
        }

        None
    }
}

impl From<&rubrum::Chart> for ChartData {
    fn from(chart: &rubrum::Chart) -> Self {
        let natal_bodies = chart
            .placements
            .iter()
            .copied()
            .map(rubrum::PlacementMotion::from)
            .collect::<Vec<_>>();

        let house_cusps = chart
            .house_cusps
            .iter()
            .copied()
            .map(|hsd| HouseCuspData {
                house: hsd.house,
                sign_degree: hsd.sign_degree,
            })
            .collect::<Vec<_>>();

        Self {
            natal_bodies,
            datasets: Vec::new(),
            house_sets: Vec::new(),
            house_cusps,
        }
    }
}

/// Cusp data expressed without styling or geometry.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HouseCuspData {
    pub house: rubrum::House,
    pub sign_degree: rubrum::SignDegree,
}
