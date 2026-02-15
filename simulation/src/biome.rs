use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiomeType {
    IceCap,
    Ocean,
    Grassland,
    Forest,
    Taiga,
    Tundra,
    Desert,
    Rainforest,
}

impl BiomeType {
    pub const ALL: &'static [BiomeType] = &[
        BiomeType::IceCap,
        BiomeType::Ocean,
        BiomeType::Grassland,
        BiomeType::Forest,
        BiomeType::Taiga,
        BiomeType::Tundra,
        BiomeType::Desert,
        BiomeType::Rainforest,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            BiomeType::IceCap => "Ice Cap",
            BiomeType::Ocean => "Ocean",
            BiomeType::Grassland => "Grassland",
            BiomeType::Forest => "Forest",
            BiomeType::Taiga => "Taiga",
            BiomeType::Tundra => "Tundra",
            BiomeType::Desert => "Desert",
            BiomeType::Rainforest => "Rainforest",
        }
    }

    pub fn color(&self) -> [u8; 3] {
        match self {
            BiomeType::IceCap => [255, 255, 255],
            BiomeType::Ocean => [28, 66, 84],
            BiomeType::Grassland => [167, 177, 84],
            BiomeType::Forest => [76, 132, 55],
            BiomeType::Taiga => [43, 63, 40],
            BiomeType::Tundra => [139, 139, 128],
            BiomeType::Desert => [253, 225, 171],
            BiomeType::Rainforest => [59, 103, 43],
        }
    }

    /// Plant growth rate multiplier for this biome (0.0 - 1.0).
    pub fn plant_growth_rate(&self) -> f32 {
        match self {
            BiomeType::IceCap => 0.0,
            BiomeType::Ocean => 0.0,
            BiomeType::Grassland => 0.6,
            BiomeType::Forest => 0.85,
            BiomeType::Taiga => 0.25,
            BiomeType::Tundra => 0.1,
            BiomeType::Desert => 0.05,
            BiomeType::Rainforest => 1.0,
        }
    }

    /// Maximum plant biomass capacity for this biome.
    pub fn max_plant_biomass(&self) -> f32 {
        match self {
            BiomeType::IceCap => 0.0,
            BiomeType::Ocean => 0.0,
            BiomeType::Grassland => 60.0,
            BiomeType::Forest => 100.0,
            BiomeType::Taiga => 30.0,
            BiomeType::Tundra => 15.0,
            BiomeType::Desert => 5.0,
            BiomeType::Rainforest => 120.0,
        }
    }

    /// Whether this biome can support land creatures.
    pub fn is_habitable(&self) -> bool {
        !matches!(self, BiomeType::IceCap | BiomeType::Ocean)
    }
}

pub struct BiomeStats {
    pub min_altitude: f32,
    pub max_altitude: f32,
    pub min_rainfall: f32,
    pub max_rainfall: f32,
    pub min_temperature: f32,
    pub max_temperature: f32,
}

// World constants inlined to avoid circular dependency with world module.
const MIN_ALTITUDE: f32 = -15000.0;
const MAX_ALTITUDE: f32 = 15000.0;
const MIN_RAINFALL: f32 = 0.0;
const MAX_RAINFALL: f32 = 13000.0;
const MIN_TEMPERATURE: f32 = -35.0;
const MAX_TEMPERATURE: f32 = 30.0;

impl From<BiomeType> for BiomeStats {
    fn from(biome_type: BiomeType) -> BiomeStats {
        match biome_type {
            BiomeType::IceCap => BiomeStats {
                min_altitude: MIN_ALTITUDE,
                max_altitude: MAX_ALTITUDE,
                min_rainfall: MIN_RAINFALL,
                max_rainfall: MAX_RAINFALL,
                min_temperature: MIN_TEMPERATURE,
                max_temperature: -15.0,
            },
            BiomeType::Ocean => BiomeStats {
                min_altitude: MIN_ALTITUDE,
                max_altitude: 0.0,
                min_rainfall: MIN_RAINFALL,
                max_rainfall: MAX_RAINFALL,
                min_temperature: -15.0,
                max_temperature: MAX_TEMPERATURE,
            },
            BiomeType::Grassland => BiomeStats {
                min_altitude: 0.0,
                max_altitude: MAX_ALTITUDE,
                min_rainfall: 15.0,
                max_rainfall: 1575.0,
                min_temperature: -5.0,
                max_temperature: MAX_TEMPERATURE,
            },
            BiomeType::Forest => BiomeStats {
                min_altitude: 0.0,
                max_altitude: MAX_ALTITUDE,
                min_rainfall: 1375.0,
                max_rainfall: 2975.0,
                min_temperature: -5.0,
                max_temperature: MAX_TEMPERATURE,
            },
            BiomeType::Taiga => BiomeStats {
                min_altitude: 0.0,
                max_altitude: MAX_ALTITUDE,
                min_rainfall: 475.0,
                max_rainfall: MAX_RAINFALL,
                min_temperature: -15.0,
                max_temperature: 0.0,
            },
            BiomeType::Tundra => BiomeStats {
                min_altitude: 0.0,
                max_altitude: MAX_ALTITUDE,
                min_rainfall: MIN_RAINFALL,
                max_rainfall: 725.0,
                min_temperature: -20.0,
                max_temperature: 0.0,
            },
            BiomeType::Desert => BiomeStats {
                min_altitude: 0.0,
                max_altitude: MAX_ALTITUDE,
                min_rainfall: MIN_RAINFALL,
                max_rainfall: 275.0,
                min_temperature: -5.0,
                max_temperature: MAX_TEMPERATURE,
            },
            BiomeType::Rainforest => BiomeStats {
                min_altitude: 0.0,
                max_altitude: MAX_ALTITUDE,
                min_rainfall: 1775.0,
                max_rainfall: MAX_RAINFALL,
                min_temperature: -5.0,
                max_temperature: MAX_TEMPERATURE,
            },
        }
    }
}
