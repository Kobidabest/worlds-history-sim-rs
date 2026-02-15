use wasm_bindgen::prelude::*;
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
