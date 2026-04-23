//! Alien planet definitions.
//!
//! A planet is the environmental substrate on which evolution happens.
//! Its star, gravity, atmosphere, solvent, and radiation profile determine
//! which biomes can appear and which selection pressures dominate — so two
//! runs on the same species pool produce very different life on different
//! worlds.

use serde::{Deserialize, Serialize};

/// Spectral class of the host star. Drives photosynthesis efficiency,
/// UV/ionizing radiation at the surface, and the base light budget for
/// temperature generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarClass {
    /// M-class red dwarf: dim, red-heavy spectrum, frequent flares.
    RedDwarf,
    /// K-class orange dwarf: stable, slightly redder than the Sun.
    OrangeDwarf,
    /// G-class sun-like star.
    Sunlike,
    /// F-class yellow-white, hotter and more UV-rich than the Sun.
    YellowWhite,
    /// A/B-class hot blue star: short-lived, heavy UV flux.
    BlueGiant,
    /// Binary system: higher radiation, variable energy.
    Binary,
}

impl StarClass {
    pub fn name(&self) -> &'static str {
        match self {
            StarClass::RedDwarf => "M-class Red Dwarf",
            StarClass::OrangeDwarf => "K-class Orange Dwarf",
            StarClass::Sunlike => "G-class Sun-like",
            StarClass::YellowWhite => "F-class Yellow-White",
            StarClass::BlueGiant => "B-class Blue Giant",
            StarClass::Binary => "Binary System",
        }
    }

    /// Base ionizing / UV flux reaching an unshielded surface (0..1).
    pub fn base_radiation(&self) -> f32 {
        match self {
            StarClass::RedDwarf => 0.55, // flare-driven spikes
            StarClass::OrangeDwarf => 0.15,
            StarClass::Sunlike => 0.25,
            StarClass::YellowWhite => 0.45,
            StarClass::BlueGiant => 0.85,
            StarClass::Binary => 0.55,
        }
    }

    /// Efficiency multiplier for photosynthesis on the surface.
    pub fn photosynthesis_efficiency(&self) -> f32 {
        match self {
            StarClass::RedDwarf => 0.35,
            StarClass::OrangeDwarf => 0.75,
            StarClass::Sunlike => 1.0,
            StarClass::YellowWhite => 1.15,
            StarClass::BlueGiant => 1.3,
            StarClass::Binary => 1.1,
        }
    }

    /// Dominant spectral hint used when tinting autotrophs.
    pub fn light_tint(&self) -> [u8; 3] {
        match self {
            StarClass::RedDwarf => [210, 90, 60],
            StarClass::OrangeDwarf => [230, 160, 80],
            StarClass::Sunlike => [255, 240, 210],
            StarClass::YellowWhite => [240, 240, 255],
            StarClass::BlueGiant => [180, 200, 255],
            StarClass::Binary => [255, 220, 200],
        }
    }
}

/// Dominant atmospheric composition. Determines breathing pressure and
/// whether free oxygen or toxic trace gases are present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtmosphereType {
    /// Oxygen-nitrogen mix, Earthlike.
    OxygenNitrogen,
    /// Mostly CO2, runaway greenhouse.
    ThickCO2,
    /// Methane / hydrogen, reducing chemistry.
    Methane,
    /// Ammonia vapour + nitrogen, cold reducing.
    Ammonia,
    /// Sulfuric acid clouds, volcanic.
    SulfuricAcid,
    /// No appreciable atmosphere (moon / airless world).
    Trace,
}

