use crate::civilization::{Leader, NationId, Population, Settlement, SettlementId};
use crate::culture::{Culture, CultureManager};
use crate::economy::EconomyManager;
use crate::history::{HistoricalEvent, HistoryManager};
use crate::military::{Army, CombatSystem, MilitaryManager, Regiment, UnitType};
use crate::names;
use crate::politics::{GovernmentType, Nation, NationManager};
use crate::religion::{Religion, ReligionManager, ReligionType};
use crate::technology::{TechTree, TechnologyId};
use crate::world::{BiomeType, ResourceType, Tile, World};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

const START_YEAR: i32 = -10000;
const INITIAL_POPULATION_GROUPS: u32 = 20;

/// The main simulation state
pub struct Simulation {
    pub world: World,
    pub current_year: i32,
    pub rng: SmallRng,

    // Entity managers
    pub nations: NationManager,
    pub cultures: CultureManager,
    pub religions: ReligionManager,
    pub military: MilitaryManager,
    pub economy: EconomyManager,
    pub history: HistoryManager,
    pub tech_tree: TechTree,

    // Entity storage
    pub settlements: HashMap<SettlementId, Settlement>,
    pub leaders: HashMap<u64, Leader>,
    pub populations: Vec<Population>,

    // ID counters
    next_settlement_id: SettlementId,
    next_leader_id: u64,
    next_pop_id: u64,

    // Statistics
    pub total_population: u64,
    pub total_nations: u32,
    pub total_settlements: u32,
    pub wars_fought: u32,
}

