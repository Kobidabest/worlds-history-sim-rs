use crate::planet::{PlanetConfig, Solvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiomeType {
    // Earthlike
    IceCap,
    Ocean,
    Grassland,
    Forest,
    Taiga,
    Tundra,
    Desert,
    Rainforest,
    // Alien additions
    AmmoniaOcean,
    MethaneLake,
    SulfuricSea,
    LavaField,
    CryoPlain,
    CrystalForest,
    FungalMat,
    GeothermalVent,
    SilicaDesert,
    HazeHighland,
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
        BiomeType::AmmoniaOcean,
        BiomeType::MethaneLake,
        BiomeType::SulfuricSea,
        BiomeType::LavaField,
        BiomeType::CryoPlain,
        BiomeType::CrystalForest,
        BiomeType::FungalMat,
        BiomeType::GeothermalVent,
        BiomeType::SilicaDesert,
        BiomeType::HazeHighland,
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
            BiomeType::AmmoniaOcean => "Ammonia Ocean",
            BiomeType::MethaneLake => "Methane Lake",
            BiomeType::SulfuricSea => "Sulfuric Sea",
            BiomeType::LavaField => "Lava Field",
            BiomeType::CryoPlain => "Cryogenic Plain",
            BiomeType::CrystalForest => "Crystal Forest",
            BiomeType::FungalMat => "Fungal Mat",
            BiomeType::GeothermalVent => "Geothermal Vent",
            BiomeType::SilicaDesert => "Silica Desert",
            BiomeType::HazeHighland => "Haze Highland",
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
            BiomeType::AmmoniaOcean => [82, 110, 122],
            BiomeType::MethaneLake => [60, 42, 30],
            BiomeType::SulfuricSea => [168, 142, 90],
            BiomeType::LavaField => [180, 60, 30],
            BiomeType::CryoPlain => [180, 200, 220],
            BiomeType::CrystalForest => [168, 130, 210],
            BiomeType::FungalMat => [120, 70, 140],
            BiomeType::GeothermalVent => [110, 60, 40],
            BiomeType::SilicaDesert => [220, 200, 230],
            BiomeType::HazeHighland => [180, 160, 120],
        }
    }

    /// Plant/autotroph productivity multiplier for this biome (0.0 - 1.0+).
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
            BiomeType::AmmoniaOcean => 0.0,
            BiomeType::MethaneLake => 0.0,
            BiomeType::SulfuricSea => 0.0,
            BiomeType::LavaField => 0.0,
            BiomeType::CryoPlain => 0.04,
            BiomeType::CrystalForest => 0.5,
            BiomeType::FungalMat => 0.7,
            BiomeType::GeothermalVent => 0.4, // chemosynthesis
            BiomeType::SilicaDesert => 0.02,
            BiomeType::HazeHighland => 0.15,
        }
    }

    /// Maximum standing biomass this tile can support.
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
            BiomeType::AmmoniaOcean => 0.0,
            BiomeType::MethaneLake => 0.0,
            BiomeType::SulfuricSea => 0.0,
            BiomeType::LavaField => 0.0,
            BiomeType::CryoPlain => 6.0,
            BiomeType::CrystalForest => 55.0,
            BiomeType::FungalMat => 80.0,
            BiomeType::GeothermalVent => 45.0,
            BiomeType::SilicaDesert => 3.0,
            BiomeType::HazeHighland => 20.0,
        }
    }

    pub fn is_liquid(&self) -> bool {
        matches!(
            self,
            BiomeType::Ocean
                | BiomeType::AmmoniaOcean
                | BiomeType::MethaneLake
                | BiomeType::SulfuricSea
        )
    }

    pub fn is_frozen(&self) -> bool {
        matches!(self, BiomeType::IceCap | BiomeType::CryoPlain)
    }

    /// Whether this biome can host land creatures.
    pub fn is_habitable(&self) -> bool {
        !self.is_liquid()
            && !matches!(self, BiomeType::IceCap | BiomeType::LavaField)
    }

    /// Background local radiation contribution (0..1). Vents and lava fields
    /// are hot; crystal forests carry piezoelectric discharge.
    pub fn local_radiation(&self) -> f32 {
        match self {
            BiomeType::GeothermalVent => 0.2,
            BiomeType::LavaField => 0.3,
            BiomeType::CrystalForest => 0.15,
            _ => 0.0,
        }
    }
}

/// Tunable climate envelope for biome placement. Stats are expressed
/// relative to the planet's temperature and rainfall ranges so they
/// re-map automatically to alien worlds.
pub struct BiomeStats {
    pub min_altitude: f32,
    pub max_altitude: f32,
    pub min_rainfall: f32,
    pub max_rainfall: f32,
    pub min_temperature: f32,
    pub max_temperature: f32,
    pub requires: BiomeRequirement,
}

/// Which worlds this biome can appear on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BiomeRequirement {
    /// Always eligible.
    Any,
    /// Only if the planetary solvent matches this.
    Solvent(Solvent),
    /// Only on planets whose atmosphere is reducing (methane/ammonia).
    Reducing,
    /// Only on sulfuric acid worlds.
    Sulfuric,
    /// Only on very hot worlds (mean_temperature >= threshold).
    HotWorld(f32),
    /// Only on very cold worlds (mean_temperature <= threshold).
    ColdWorld(f32),
    /// Requires free oxygen to exist.
    Oxygen,
}