impl AtmosphereType {
    pub fn name(&self) -> &'static str {
        match self {
            AtmosphereType::OxygenNitrogen => "O2 / N2",
            AtmosphereType::ThickCO2 => "Thick CO2",
            AtmosphereType::Methane => "Methane",
            AtmosphereType::Ammonia => "Ammonia",
            AtmosphereType::SulfuricAcid => "Sulfuric",
            AtmosphereType::Trace => "Trace",
        }
    }

    /// 0..1. How toxic unshielded respiration is for a "generic" organism.
    pub fn toxicity(&self) -> f32 {
        match self {
            AtmosphereType::OxygenNitrogen => 0.0,
            AtmosphereType::ThickCO2 => 0.35,
            AtmosphereType::Methane => 0.5,
            AtmosphereType::Ammonia => 0.55,
            AtmosphereType::SulfuricAcid => 0.9,
            AtmosphereType::Trace => 0.7, // asphyxiation, not toxicity per se
        }
    }

    /// Greenhouse multiplier applied to mean temperature.
    pub fn greenhouse(&self) -> f32 {
        match self {
            AtmosphereType::OxygenNitrogen => 1.0,
            AtmosphereType::ThickCO2 => 1.8,
            AtmosphereType::Methane => 1.2,
            AtmosphereType::Ammonia => 0.7,
            AtmosphereType::SulfuricAcid => 2.2,
            AtmosphereType::Trace => 0.4,
        }
    }

    /// Atmospheric shielding against stellar radiation (0..1 reduction).
    pub fn radiation_shield(&self) -> f32 {
        match self {
            AtmosphereType::OxygenNitrogen => 0.75,
            AtmosphereType::ThickCO2 => 0.85,
            AtmosphereType::Methane => 0.55,
            AtmosphereType::Ammonia => 0.55,
            AtmosphereType::SulfuricAcid => 0.9,
            AtmosphereType::Trace => 0.05,
        }
    }
}

/// Dominant liquid on the surface. Water is special-cased for Earthlike.
/// Non-aqueous solvents re-skin the "ocean" biome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Solvent {
    Water,
    Ammonia,
    Methane,
    Sulfuric,
    /// No standing liquid (desert / iceball sublimating directly).
    None,
}

impl Solvent {
    pub fn name(&self) -> &'static str {
        match self {
            Solvent::Water => "Liquid water",
            Solvent::Ammonia => "Liquid ammonia",
            Solvent::Methane => "Liquid methane",
            Solvent::Sulfuric => "Sulfuric acid",
            Solvent::None => "Dry / frozen",
        }
    }

    /// Ocean tint for the rendered map.
    pub fn ocean_color(&self) -> [u8; 3] {
        match self {
            Solvent::Water => [28, 66, 84],
            Solvent::Ammonia => [82, 110, 122],
            Solvent::Methane => [60, 42, 30],
            Solvent::Sulfuric => [168, 142, 90],
            Solvent::None => [90, 72, 56],
        }
    }
}

/// A named preset of alien worlds suitable for UI selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanetPreset {
    Earthlike,
    DesertWorld,
    OceanWorld,
    IceWorld,
    SuperEarth,
    LowGravity,
    VenusianHell,
    TitanAnalogue,
    BlueGiantHothouse,
    TidallyLockedRedDwarf,
}