impl Simulation {
    pub fn new(width: u32, height: u32, seed: u32) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed as u64);

        // Generate world
        let world = World::new(width, height, seed);

        let mut sim = Simulation {
            world,
            current_year: START_YEAR,
            rng,
            nations: NationManager::new(),
            cultures: CultureManager::new(),
            religions: ReligionManager::new(),
            military: MilitaryManager::new(),
            economy: EconomyManager::new(),
            history: HistoryManager::new(),
            tech_tree: TechTree::new(),
            settlements: HashMap::new(),
            leaders: HashMap::new(),
            populations: Vec::new(),
            next_settlement_id: 1,
            next_leader_id: 1,
            next_pop_id: 1,
            total_population: 0,
            total_nations: 0,
            total_settlements: 0,
            wars_fought: 0,
        };

        // Record world creation
        sim.history.record(HistoricalEvent::WorldCreated {
            year: START_YEAR,
            seed,
        });

        // Initialize starting populations
        sim.spawn_initial_populations();

        sim
    }

    fn spawn_initial_populations(&mut self) {
        let suitable_locations = self.world.find_suitable_settlement_locations();
        let num_to_spawn = (INITIAL_POPULATION_GROUPS as usize).min(suitable_locations.len());

        for i in 0..num_to_spawn {
            let (x, y, _score) = suitable_locations[i];

            // Create a culture for this population
            let culture_name = names::generate_culture_name(&mut self.rng);
            let culture_id = self.cultures.create_culture(
                culture_name.clone(),
                self.current_year,
                (x, y),
                &mut self.rng,
            );

            // Create population group
            let pop = Population {
                id: self.next_pop_id,
                name: culture_name.clone(),
                size: self.rng.gen_range(50..200),
                location: (x, y),
                culture_id,
                religion_id: None,
                is_nomadic: self.rng.gen_bool(0.6),
                migration_target: None,
                skills: Default::default(),
            };

            self.total_population += pop.size as u64;
            self.populations.push(pop);
            self.next_pop_id += 1;

            // Potentially create first settlement
            if i == 0 || self.rng.gen_bool(0.3) {
                self.create_settlement((x, y), culture_id, None);
            }
        }
    }

    fn create_settlement(
        &mut self,
        location: (u32, u32),
        culture_id: u64,
        nation_id: Option<NationId>,
    ) -> SettlementId {
        let culture_name = self
            .cultures
            .get(culture_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let name = names::generate_settlement_name(&mut self.rng, &culture_name);
        let settlement_id = self.next_settlement_id;
        self.next_settlement_id += 1;

        let mut settlement =
            Settlement::new(settlement_id, name.clone(), location.0, location.1, self.current_year);
        settlement.primary_culture_id = Some(culture_id);
        settlement.owner_nation_id = nation_id;

        // Add resources from the tile
        if let Some(tile) = self.world.get_tile(location.0, location.1) {
            for resource in &tile.resources {
                settlement
                    .available_resources
                    .insert(*resource, self.rng.gen_range(1..5));
            }
        }

        // Record event
        if self.total_settlements == 0 {
            self.history.record(HistoricalEvent::FirstSettlement {
                year: self.current_year,
                settlement_name: name.clone(),
                location,
            });
        }

        self.history.record(HistoricalEvent::SettlementFounded {
            year: self.current_year,
            settlement_id,
            name,
            nation_id: nation_id.unwrap_or(0),
        });

        self.settlements.insert(settlement_id, settlement);
        self.total_settlements += 1;

        // Update tile ownership
        if let Some(tile) = self.world.get_tile_mut(location.0, location.1) {
            tile.settlement_id = Some(settlement_id);
            tile.owner_nation_id = nation_id;
        }

        settlement_id
    }

    fn create_nation(&mut self, capital_location: (u32, u32), culture_id: u64) -> NationId {
        let culture_name = self
            .cultures
            .get(culture_id)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let name = names::generate_nation_name(&mut self.rng, &culture_name);
        let nation_id = self.nations.create_nation(
            name.clone(),
            self.current_year,
            capital_location,
            &mut self.rng,
        );

        // Create leader
        let leader_id = self.next_leader_id;
        self.next_leader_id += 1;
        let leader = Leader::generate(&mut self.rng, leader_id, self.current_year - 30, &culture_name);

        if let Some(nation) = self.nations.get_mut(nation_id) {
            nation.primary_culture_id = Some(culture_id);
            nation.current_leader_id = Some(leader_id);
            nation.ruling_dynasty = Some(leader.dynasty_name.clone());
        }

        // Record events
        self.history.record(HistoricalEvent::NationFounded {
            year: self.current_year,
            nation_id,
            name: name.clone(),
            founder: leader.name.clone(),
        });

        self.history.record(HistoricalEvent::LeaderRise {
            year: self.current_year,
            nation_id,
            leader_name: leader.name.clone(),
            title: "Chief".to_string(),
        });

        self.leaders.insert(leader_id, leader);
        self.total_nations += 1;

        nation_id
    }

    fn create_religion(&mut self, location: (u32, u32)) -> u64 {
        let name = names::generate_religion_name(&mut self.rng);
        let religion_type = if self.rng.gen_bool(0.6) {
            ReligionType::Polytheism
        } else if self.rng.gen_bool(0.3) {
            ReligionType::Animism
        } else {
            ReligionType::AncestorWorship
        };

        let religion_id =
            self.religions
                .create_religion(name.clone(), religion_type, self.current_year, &mut self.rng);

        if let Some(religion) = self.religions.get_mut(religion_id) {
            religion.holy_city = Some(location);
        }

        self.history.record(HistoricalEvent::ReligionFounded {
            year: self.current_year,
            religion_id,
            name,
            founder: None,
        });

        religion_id
    }

    pub fn tick(&mut self) {
        self.current_year += 1;

        // Process populations
        self.process_populations();

        // Process settlements
        self.process_settlements();

        // Process nations
        self.process_nations();

        // Process cultures
        self.process_cultures();

        // Process religions
        self.process_religions();

        // Process military
        self.process_military();

        // Process economy
        self.economy.tick();

        // Process random events
        self.process_random_events();

        // Update statistics
        self.update_statistics();
    }

    fn process_populations(&mut self) {
        let mut new_settlements: Vec<((u32, u32), u64)> = Vec::new();

        for pop in &mut self.populations {
            // Population growth
            let growth_rate = 0.002 + self.rng.gen_range(-0.001..0.003);
            let growth = (pop.size as f32 * growth_rate) as i32;
            pop.size = (pop.size as i32 + growth).max(10) as u32;

            // Migration for nomadic peoples
            if pop.is_nomadic && self.rng.gen_bool(0.1) {
                // Find a new location
                let neighbors = self.world.get_neighbors(pop.location.0, pop.location.1);
                let valid_neighbors: Vec<(u32, u32)> = neighbors
                    .into_iter()
                    .filter(|(x, y)| {
                        self.world
                            .get_tile(*x, *y)
                            .map(|t| t.is_land() && t.biome.habitability() > 0.2)
                            .unwrap_or(false)
                    })
                    .collect();

                if !valid_neighbors.is_empty() {
                    let idx = self.rng.gen_range(0..valid_neighbors.len());
                    pop.location = valid_neighbors[idx];
                }
            }

            // Chance to settle
            if pop.is_nomadic && pop.size > 100 && self.rng.gen_bool(0.01) {
                let tile = self.world.get_tile(pop.location.0, pop.location.1);
                if let Some(t) = tile {
                    if t.is_land() && t.settlement_id.is_none() && t.biome.habitability() > 0.4 {
                        pop.is_nomadic = false;
                        new_settlements.push((pop.location, pop.culture_id));
                    }
                }
            }
        }

        // Create new settlements
        for (location, culture_id) in new_settlements {
            self.create_settlement(location, culture_id, None);
        }

        // Merge small population groups
        self.populations.retain(|p| p.size > 5);
    }

    fn process_settlements(&mut self) {
        let settlement_ids: Vec<SettlementId> = self.settlements.keys().cloned().collect();

        for id in settlement_ids {
            let (location, fertility, should_create_nation) = {
                let settlement = self.settlements.get(&id).unwrap();
                let tile = self.world.get_tile(settlement.x, settlement.y);
                let fertility = tile.map(|t| t.fertility).unwrap_or(0.5);

                let should_create_nation = settlement.owner_nation_id.is_none()
                    && settlement.population > 500
                    && self.rng.gen_bool(0.02);

                ((settlement.x, settlement.y), fertility, should_create_nation)
            };

            // Update settlement
            if let Some(settlement) = self.settlements.get_mut(&id) {
                settlement.tick(fertility);
            }

            // Potentially form a nation
            if should_create_nation {
                let culture_id = self
                    .settlements
                    .get(&id)
                    .and_then(|s| s.primary_culture_id)
                    .unwrap_or(1);

                let nation_id = self.create_nation(location, culture_id);

                if let Some(settlement) = self.settlements.get_mut(&id) {
                    settlement.owner_nation_id = Some(nation_id);
                }

                if let Some(nation) = self.nations.get_mut(nation_id) {
                    nation.capital_id = Some(id);
                    nation.settlements.push(id);
                }
            }
        }

        // Chance to spawn religion in large settlements
        // First, collect candidates to avoid borrow issues
        let religion_candidates: Vec<(SettlementId, u32, u32)> = self
            .settlements
            .iter()
            .filter(|(_, s)| s.population > 1000 && s.primary_religion_id.is_none())
            .map(|(id, s)| (*id, s.x, s.y))
            .collect();

        for (id, x, y) in religion_candidates {
            if self.rng.gen_bool(0.005) {
                let religion_id = self.create_religion((x, y));
                if let Some(s) = self.settlements.get_mut(&id) {
                    s.primary_religion_id = Some(religion_id);
                }
            }
        }
    }

    fn process_nations(&mut self) {
        self.nations.tick(&self.tech_tree);

        let nation_ids: Vec<NationId> = self
            .nations
            .nations
            .iter()
            .filter(|(_, n)| n.is_alive)
            .map(|(id, _)| *id)
            .collect();

        for nation_id in nation_ids {
            // Research progress
            if let Some(nation) = self.nations.get_mut(nation_id) {
                // Get research from settlements
                let research_points: f32 = nation
                    .settlements
                    .iter()
                    .filter_map(|sid| self.settlements.get(sid))
                    .map(|s| s.calculate_research())
                    .sum();

                let completed = nation
                    .research_state
                    .add_research_points(research_points, &self.tech_tree);

                for tech_id in completed {
                    if let Some(tech) = self.tech_tree.get(tech_id) {
                        self.history.record(HistoricalEvent::TechnologyDiscovered {
                            year: self.current_year,
                            nation_id,
                            tech_id,
                            tech_name: tech.name.clone(),
                        });
                    }
                }

                // Auto-select next research if none
                if nation.research_state.current_research.is_none() {
                    let available = self
                        .tech_tree
                        .get_available(&nation.research_state.researched_technologies);
                    if !available.is_empty() {
                        let idx = self.rng.gen_range(0..available.len());
                        nation
                            .research_state
                            .start_research(available[idx].id, &self.tech_tree);
                    }
                }

                // Expansion
                if nation.stability > 0.5 && self.rng.gen_bool(0.02) {
                    self.try_nation_expansion(nation_id);
                }
            }

            // Check for leader death
            self.check_leader_succession(nation_id);

            // Check for government evolution
            self.check_government_evolution(nation_id);
        }

        // Process diplomacy and wars
        self.process_diplomacy();
    }

    fn try_nation_expansion(&mut self, nation_id: NationId) {
        let current_tiles = {
            self.nations
                .get(nation_id)
                .map(|n| n.controlled_tiles.clone())
                .unwrap_or_default()
        };

        // Find adjacent unclaimed tiles
        let mut expansion_candidates: Vec<(u32, u32)> = Vec::new();

        for (x, y) in &current_tiles {
            let neighbors = self.world.get_neighbors(*x, *y);
            for (nx, ny) in neighbors {
                if let Some(tile) = self.world.get_tile(nx, ny) {
                    if tile.is_land()
                        && tile.owner_nation_id.is_none()
                        && !current_tiles.contains(&(nx, ny))
                    {
                        expansion_candidates.push((nx, ny));
                    }
                }
            }
        }

        // Claim some tiles
        let to_claim = expansion_candidates
            .into_iter()
            .take(3)
            .collect::<Vec<_>>();

        if !to_claim.is_empty() {
            if let Some(nation) = self.nations.get_mut(nation_id) {
                nation.add_territory(&to_claim);
            }

            for (x, y) in &to_claim {
                if let Some(tile) = self.world.get_tile_mut(*x, *y) {
                    tile.owner_nation_id = Some(nation_id);
                }
            }
        }
    }

    fn check_leader_succession(&mut self, nation_id: NationId) {
        let (leader_id, should_die) = {
            if let Some(nation) = self.nations.get(nation_id) {
                if let Some(lid) = nation.current_leader_id {
                    if let Some(leader) = self.leaders.get(&lid) {
                        let age = leader.age(self.current_year);
                        let death_chance = if age > 60 {
                            0.05 + (age - 60) as f32 * 0.01
                        } else {
                            0.005
                        };
                        (Some(lid), self.rng.gen_bool(death_chance as f64))
                    } else {
                        (None, false)
                    }
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            }
        };

        if should_die {
            if let Some(lid) = leader_id {
                let old_name = self
                    .leaders
                    .get(&lid)
                    .map(|l| l.name.clone())
                    .unwrap_or_default();

                if let Some(leader) = self.leaders.get_mut(&lid) {
                    leader.death_year = Some(self.current_year);
                }

                self.history.record(HistoricalEvent::LeaderDeath {
                    year: self.current_year,
                    nation_id,
                    leader_name: old_name,
                    cause: "natural causes".to_string(),
                });

                // Create new leader
                let culture_name = self
                    .nations
                    .get(nation_id)
                    .and_then(|n| n.primary_culture_id)
                    .and_then(|cid| self.cultures.get(cid))
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                let new_leader_id = self.next_leader_id;
                self.next_leader_id += 1;

                // Generate age offset before the call to avoid double mutable borrow
                let age_offset = self.rng.gen_range(25..50);
                let birth_year = self.current_year - age_offset;

                let new_leader = Leader::generate(
                    &mut self.rng,
                    new_leader_id,
                    birth_year,
                    &culture_name,
                );

                let new_name = new_leader.name.clone();
                self.leaders.insert(new_leader_id, new_leader);

                if let Some(nation) = self.nations.get_mut(nation_id) {
                    nation.current_leader_id = Some(new_leader_id);
                    nation.leaders_history.push(new_leader_id);
                    nation.legitimacy -= 0.2;
                }

                self.history.record(HistoricalEvent::LeaderRise {
                    year: self.current_year,
                    nation_id,
                    leader_name: new_name,
                    title: "Ruler".to_string(),
                });
            }
        }
    }

    fn check_government_evolution(&mut self, nation_id: NationId) {
        if let Some(nation) = self.nations.get_mut(nation_id) {
            let pop = nation.total_population;
            let current_gov = nation.government;

            let new_gov = if pop > 100000
                && current_gov == GovernmentType::Tribal
                && self.rng.gen_bool(0.01)
            {
                Some(GovernmentType::Chiefdom)
            } else if pop > 500000
                && matches!(
                    current_gov,
                    GovernmentType::Chiefdom | GovernmentType::Tribal
                )
                && self.rng.gen_bool(0.005)
            {
                Some(GovernmentType::Monarchy)
            } else if pop > 1000000 && self.rng.gen_bool(0.002) {
                if nation.total_population > 5000000 {
                    Some(GovernmentType::Empire)
                } else {
                    Some(GovernmentType::Feudal)
                }
            } else {
                None
            };

            if let Some(new) = new_gov {
                let old_name = format!("{:?}", current_gov);
                let new_name = format!("{:?}", new);

                nation.change_government(new);

                self.history.record(HistoricalEvent::GovernmentChange {
                    year: self.current_year,
                    nation_id,
                    old_type: old_name,
                    new_type: new_name,
                });
            }
        }
    }

    fn process_diplomacy(&mut self) {
        // Simple war declaration logic
        let nations: Vec<(NationId, u32, f32)> = self
            .nations
            .nations
            .iter()
            .filter(|(_, n)| n.is_alive)
            .map(|(id, n)| (*id, n.total_population, n.military_strength))
            .collect();

        for i in 0..nations.len() {
            for j in (i + 1)..nations.len() {
                let (id_a, pop_a, strength_a) = nations[i];
                let (id_b, pop_b, strength_b) = nations[j];

                // Check if neighbors (simplified)
                let are_neighbors = {
                    let tiles_a = self
                        .nations
                        .get(id_a)
                        .map(|n| n.controlled_tiles.clone())
                        .unwrap_or_default();
                    let tiles_b = self
                        .nations
                        .get(id_b)
                        .map(|n| n.controlled_tiles.clone())
                        .unwrap_or_default();

                    tiles_a.iter().any(|(x, y)| {
                        self.world
                            .get_neighbors(*x, *y)
                            .iter()
                            .any(|pos| tiles_b.contains(pos))
                    })
                };

                if are_neighbors
                    && !self
                        .nations
                        .get(id_a)
                        .map(|n| n.at_war_with.contains(&id_b))
                        .unwrap_or(false)
                {
                    // War chance
                    if self.rng.gen_bool(0.001) {
                        // Declare war
                        if let Some(nation_a) = self.nations.get_mut(id_a) {
                            nation_a.declare_war(id_b, self.current_year);
                        }
                        if let Some(nation_b) = self.nations.get_mut(id_b) {
                            nation_b.at_war_with.insert(id_a);
                        }

                        self.history.record(HistoricalEvent::WarDeclared {
                            year: self.current_year,
                            attacker: id_a,
                            defender: id_b,
                            casus_belli: "territorial dispute".to_string(),
                        });

                        self.wars_fought += 1;
                    }
                }
            }
        }

        // Process existing wars
        self.process_wars();
    }

    fn process_wars(&mut self) {
        let wars: Vec<(NationId, NationId)> = self.nations.get_wars();

        for (nation_a, nation_b) in wars {
            // Simple war resolution - chance to end
            if self.rng.gen_bool(0.02) {
                // War ends
                if let Some(nation) = self.nations.get_mut(nation_a) {
                    nation.make_peace(nation_b, self.current_year);
                }
                if let Some(nation) = self.nations.get_mut(nation_b) {
                    nation.at_war_with.remove(&nation_a);
                }

                // Determine winner (simplified)
                let strength_a = self
                    .nations
                    .get(nation_a)
                    .map(|n| n.military_strength)
                    .unwrap_or(0.0);
                let strength_b = self
                    .nations
                    .get(nation_b)
                    .map(|n| n.military_strength)
                    .unwrap_or(0.0);

                let winner = if strength_a > strength_b * 1.2 {
                    Some(nation_a)
                } else if strength_b > strength_a * 1.2 {
                    Some(nation_b)
                } else {
                    None
                };

                self.history.record(HistoricalEvent::WarEnded {
                    year: self.current_year,
                    winner,
                    loser: winner.map(|w| if w == nation_a { nation_b } else { nation_a }),
                    treaty_name: format!("Peace of {}", self.current_year),
                });
            } else {
                // War continues - increase war exhaustion
                if let Some(nation) = self.nations.get_mut(nation_a) {
                    nation.war_exhaustion = (nation.war_exhaustion + 0.01).min(1.0);
                }
                if let Some(nation) = self.nations.get_mut(nation_b) {
                    nation.war_exhaustion = (nation.war_exhaustion + 0.01).min(1.0);
                }
            }
        }
    }

    fn process_cultures(&mut self) {
        let events = self.cultures.tick(&mut self.rng);

        for event in events {
            match event {
                crate::culture::CultureEvent::TraitMutation {
                    culture_id,
                    old_trait,
                    new_trait,
                } => {
                    self.history.record(HistoricalEvent::CultureMutation {
                        year: self.current_year,
                        culture_id,
                        description: format!("{:?} evolved into {:?}", old_trait, new_trait),
                    });
                }
                _ => {}
            }
        }

        // Update culture population counts
        for culture in self.cultures.cultures.values_mut() {
            culture.population = 0;
            culture.settlements = 0;
        }

        for settlement in self.settlements.values() {
            if let Some(culture_id) = settlement.primary_culture_id {
                if let Some(culture) = self.cultures.get_mut(culture_id) {
                    culture.population += settlement.population;
                    culture.settlements += 1;
                }
            }
        }
    }

    fn process_religions(&mut self) {
        let events = self.religions.tick(self.current_year, &mut self.rng);

        for event in events {
            match event {
                crate::religion::ReligionEvent::Schism {
                    parent_id,
                    child_id,
                    child_name,
                } => {
                    self.history.record(HistoricalEvent::ReligionSchism {
                        year: self.current_year,
                        parent_id,
                        child_id,
                        child_name,
                    });
                }
                _ => {}
            }
        }

        // Update religion follower counts
        for religion in self.religions.religions.values_mut() {
            religion.follower_count = 0;
            religion.settlement_count = 0;
        }

        for settlement in self.settlements.values() {
            if let Some(religion_id) = settlement.primary_religion_id {
                if let Some(religion) = self.religions.get_mut(religion_id) {
                    religion.follower_count += settlement.population;
                    religion.settlement_count += 1;
                }
            }
        }

        // Religion spread
        for (_sid, settlement) in &mut self.settlements {
            if settlement.primary_religion_id.is_none() && self.rng.gen_bool(0.01) {
                // Get nearby religions
                let religions: Vec<(u64, f32)> =
                    self.religions.get_spreading_religions((settlement.x, settlement.y));
                if !religions.is_empty() {
                    let (religion_id, _) = religions[0];
                    settlement.primary_religion_id = Some(religion_id);
                }
            }
        }
    }

    fn process_military(&mut self) {
        self.military.tick();

        // Update nation military stats
        for (nation_id, nation) in &mut self.nations.nations {
            let army_strength: f32 = self
                .military
                .get_armies_of_nation(*nation_id)
                .iter()
                .map(|a| a.total_strength())
                .sum();
            nation.military_strength = army_strength;
        }
    }

    fn process_random_events(&mut self) {
        // Plague
        if self.rng.gen_bool(0.0001) && self.total_population > 100000 {
            let mortality = self.rng.gen_range(0.05..0.3);
            let deaths = (self.total_population as f64 * mortality as f64) as u64;

            // Reduce population
            for settlement in self.settlements.values_mut() {
                let local_deaths = (settlement.population as f64 * mortality as f64) as u32;
                settlement.population = settlement.population.saturating_sub(local_deaths);
            }

            let affected_nations: Vec<NationId> = self
                .nations
                .nations
                .iter()
                .filter(|(_, n)| n.is_alive)
                .map(|(id, _)| *id)
                .collect();

            self.history.record(HistoricalEvent::Plague {
                year: self.current_year,
                affected_nations,
                deaths,
            });
        }

        // Famine
        if self.rng.gen_bool(0.001) {
            for (nation_id, nation) in &self.nations.nations {
                if nation.is_alive && self.rng.gen_bool(0.3) {
                    let deaths = (nation.total_population as f64 * 0.05) as u32;
                    self.history.record(HistoricalEvent::Famine {
                        year: self.current_year,
                        nation_id: *nation_id,
                        deaths,
                    });
                }
            }
        }
    }

    fn update_statistics(&mut self) {
        // Update total population
        self.total_population = self.settlements.values().map(|s| s.population as u64).sum();

        // Update nation populations
        for nation in self.nations.nations.values_mut() {
            nation.total_population = nation
                .settlements
                .iter()
                .filter_map(|sid| self.settlements.get(sid))
                .map(|s| s.population)
                .sum();
        }
    }

    // JSON serialization methods for WASM interface

    pub fn get_state_json(&self) -> String {
        let state = SimulationState {
            current_year: self.current_year,
            total_population: self.total_population,
            total_nations: self.nations.nations.values().filter(|n| n.is_alive).count() as u32,
            total_settlements: self.settlements.len() as u32,
            total_cultures: self.cultures.cultures.len() as u32,
            total_religions: self.religions.religions.len() as u32,
            wars_active: self.nations.get_wars().len() as u32,
        };
        serde_json::to_string(&state).unwrap_or_default()
    }

    pub fn get_world_json(&self) -> String {
        let world_data = WorldData {
            width: self.world.width,
            height: self.world.height,
            tiles: self
                .world
                .tiles
                .iter()
                .flatten()
                .map(|t| TileData {
                    x: t.x,
                    y: t.y,
                    biome: format!("{:?}", t.biome),
                    altitude: t.altitude,
                    owner_nation_id: t.owner_nation_id,
                    settlement_id: t.settlement_id,
                })
                .collect(),
        };
        serde_json::to_string(&world_data).unwrap_or_default()
    }

    pub fn get_civilizations_json(&self) -> String {
        let civs: Vec<CivilizationData> = self
            .nations
            .nations
            .values()
            .filter(|n| n.is_alive)
            .map(|n| CivilizationData {
                id: n.id,
                name: n.name.clone(),
                population: n.total_population,
                settlements: n.settlements.len() as u32,
                territory: n.controlled_tiles.len() as u32,
                government: format!("{:?}", n.government),
                treasury: n.treasury,
                military_strength: n.military_strength,
                color: n.primary_color,
            })
            .collect();
        serde_json::to_string(&civs).unwrap_or_default()
    }

    pub fn get_history_json(&self) -> String {
        let events: Vec<HistoryEntry> = self
            .history
            .get_important_events(5)
            .iter()
            .map(|e| HistoryEntry {
                year: e.year(),
                description: e.description(),
                importance: e.importance(),
            })
            .collect();
        serde_json::to_string(&events).unwrap_or_default()
    }

    pub fn get_tile_info_json(&self, x: u32, y: u32) -> String {
        if let Some(tile) = self.world.get_tile(x, y) {
            let settlement = tile
                .settlement_id
                .and_then(|id| self.settlements.get(&id));
            let nation = tile
                .owner_nation_id
                .and_then(|id| self.nations.get(id));

            let info = TileInfo {
                x,
                y,
                biome: format!("{:?}", tile.biome),
                altitude: tile.altitude,
                temperature: tile.temperature,
                rainfall: tile.rainfall,
                is_river: tile.is_river,
                is_coastal: tile.is_coastal,
                resources: tile.resources.iter().map(|r| format!("{:?}", r)).collect(),
                settlement_name: settlement.map(|s| s.name.clone()),
                settlement_population: settlement.map(|s| s.population),
                nation_name: nation.map(|n| n.name.clone()),
            };
            serde_json::to_string(&info).unwrap_or_default()
        } else {
            "{}".to_string()
        }
    }

    pub fn get_statistics_json(&self) -> String {
        let stats = Statistics {
            year: self.current_year,
            total_population: self.total_population,
            total_nations: self.nations.nations.values().filter(|n| n.is_alive).count() as u32,
            total_settlements: self.settlements.len() as u32,
            total_cultures: self.cultures.cultures.len() as u32,
            total_religions: self.religions.religions.len() as u32,
            wars_fought: self.wars_fought,
            technologies_discovered: self
                .nations
                .nations
                .values()
                .map(|n| n.research_state.researched_technologies.len())
                .max()
                .unwrap_or(0) as u32,
        };
        serde_json::to_string(&stats).unwrap_or_default()
    }
}

// Data structures for JSON serialization

#[derive(Serialize, Deserialize)]
struct SimulationState {
    current_year: i32,
    total_population: u64,
    total_nations: u32,
    total_settlements: u32,
    total_cultures: u32,
    total_religions: u32,
    wars_active: u32,
}

#[derive(Serialize, Deserialize)]
struct WorldData {
    width: u32,
    height: u32,
    tiles: Vec<TileData>,
}

#[derive(Serialize, Deserialize)]
struct TileData {
    x: u32,
    y: u32,
    biome: String,
    altitude: f32,
    owner_nation_id: Option<NationId>,
    settlement_id: Option<SettlementId>,
}

#[derive(Serialize, Deserialize)]
struct CivilizationData {
    id: NationId,
    name: String,
    population: u32,
    settlements: u32,
    territory: u32,
    government: String,
    treasury: f32,
    military_strength: f32,
    color: (u8, u8, u8),
}

#[derive(Serialize, Deserialize)]
struct HistoryEntry {
    year: i32,
    description: String,
    importance: u8,
}

#[derive(Serialize, Deserialize)]
struct TileInfo {
    x: u32,
    y: u32,
    biome: String,
    altitude: f32,
    temperature: f32,
    rainfall: f32,
    is_river: bool,
    is_coastal: bool,
    resources: Vec<String>,
    settlement_name: Option<String>,
    settlement_population: Option<u32>,
    nation_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Statistics {
    year: i32,
    total_population: u64,
    total_nations: u32,
    total_settlements: u32,
    total_cultures: u32,
    total_religions: u32,
    wars_fought: u32,
    technologies_discovered: u32,
}
