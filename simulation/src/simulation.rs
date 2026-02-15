use crate::{
    creature::{Creature, CreatureId},
    ecosystem::TileEcosystem,
    genetics::Genome,
    species::SpeciesRegistry,
    world::World,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub world_width: u32,
    pub world_height: u32,
    pub initial_herbivore_species: u32,
    pub initial_carnivore_species: u32,
    pub creatures_per_species: u32,
    pub max_creatures: usize,
    pub speciation_threshold: f32,
    pub speciation_check_interval: u64,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            world_width: 200,
            world_height: 100,
            initial_herbivore_species: 5,
            initial_carnivore_species: 2,
            creatures_per_species: 40,
            max_creatures: 12000,
            speciation_threshold: 0.32,
            speciation_check_interval: 50,
        }
    }
}

/// Population history entry for graphing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationSnapshot {
    pub tick: u64,
    pub total: u32,
    pub herbivores: u32,
    pub carnivores: u32,
    pub species_count: u32,
}

/// The main simulation state.
#[derive(Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub config: SimConfig,
    pub world: World,
    pub creatures: Vec<Creature>,
    pub ecosystems: Vec<Vec<TileEcosystem>>,
    pub species_registry: SpeciesRegistry,
    pub tick: u64,
    pub next_creature_id: CreatureId,
    #[serde(skip, default = "default_rng")]
    pub rng: SmallRng,
    pub seed: u32,
    pub population_history: Vec<PopulationSnapshot>,
    /// Spatial index: tile (y, x) -> list of creature indices
    #[serde(skip)]
    spatial_index: HashMap<(usize, usize), Vec<usize>>,
}

fn default_rng() -> SmallRng {
    SmallRng::seed_from_u64(0)
}

impl Simulation {
    /// Create a new simulation with the given seed and config.
    pub fn new(seed: u32, config: SimConfig) -> Self {
        let rng = SmallRng::seed_from_u64(seed as u64 + 1000);
        let world = World::generate(config.world_width, config.world_height, seed);

        // Initialize ecosystems
        let mut ecosystems = Vec::with_capacity(config.world_height as usize);
        for y in 0..config.world_height as usize {
            let mut row = Vec::with_capacity(config.world_width as usize);
            for x in 0..config.world_width as usize {
                let biome = world.terrain[y][x].dominant_biome();
                let eco = TileEcosystem::new(biome.max_plant_biomass(), biome.plant_growth_rate());
                row.push(eco);
            }
            ecosystems.push(row);
        }

        let mut sim = Simulation {
            config,
            world,
            creatures: Vec::new(),
            ecosystems,
            species_registry: SpeciesRegistry::new(),
            tick: 0,
            next_creature_id: 1,
            rng,
            seed,
            population_history: Vec::new(),
            spatial_index: HashMap::new(),
        };

        sim.populate_initial_creatures();
        sim.rebuild_spatial_index();
        sim
    }

    fn populate_initial_creatures(&mut self) {
        let habitable = self.world.habitable_tiles();
        if habitable.is_empty() {
            return;
        }

        // Create herbivore species
        for _ in 0..self.config.initial_herbivore_species {
            let genome = Genome::random_herbivore(&mut self.rng);
            let species_id =
                self.species_registry
                    .create_species(None, genome.clone(), 0, &mut self.rng);

            for _ in 0..self.config.creatures_per_species {
                let idx = self.rng.gen_range(0..habitable.len());
                let (x, y) = habitable[idx];
                let g = Genome::random_herbivore(&mut self.rng);
                let creature =
                    Creature::new(self.next_creature_id, species_id, g, x, y, 0, &mut self.rng);
                self.next_creature_id += 1;
                self.species_registry.record_birth(species_id, 0);
                self.creatures.push(creature);
            }
        }

        // Create carnivore species
        for _ in 0..self.config.initial_carnivore_species {
            let genome = Genome::random_carnivore(&mut self.rng);
            let species_id =
                self.species_registry
                    .create_species(None, genome.clone(), 0, &mut self.rng);

            for _ in 0..self.config.creatures_per_species {
                let idx = self.rng.gen_range(0..habitable.len());
                let (x, y) = habitable[idx];
                let g = Genome::random_carnivore(&mut self.rng);
                let creature =
                    Creature::new(self.next_creature_id, species_id, g, x, y, 0, &mut self.rng);
                self.next_creature_id += 1;
                self.species_registry.record_birth(species_id, 0);
                self.creatures.push(creature);
            }
        }
    }