impl PlanetPreset {
    pub const ALL: &'static [PlanetPreset] = &[
        PlanetPreset::Earthlike,
        PlanetPreset::DesertWorld,
        PlanetPreset::OceanWorld,
        PlanetPreset::IceWorld,
        PlanetPreset::SuperEarth,
        PlanetPreset::LowGravity,
        PlanetPreset::VenusianHell,
        PlanetPreset::TitanAnalogue,
        PlanetPreset::BlueGiantHothouse,
        PlanetPreset::TidallyLockedRedDwarf,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            PlanetPreset::Earthlike => "Earthlike",
            PlanetPreset::DesertWorld => "Desert World",
            PlanetPreset::OceanWorld => "Ocean World",
            PlanetPreset::IceWorld => "Ice World",
            PlanetPreset::SuperEarth => "Super-Earth",
            PlanetPreset::LowGravity => "Low-Gravity Moon",
            PlanetPreset::VenusianHell => "Venusian Hell",
            PlanetPreset::TitanAnalogue => "Titan Analogue",
            PlanetPreset::BlueGiantHothouse => "Blue-Giant Hothouse",
            PlanetPreset::TidallyLockedRedDwarf => "Red Dwarf Tide-Lock",
        }
    }

    pub fn from_index(i: u32) -> Self {
        Self::ALL
            .get(i as usize)
            .copied()
            .unwrap_or(PlanetPreset::Earthlike)
    }

    pub fn config(&self) -> PlanetConfig {
        match self {
            PlanetPreset::Earthlike => PlanetConfig {
                name: "Earthlike".to_string(),
                preset: *self,
                star_class: StarClass::Sunlike,
                atmosphere: AtmosphereType::OxygenNitrogen,
                solvent: Solvent::Water,
                gravity: 1.0,
                surface_pressure: 1.0,
                base_radiation: 0.15,
                mean_temperature: -2.5,
                temperature_span: 65.0,
                ocean_fraction: 0.55,
                mutation_multiplier: 1.0,
                day_length: 1.0,
            },
            PlanetPreset::DesertWorld => PlanetConfig {
                name: "Arrakis-style desert".to_string(),
                preset: *self,
                star_class: StarClass::OrangeDwarf,
                atmosphere: AtmosphereType::OxygenNitrogen,
                solvent: Solvent::None,
                gravity: 0.9,
                surface_pressure: 0.7,
                base_radiation: 0.4,
                mean_temperature: 15.0,
                temperature_span: 80.0,
                ocean_fraction: 0.05,
                mutation_multiplier: 1.1,
                day_length: 1.4,
            },
            PlanetPreset::OceanWorld => PlanetConfig {
                name: "Pelagic ocean world".to_string(),
                preset: *self,
                star_class: StarClass::Sunlike,
                atmosphere: AtmosphereType::OxygenNitrogen,
                solvent: Solvent::Water,
                gravity: 1.15,
                surface_pressure: 1.4,
                base_radiation: 0.1,
                mean_temperature: 6.0,
                temperature_span: 35.0,
                ocean_fraction: 0.92,
                mutation_multiplier: 1.0,
                day_length: 0.9,
            },
            PlanetPreset::IceWorld => PlanetConfig {
                name: "Glaciated iceball".to_string(),
                preset: *self,
                star_class: StarClass::OrangeDwarf,
                atmosphere: AtmosphereType::OxygenNitrogen,
                solvent: Solvent::Water,
                gravity: 0.85,
                surface_pressure: 0.8,
                base_radiation: 0.25,
                mean_temperature: -30.0,
                temperature_span: 40.0,
                ocean_fraction: 0.3,
                mutation_multiplier: 0.9,
                day_length: 1.1,
            },
            PlanetPreset::SuperEarth => PlanetConfig {
                name: "High-gravity super-Earth".to_string(),
                preset: *self,
                star_class: StarClass::Sunlike,
                atmosphere: AtmosphereType::ThickCO2,
                solvent: Solvent::Water,
                gravity: 2.2,
                surface_pressure: 3.5,
                base_radiation: 0.1,
                mean_temperature: 10.0,
                temperature_span: 50.0,
                ocean_fraction: 0.6,
                mutation_multiplier: 1.0,
                day_length: 0.7,
            },
            PlanetPreset::LowGravity => PlanetConfig {
                name: "Low-gravity moon".to_string(),
                preset: *self,
                star_class: StarClass::Sunlike,
                atmosphere: AtmosphereType::OxygenNitrogen,
                solvent: Solvent::Water,
                gravity: 0.38,
                surface_pressure: 0.5,
                base_radiation: 0.5,
                mean_temperature: -5.0,
                temperature_span: 70.0,
                ocean_fraction: 0.35,
                mutation_multiplier: 1.2,
                day_length: 1.8,
            },
            PlanetPreset::VenusianHell => PlanetConfig {
                name: "Venusian hothouse".to_string(),
                preset: *self,
                star_class: StarClass::Sunlike,
                atmosphere: AtmosphereType::SulfuricAcid,
                solvent: Solvent::Sulfuric,
                gravity: 0.91,
                surface_pressure: 9.0,
                base_radiation: 0.1,
                mean_temperature: 60.0,
                temperature_span: 25.0,
                ocean_fraction: 0.2,
                mutation_multiplier: 1.3,
                day_length: 4.0,
            },
            PlanetPreset::TitanAnalogue => PlanetConfig {
                name: "Titan analogue".to_string(),
                preset: *self,
                star_class: StarClass::OrangeDwarf,
                atmosphere: AtmosphereType::Methane,
                solvent: Solvent::Methane,
                gravity: 0.14,
                surface_pressure: 1.45,
                base_radiation: 0.25,
                mean_temperature: -90.0,
                temperature_span: 15.0,
                ocean_fraction: 0.3,
                mutation_multiplier: 0.85,
                day_length: 2.1,
            },
            PlanetPreset::BlueGiantHothouse => PlanetConfig {
                name: "Blue-giant hothouse".to_string(),
                preset: *self,
                star_class: StarClass::BlueGiant,
                atmosphere: AtmosphereType::ThickCO2,
                solvent: Solvent::Water,
                gravity: 1.3,
                surface_pressure: 2.1,
                base_radiation: 0.9,
                mean_temperature: 35.0,
                temperature_span: 55.0,
                ocean_fraction: 0.45,
                mutation_multiplier: 1.6,
                day_length: 0.6,
            },
            PlanetPreset::TidallyLockedRedDwarf => PlanetConfig {
                name: "Tide-locked twilight world".to_string(),
                preset: *self,
                star_class: StarClass::RedDwarf,
                atmosphere: AtmosphereType::OxygenNitrogen,
                solvent: Solvent::Water,
                gravity: 0.8,
                surface_pressure: 0.9,
                base_radiation: 0.7,
                mean_temperature: -10.0,
                temperature_span: 120.0, // big day-night gradient
                ocean_fraction: 0.35,
                mutation_multiplier: 1.4,
                day_length: 20.0,
            },
        }
    }
}

