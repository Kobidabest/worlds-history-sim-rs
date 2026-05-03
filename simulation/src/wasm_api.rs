use wasm_bindgen::prelude::*;
use crate::planet::PlanetPreset;
use crate::simulation::{SimConfig, Simulation};

/// WASM-exposed simulation handle.
#[wasm_bindgen]
pub struct WasmSimulation {
    sim: Simulation,
}

#[wasm_bindgen]
impl WasmSimulation {
    /// Create a new simulation with the given seed.
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> WasmSimulation {
        WasmSimulation {
            sim: Simulation::new(seed, SimConfig::default()),
        }
    }

    /// Create a simulation with custom dimensions.
    pub fn new_with_size(seed: u32, width: u32, height: u32) -> WasmSimulation {
        let config = SimConfig {
            world_width: width,
            world_height: height,
            ..SimConfig::default()
        };
        WasmSimulation {
            sim: Simulation::new(seed, config),
        }
    }

    /// Create a simulation on a specific alien planet preset.
    ///
    /// `preset_index` corresponds to [`PlanetPreset::ALL`].
    pub fn new_on_planet(
        seed: u32,
        width: u32,
        height: u32,
        preset_index: u32,
    ) -> WasmSimulation {
        let config = SimConfig {
            world_width: width,
            world_height: height,
            planet_preset: PlanetPreset::from_index(preset_index),
            ..SimConfig::default()
        };
        WasmSimulation {
            sim: Simulation::new(seed, config),
        }
    }

    /// Get the JSON-encoded list of available planet presets.
    pub fn planet_presets() -> String {
        let arr: Vec<serde_json::Value> = PlanetPreset::ALL
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let cfg = p.config();
                serde_json::json!({
                    "index": i,
                    "id": format!("{:?}", p),
                    "name": p.name(),
                    "description": cfg.name,
                    "star": cfg.star_class.name(),
                    "atmosphere": cfg.atmosphere.name(),
                    "solvent": cfg.solvent.name(),
                    "gravity": cfg.gravity,
                    "mean_temperature": cfg.mean_temperature,
                    "radiation": cfg.effective_radiation(),
                    "toxicity": cfg.toxic_load(),
                    "ocean_fraction": cfg.ocean_fraction,
                })
            })
            .collect();
        serde_json::to_string(&arr).unwrap_or_else(|_| "[]".to_string())
    }

    /// Advance the simulation by N ticks.
    pub fn tick(&mut self, steps: u32) {
        for _ in 0..steps {
            self.sim.tick();
        }
    }

    /// Get the current tick number.
    pub fn get_tick(&self) -> u64 {
        self.sim.tick
    }

    /// Get the world width.
    pub fn get_width(&self) -> u32 {
        self.sim.config.world_width
    }

    /// Get the world height.
    pub fn get_height(&self) -> u32 {
        self.sim.config.world_height
    }

    /// Get terrain data as RGBA pixel buffer.
    pub fn get_terrain_rgba(&self) -> Vec<u8> {
        self.sim.get_terrain_rgba()
    }

    /// Get creature positions and colors as flat float buffer.
    /// Layout: [x, y, r, g, b, size, diet, energy] per creature (8 floats each).
    pub fn get_creature_data(&self) -> Vec<f32> {
        self.sim.get_creature_data()
    }

    /// Get simulation statistics as JSON string.
    pub fn get_stats(&self) -> String {
        self.sim.get_stats_json()
    }

    /// Get population history as JSON string.
    pub fn get_history(&self) -> String {
        self.sim.get_history_json()
    }

    /// Get info about a specific tile as JSON string.
    pub fn get_tile_info(&self, x: u32, y: u32) -> String {
        self.sim.get_tile_info_json(x, y)
    }

    /// Get the total number of living creatures.
    pub fn get_population(&self) -> u32 {
        self.sim.creatures.iter().filter(|c| c.alive).count() as u32
    }

    /// Get the number of living species.
    pub fn get_species_count(&self) -> u32 {
        self.sim.species_registry.living_species().len() as u32
    }
}
