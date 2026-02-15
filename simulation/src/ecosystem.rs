use serde::{Deserialize, Serialize};

/// Per-tile ecosystem state tracking plant biomass and creature counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileEcosystem {
    /// Current plant biomass available for herbivores.
    pub plant_biomass: f32,
    /// Maximum plant biomass this tile can support.
    pub max_biomass: f32,
    /// Rate at which plants regrow per tick.
    pub growth_rate: f32,
    /// Number of creatures currently on this tile.
    pub creature_count: u32,
    /// Number of herbivores on this tile.
    pub herbivore_count: u32,
    /// Number of carnivores on this tile.
    pub carnivore_count: u32,
}

impl TileEcosystem {
    pub fn new(max_biomass: f32, growth_rate: f32) -> Self {
        Self {
            plant_biomass: max_biomass * 0.5, // Start at half capacity
            max_biomass,
            growth_rate,
            creature_count: 0,
            herbivore_count: 0,
            carnivore_count: 0,
        }
    }

    /// Regrow plants towards carrying capacity (logistic growth).
    pub fn tick_plant_growth(&mut self) {
        if self.max_biomass <= 0.0 {
            return;
        }

        // Logistic growth: grows faster when biomass is low, slows near capacity
        let growth_factor = 1.0 - (self.plant_biomass / self.max_biomass);
        let growth = self.growth_rate * growth_factor * self.max_biomass * 0.02;
        self.plant_biomass = (self.plant_biomass + growth).min(self.max_biomass);
    }

    /// Consume some plant biomass. Returns the amount actually consumed.
    pub fn consume_plants(&mut self, amount: f32) -> f32 {
        let consumed = amount.min(self.plant_biomass);
        self.plant_biomass -= consumed;
        consumed
    }

    /// Reset creature counts for a new tick.
    pub fn reset_counts(&mut self) {
        self.creature_count = 0;
        self.herbivore_count = 0;
        self.carnivore_count = 0;
    }
}

/// Carrying capacity for creatures on a tile based on biome productivity.
pub fn tile_carrying_capacity(max_biomass: f32) -> u32 {
    // Roughly 1 creature per 5 units of max biomass
    (max_biomass / 5.0).max(0.0) as u32
}