/// Full planetary environment parameters used by the world generator
/// and by per-tick selection pressures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetConfig {
    pub name: String,
    pub preset: PlanetPreset,
    pub star_class: StarClass,
    pub atmosphere: AtmosphereType,
    pub solvent: Solvent,
    /// Surface gravity in Earth-g.
    pub gravity: f32,
    /// Surface pressure in Earth atmospheres.
    pub surface_pressure: f32,
    /// Fraction of stellar radiation reaching the surface before shielding.
    pub base_radiation: f32,
    /// Mean global temperature in °C.
    pub mean_temperature: f32,
    /// Equator-to-pole temperature swing in °C.
    pub temperature_span: f32,
    /// Target fraction of tiles covered by ocean/liquid (0..1).
    pub ocean_fraction: f32,
    /// Per-gene mutation rate multiplier applied during reproduction.
    pub mutation_multiplier: f32,
    /// Day length in Earth days (visual only, but affects thermal swing).
    pub day_length: f32,
}

impl PlanetConfig {
    pub fn earthlike() -> Self {
        PlanetPreset::Earthlike.config()
    }

    /// Effective surface radiation after atmospheric shielding.
    pub fn effective_radiation(&self) -> f32 {
        let stellar = (self.star_class.base_radiation() + self.base_radiation) * 0.5;
        (stellar * (1.0 - self.atmosphere.radiation_shield())).clamp(0.0, 1.0)
    }

    /// How corrosive/toxic unshielded respiration is (0..1).
    pub fn toxic_load(&self) -> f32 {
        self.atmosphere.toxicity()
    }

    /// Temperature of the coldest polar tile (rough).
    pub fn min_temperature(&self) -> f32 {
        self.mean_temperature - self.temperature_span * 0.5
    }

    /// Temperature of the warmest equatorial tile (rough).
    pub fn max_temperature(&self) -> f32 {
        self.mean_temperature + self.temperature_span * 0.5
    }

    /// Altitude span scales with gravity: stronger gravity = shallower relief.
    pub fn altitude_span(&self) -> (f32, f32) {
        let base = 15000.0 / self.gravity.max(0.3);
        (-base, base)
    }
}

impl Default for PlanetConfig {
    fn default() -> Self {
        Self::earthlike()
    }
}