impl BiomeRequirement {
    pub fn matches(&self, planet: &PlanetConfig) -> bool {
        use crate::planet::AtmosphereType;
        match self {
            BiomeRequirement::Any => true,
            BiomeRequirement::Solvent(s) => planet.solvent == *s,
            BiomeRequirement::Reducing => matches!(
                planet.atmosphere,
                AtmosphereType::Methane | AtmosphereType::Ammonia
            ),
            BiomeRequirement::Sulfuric => {
                planet.atmosphere == AtmosphereType::SulfuricAcid
                    || planet.solvent == Solvent::Sulfuric
            }
            BiomeRequirement::HotWorld(t) => planet.mean_temperature >= *t,
            BiomeRequirement::ColdWorld(t) => planet.mean_temperature <= *t,
            BiomeRequirement::Oxygen => planet.atmosphere == AtmosphereType::OxygenNitrogen,
        }
    }
}

/// Build climate envelopes relative to the active planet. Temperatures
/// are absolute °C; rainfall is scaled to 0..13000 as on Earth. The
/// world generator rescales rainfall automatically for the planet.
pub fn biome_stats_for(biome_type: BiomeType, planet: &PlanetConfig) -> BiomeStats {
    let (min_alt, max_alt) = planet.altitude_span();
    let (min_temp, max_temp) = (planet.min_temperature(), planet.max_temperature());
    let span = (max_temp - min_temp).max(1.0);
    // Helper to map a 0..1 temperature fraction into the planet's actual range.
    let t = |lo: f32, hi: f32| (min_temp + span * lo, min_temp + span * hi);

    match biome_type {
        BiomeType::IceCap => {
            let (_, hi) = t(0.0, 0.2);
            BiomeStats {
                min_altitude: min_alt,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 13000.0,
                min_temperature: min_temp,
                max_temperature: hi.min(-5.0),
                requires: BiomeRequirement::Any,
            }
        }
        BiomeType::Ocean => BiomeStats {
            min_altitude: min_alt,
            max_altitude: 0.0,
            min_rainfall: 0.0,
            max_rainfall: 13000.0,
            min_temperature: -15.0_f32.max(min_temp),
            max_temperature: max_temp,
            requires: BiomeRequirement::Solvent(Solvent::Water),
        },
        BiomeType::Grassland => {
            let (lo, hi) = t(0.45, 1.0);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 15.0,
                max_rainfall: 1575.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Oxygen,
            }
        }
        BiomeType::Forest => {
            let (lo, hi) = t(0.45, 1.0);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 1375.0,
                max_rainfall: 2975.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Oxygen,
            }
        }
        BiomeType::Taiga => {
            let (lo, hi) = t(0.25, 0.55);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 475.0,
                max_rainfall: 13000.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Oxygen,
            }
        }
        BiomeType::Tundra => {
            let (lo, hi) = t(0.15, 0.45);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 725.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Oxygen,
            }
        }
        BiomeType::Desert => {
            let (lo, hi) = t(0.5, 1.0);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 275.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Any,
            }
        }
        BiomeType::Rainforest => {
            let (lo, hi) = t(0.55, 1.0);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 1775.0,
                max_rainfall: 13000.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Oxygen,
            }
        }
        BiomeType::AmmoniaOcean => BiomeStats {
            min_altitude: min_alt,
            max_altitude: 0.0,
            min_rainfall: 0.0,
            max_rainfall: 13000.0,
            min_temperature: min_temp,
            max_temperature: max_temp,
            requires: BiomeRequirement::Solvent(Solvent::Ammonia),
        },
        BiomeType::MethaneLake => BiomeStats {
            min_altitude: min_alt,
            max_altitude: 0.0,
            min_rainfall: 0.0,
            max_rainfall: 13000.0,
            min_temperature: min_temp,
            max_temperature: max_temp,
            requires: BiomeRequirement::Solvent(Solvent::Methane),
        },
        BiomeType::SulfuricSea => BiomeStats {
            min_altitude: min_alt,
            max_altitude: 0.0,
            min_rainfall: 0.0,
            max_rainfall: 13000.0,
            min_temperature: min_temp,
            max_temperature: max_temp,
            requires: BiomeRequirement::Solvent(Solvent::Sulfuric),
        },
        BiomeType::LavaField => {
            let (lo, hi) = t(0.85, 1.0);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 13000.0,
                min_temperature: lo.max(45.0),
                max_temperature: hi,
                requires: BiomeRequirement::HotWorld(35.0),
            }
        }
        BiomeType::CryoPlain => {
            let (_, hi) = t(0.0, 0.35);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 4000.0,
                min_temperature: min_temp,
                max_temperature: hi.min(-25.0),
                requires: BiomeRequirement::ColdWorld(-40.0),
            }
        }
        BiomeType::CrystalForest => {
            let (lo, hi) = t(0.3, 0.9);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 600.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Any,
            }
        }
        BiomeType::FungalMat => {
            let (lo, hi) = t(0.35, 0.8);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 1000.0,
                max_rainfall: 13000.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Reducing,
            }
        }
        BiomeType::GeothermalVent => {
            let (lo, hi) = t(0.4, 0.95);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 13000.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Any,
            }
        }
        BiomeType::SilicaDesert => {
            let (lo, hi) = t(0.35, 0.95);
            BiomeStats {
                min_altitude: 0.0,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 200.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Any,
            }
        }
        BiomeType::HazeHighland => {
            let (lo, hi) = t(0.3, 0.8);
            BiomeStats {
                min_altitude: 2000.0,
                max_altitude: max_alt,
                min_rainfall: 0.0,
                max_rainfall: 1500.0,
                min_temperature: lo,
                max_temperature: hi,
                requires: BiomeRequirement::Any,
            }
        }
    }
}