    fn rebuild_spatial_index(&mut self) {
        self.spatial_index.clear();
        for (idx, creature) in self.creatures.iter().enumerate() {
            if creature.alive {
                self.spatial_index
                    .entry((creature.y, creature.x))
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }
    }

    /// Run one simulation tick.
    pub fn tick(&mut self) {
        self.tick += 1;

        // Reset ecosystem counts
        for row in &mut self.ecosystems {
            for eco in row.iter_mut() {
                eco.reset_counts();
                eco.tick_plant_growth();
            }
        }

        let width = self.config.world_width as usize;
        let height = self.config.world_height as usize;

        // Phase 1: Movement and environment
        for i in 0..self.creatures.len() {
            if !self.creatures[i].alive {
                continue;
            }

            // Choose new direction periodically
            if self.creatures[i].target_x.is_none() || self.tick % 3 == 0 {
                self.creatures[i].choose_direction(width, height, &mut self.rng);
            }

            // Move
            self.creatures[i].move_towards_target(width, height);

            // Apply environmental stress
            let x = self.creatures[i].x;
            let y = self.creatures[i].y;
            let temp = self.world.terrain[y][x].temperature;
            let rain = self.world.terrain[y][x].rainfall;

            self.creatures[i].apply_temperature_stress(temp);
            self.creatures[i].apply_drought_stress(rain);

            // Metabolism
            self.creatures[i].tick_metabolism();
        }

        // Phase 2: Feeding
        // Rebuild spatial index after movement
        self.rebuild_spatial_index();

        // Update ecosystem creature counts
        for creature in &self.creatures {
            if !creature.alive {
                continue;
            }
            let eco = &mut self.ecosystems[creature.y][creature.x];
            eco.creature_count += 1;
            if creature.phenotype.is_herbivore() {
                eco.herbivore_count += 1;
            }
            if creature.phenotype.is_carnivore() {
                eco.carnivore_count += 1;
            }
        }

        // Herbivore feeding
        for i in 0..self.creatures.len() {
            if !self.creatures[i].alive {
                continue;
            }
            if self.creatures[i].phenotype.diet < 0.6 {
                let x = self.creatures[i].x;
                let y = self.creatures[i].y;
                let available = self.ecosystems[y][x].plant_biomass;
                let consumed = self.creatures[i].eat_plants(available);
                self.ecosystems[y][x].consume_plants(consumed);
            }
        }

        // Carnivore hunting
        let spatial_clone = self.spatial_index.clone();
        for i in 0..self.creatures.len() {
            if !self.creatures[i].alive || self.creatures[i].phenotype.diet < 0.2 {
                continue;
            }

            let pos = (self.creatures[i].y, self.creatures[i].x);
            if let Some(tile_creatures) = spatial_clone.get(&pos) {
                // Find a prey target on the same tile
                for &prey_idx in tile_creatures {
                    if prey_idx == i || !self.creatures[prey_idx].alive {
                        continue;
                    }

                    // Prefer hunting smaller creatures or herbivores
                    if self.creatures[prey_idx].phenotype.body_size
                        < self.creatures[i].phenotype.body_size * 1.5
                    {
                        // Split borrows manually using indices
                        let (hunter, prey) = if i < prey_idx {
                            let (left, right) = self.creatures.split_at_mut(prey_idx);
                            (&mut left[i], &mut right[0])
                        } else {
                            let (left, right) = self.creatures.split_at_mut(i);
                            (&mut right[0], &mut left[prey_idx])
                        };

                        if hunter.hunt(prey, &mut self.rng) {
                            break; // One kill per tick
                        }
                    }
                }
            }
        }

        // Phase 3: Reproduction
        let mut new_creatures: Vec<Creature> = Vec::new();

        let spatial_clone = self.spatial_index.clone();
        for i in 0..self.creatures.len() {
            if !self.creatures[i].can_reproduce() {
                continue;
            }
            if self.creatures.len() + new_creatures.len() >= self.config.max_creatures {
                break;
            }

            let pos = (self.creatures[i].y, self.creatures[i].x);
            if let Some(tile_creatures) = spatial_clone.get(&pos) {
                for &partner_idx in tile_creatures {
                    if partner_idx == i
                        || !self.creatures[partner_idx].alive
                        || !self.creatures[partner_idx].can_reproduce()
                    {
                        continue;
                    }

                    // Must be same species (or close enough genetically)
                    if self.creatures[i].species_id != self.creatures[partner_idx].species_id {
                        continue;
                    }

                    let next_id = self.next_creature_id;

                    let (parent_a, parent_b) = if i < partner_idx {
                        let (left, right) = self.creatures.split_at_mut(partner_idx);
                        (&mut left[i], &mut right[0])
                    } else {
                        let (left, right) = self.creatures.split_at_mut(i);
                        (&mut right[0], &mut left[partner_idx])
                    };

                    let offspring = parent_a.reproduce(parent_b, next_id, &mut self.rng);
                    self.next_creature_id += offspring.len() as u64;

                    for child in offspring {
                        self.species_registry
                            .record_birth(child.species_id, child.generation);
                        new_creatures.push(child);
                    }
                    break; // One reproduction event per tick per creature
                }
            }
        }

        // Add new creatures
        self.creatures.extend(new_creatures);

        // Phase 4: Remove dead creatures
        let tick = self.tick;
        let registry = &mut self.species_registry;
        self.creatures.retain(|c| {
            if !c.alive {
                registry.record_death(c.species_id, tick);
                false
            } else {
                true
            }
        });

        // Phase 5: Speciation check (periodic)
        if self.tick % self.config.speciation_check_interval == 0 {
            self.check_speciation();
        }

        // Phase 6: Record population snapshot
        if self.tick % 10 == 0 {
            let herbivores = self
                .creatures
                .iter()
                .filter(|c| c.alive && c.phenotype.is_herbivore())
                .count() as u32;
            let carnivores = self
                .creatures
                .iter()
                .filter(|c| c.alive && c.phenotype.is_carnivore())
                .count() as u32;
            let total = self.creatures.len() as u32;

            self.population_history.push(PopulationSnapshot {
                tick: self.tick,
                total,
                herbivores,
                carnivores,
                species_count: self.species_registry.living_species().len() as u32,
            });

            // Keep history manageable (last 500 entries)
            if self.population_history.len() > 500 {
                self.population_history.drain(0..100);
            }
        }
    }

    fn check_speciation(&mut self) {
        let mut reassignments: Vec<(usize, u32)> = Vec::new();

        for (idx, creature) in self.creatures.iter().enumerate() {
            if !creature.alive {
                continue;
            }

            if self
                .species_registry
                .check_speciation(&creature.genome, creature.species_id, self.config.speciation_threshold)
            {
                // Create new species from this creature's genome
                let new_species_id = self.species_registry.create_species(
                    Some(creature.species_id),
                    creature.genome.clone(),
                    self.tick,
                    &mut self.rng,
                );
                reassignments.push((idx, new_species_id));
            }
        }

        // Apply reassignments
        for (idx, new_species_id) in reassignments {
            let old_species_id = self.creatures[idx].species_id;
            self.creatures[idx].species_id = new_species_id;
            self.species_registry.record_death(old_species_id, self.tick);
            self.species_registry
                .record_birth(new_species_id, self.creatures[idx].generation);

            // Reassign nearby creatures of the same old species with similar genomes
            let cx = self.creatures[idx].x;
            let cy = self.creatures[idx].y;
            let genome = self.creatures[idx].genome.clone();

            for other_idx in 0..self.creatures.len() {
                if other_idx == idx || !self.creatures[other_idx].alive {
                    continue;
                }
                if self.creatures[other_idx].species_id != old_species_id {
                    continue;
                }

                let dx = (self.creatures[other_idx].x as i32 - cx as i32).abs();
                let dy = (self.creatures[other_idx].y as i32 - cy as i32).abs();
                if dx > 15 || dy > 15 {
                    continue;
                }

                if self.creatures[other_idx].genome.distance(&genome) < self.config.speciation_threshold * 0.7 {
                    self.creatures[other_idx].species_id = new_species_id;
                    self.species_registry.record_death(old_species_id, self.tick);
                    self.species_registry
                        .record_birth(new_species_id, self.creatures[other_idx].generation);
                }
            }
        }
    }

    // ========== Data accessors for WASM API ==========

    /// Get the terrain color data as a flat RGBA buffer.
    pub fn get_terrain_rgba(&self) -> Vec<u8> {
        let w = self.config.world_width as usize;
        let h = self.config.world_height as usize;
        let mut buf = vec![0u8; w * h * 4];

        for y in 0..h {
            for x in 0..w {
                let biome = self.world.terrain[y][x].dominant_biome();
                let [r, g, b] = biome.color();
                let idx = (y * w + x) * 4;
                buf[idx] = r;
                buf[idx + 1] = g;
                buf[idx + 2] = b;
                buf[idx + 3] = 255;
            }
        }

        buf
    }

    /// Get creature data as flat buffer: [x, y, r, g, b, size, diet, ...] per creature.
    /// Each creature takes 8 floats.
    pub fn get_creature_data(&self) -> Vec<f32> {
        let mut buf = Vec::with_capacity(self.creatures.len() * 8);

        for c in &self.creatures {
            if !c.alive {
                continue;
            }

            let color = self
                .species_registry
                .species
                .get(&c.species_id)
                .map(|s| s.color)
                .unwrap_or([200, 200, 200]);

            buf.push(c.x as f32 + c.sub_x);
            buf.push(c.y as f32 + c.sub_y);
            buf.push(color[0] as f32);
            buf.push(color[1] as f32);
            buf.push(color[2] as f32);
            buf.push(c.phenotype.body_size);
            buf.push(c.phenotype.diet);
            buf.push(c.energy);
        }

        buf
    }

    /// Get a JSON-serializable summary of the simulation state.
    pub fn get_stats_json(&self) -> String {
        let living = self.species_registry.living_species();
        let total_pop: u32 = living.iter().map(|s| s.population).sum();

        let species_info: Vec<serde_json::Value> = living
            .iter()
            .map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "name": s.name,
                    "color": s.color,
                    "population": s.population,
                    "diet": s.diet_label.name(),
                    "peak_population": s.peak_population,
                    "avg_generation": format!("{:.1}", s.average_generation()),
                    "ancestor_id": s.ancestor_id,
                })
            })
            .collect();

        serde_json::json!({
            "tick": self.tick,
            "total_population": total_pop,
            "total_creatures": self.creatures.len(),
            "living_species_count": living.len(),
            "total_species_ever": self.species_registry.species.len(),
            "species": species_info,
        })
        .to_string()
    }

    /// Get population history for graphing.
    pub fn get_history_json(&self) -> String {
        serde_json::to_string(&self.population_history).unwrap_or_default()
    }

    /// Get info about a specific tile.
    pub fn get_tile_info_json(&self, x: u32, y: u32) -> String {
        let x = x as usize;
        let y = y as usize;

        if y >= self.config.world_height as usize || x >= self.config.world_width as usize {
            return "{}".to_string();
        }

        let cell = &self.world.terrain[y][x];
        let eco = &self.ecosystems[y][x];
        let biome = cell.dominant_biome();

        let creatures_here: Vec<serde_json::Value> = self
            .creatures
            .iter()
            .filter(|c| c.alive && c.x == x && c.y == y)
            .take(20) // Limit to avoid huge responses
            .map(|c| {
                let species_name = self
                    .species_registry
                    .species
                    .get(&c.species_id)
                    .map(|s| s.name.as_str())
                    .unwrap_or("Unknown");

                serde_json::json!({
                    "id": c.id,
                    "species": species_name,
                    "species_id": c.species_id,
                    "age": c.age,
                    "energy": format!("{:.1}", c.energy),
                    "health": format!("{:.2}", c.health),
                    "size": format!("{:.1}", c.phenotype.body_size),
                    "speed": format!("{:.1}", c.phenotype.speed),
                    "diet": format!("{:.2}", c.phenotype.diet),
                    "diet_label": if c.phenotype.is_herbivore() { "Herbivore" } else if c.phenotype.is_carnivore() { "Carnivore" } else { "Omnivore" },
                    "cold_tol": format!("{:.1}", c.phenotype.cold_tolerance),
                    "heat_tol": format!("{:.1}", c.phenotype.heat_tolerance),
                    "camouflage": format!("{:.2}", c.phenotype.camouflage),
                    "generation": c.generation,
                    "kills": c.kills,
                    "children": c.children_produced,
                    "activity": format!("{:?}", c.activity),
                })
            })
            .collect();

        let biome_presences: Vec<serde_json::Value> = cell
            .biome_presences
            .iter()
            .map(|(bt, p): &(crate::biome::BiomeType, f32)| {
                serde_json::json!({
                    "biome": bt.name(),
                    "presence": format!("{:.0}%", p * 100.0),
                })
            })
            .collect();

        serde_json::json!({
            "x": x,
            "y": y,
            "altitude": format!("{:.0}", cell.altitude),
            "temperature": format!("{:.1}", cell.temperature),
            "rainfall": format!("{:.0}", cell.rainfall),
            "biome": biome.name(),
            "biome_presences": biome_presences,
            "plant_biomass": format!("{:.1}", eco.plant_biomass),
            "max_biomass": format!("{:.1}", eco.max_biomass),
            "creature_count": eco.creature_count,
            "creatures": creatures_here,
        })
        .to_string()
    }
}
