use crate::genetics::{Genome, Phenotype};
use crate::planet::PlanetConfig;
use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};

/// Unique creature identifier.
pub type CreatureId = u64;

/// What the creature is currently doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Activity {
    Idle,
    MovingToFood,
    Eating,
    Hunting,
    Fleeing,
    Reproducing,
    Wandering,
    SeekingMate,
}

/// Sex used for sexual reproduction. Assigned once at birth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sex {
    Female,
    Male,
}

/// An individual creature in the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creature {
    pub id: CreatureId,
    pub species_id: u32,
    pub genome: Genome,
    pub phenotype: Phenotype,
    pub sex: Sex,

    // Position
    pub x: usize,
    pub y: usize,
    /// Sub-tile position for smooth rendering (0.0-1.0).
    pub sub_x: f32,
    pub sub_y: f32,

    // State
    pub energy: f32,
    pub age: u32,
    pub health: f32,
    pub activity: Activity,
    pub alive: bool,

    // Reproduction
    pub reproduction_cooldown: u32,

    // Movement target
    pub target_x: Option<usize>,
    pub target_y: Option<usize>,

    // Statistics
    pub kills: u32,
    pub children_produced: u32,
    pub generation: u32,
}

impl Creature {
    pub fn new(
        id: CreatureId,
        species_id: u32,
        genome: Genome,
        x: usize,
        y: usize,
        generation: u32,
        rng: &mut SmallRng,
    ) -> Self {
        let phenotype = Phenotype::from_genome(&genome);
        let initial_energy = phenotype.fertility_threshold * 0.6;
        let sex = if rng.gen_bool(0.5) { Sex::Female } else { Sex::Male };
        // Juveniles can't breed until they grow up.
        let initial_cooldown = phenotype.maturity_age.max(15);

        Self {
            id,
            species_id,
            genome,
            phenotype,
            sex,
            x,
            y,
            sub_x: rng.gen_range(0.0..1.0),
            sub_y: rng.gen_range(0.0..1.0),
            energy: initial_energy,
            age: 0,
            health: 1.0,
            activity: Activity::Idle,
            alive: true,
            reproduction_cooldown: initial_cooldown,
            target_x: None,
            target_y: None,
            kills: 0,
            children_produced: 0,
            generation,
        }
    }

    pub fn is_mature(&self) -> bool {
        self.phenotype.is_mature(self.age)
    }

    /// Update creature for one tick. Returns energy change.
    pub fn tick_metabolism(&mut self) -> f32 {
        if !self.alive {
            return 0.0;
        }

        self.age += 1;

        // Base metabolism cost
        let cost = self.phenotype.base_energy_cost();
        self.energy -= cost;

        // Reduce reproduction cooldown
        if self.reproduction_cooldown > 0 {
            self.reproduction_cooldown -= 1;
        }

        // Death from old age
        if self.age >= self.phenotype.max_age {
            self.alive = false;
            return -cost;
        }

        // Death from starvation
        if self.energy <= 0.0 {
            self.alive = false;
            return -cost;
        }

        // Death from poor health
        if self.health <= 0.0 {
            self.alive = false;
            return -cost;
        }

        -cost
    }

    /// Apply environmental stress from temperature mismatch.
    pub fn apply_temperature_stress(&mut self, temperature: f32) {
        if !self.alive {
            return;
        }

        if temperature < self.phenotype.cold_tolerance {
            let stress = (self.phenotype.cold_tolerance - temperature) / 20.0;
            self.health -= stress * 0.05;
            self.energy -= stress * 0.5;
        }

        if temperature > self.phenotype.heat_tolerance {
            let stress = (temperature - self.phenotype.heat_tolerance) / 20.0;
            self.health -= stress * 0.05;
            self.energy -= stress * 0.5;
        }
    }

    /// Apply drought stress based on rainfall and drought tolerance.
    pub fn apply_drought_stress(&mut self, rainfall: f32) {
        if !self.alive {
            return;
        }

        let water_need = 1.0 - self.phenotype.drought_tolerance;
        let min_rainfall = water_need * 500.0;

        if rainfall < min_rainfall {
            let stress = (min_rainfall - rainfall) / 500.0;
            self.health -= stress * 0.02;
            self.energy -= stress * 0.3;
        }
    }

    /// Apply planet-level pressures: ionizing radiation, toxic atmosphere,
    /// gravity, and ambient photosynthesis. These are the traits that
    /// make evolution on alien worlds look genuinely different.
    pub fn apply_planet_stress(
        &mut self,
        planet: &PlanetConfig,
        local_radiation_bonus: f32,
        in_liquid_biome: bool,
    ) {
        if !self.alive {
            return;
        }

        // Ionizing radiation damages unshielded DNA -> health loss.
        let rad = (planet.effective_radiation() + local_radiation_bonus).min(1.0);
        let unshielded = (rad - self.phenotype.radiation_tolerance).max(0.0);
        if unshielded > 0.0 {
            self.health -= unshielded * 0.035;
            self.energy -= unshielded * 0.4;
        }

        // Toxic atmosphere: anyone not chemically adapted suffers.
        let tox = planet.toxic_load();
        let exposed = (tox - self.phenotype.toxic_tolerance).max(0.0);
        if exposed > 0.0 {
            self.health -= exposed * 0.04;
            self.energy -= exposed * 0.5;
        }

        // Pressure mismatch is symmetric: too-low and too-high are both bad.
        // Gene 0.0 = vacuum-adapted, 1.0 = deep-crush-adapted.
        let p_target = (planet.surface_pressure / 6.0).clamp(0.0, 1.0);
        let p_gap = (p_target - self.phenotype.pressure_tolerance).abs();
        if p_gap > 0.15 {
            let s = (p_gap - 0.15) * 1.5;
            self.health -= s * 0.02;
            self.energy -= s * 0.25;
        }

        // Gravity scales movement and base metabolism costs.
        // We already burned base metabolism this tick, so pay the delta here.
        let g_penalty = (planet.gravity - 1.0) * self.phenotype.body_size * 0.08;
        if g_penalty > 0.0 {
            self.energy -= g_penalty;
            // Extreme gravity can also injure large bodies.
            if planet.gravity > 1.8 && self.phenotype.body_size > 3.5 {
                self.health -= (planet.gravity - 1.8) * 0.02;
            }
        }

        // Solvent mismatch: being stranded in liquid you can't metabolize is bad.
        if in_liquid_biome {
            // Drowning / solvent incompatibility for any creature not on its
            // home solvent. For simplicity, penalize all land creatures in
            // liquid tiles slightly.
            self.health -= 0.03;
            self.energy -= 0.5;
        }

        // Passive photosynthesis gain, modulated by the star class and
        // local biome productivity. The scale here is small; the real
        // benefit is lower net metabolism in Phenotype::base_energy_cost.
        if self.phenotype.photosynthesis > 0.05 {
            let solar = planet.star_class.photosynthesis_efficiency();
            self.energy += self.phenotype.photosynthesis * solar * 0.4;
        }
    }

    /// Eat plant biomass. Returns amount consumed.
    pub fn eat_plants(&mut self, available: f32) -> f32 {
        if !self.alive || self.phenotype.is_carnivore() {
            return 0.0;
        }

        let herbivore_efficiency = 1.0 - self.phenotype.diet;
        let max_intake = self.phenotype.body_size * 3.0 * herbivore_efficiency;
        let consumed = available.min(max_intake);

        self.energy += consumed * herbivore_efficiency;
        self.activity = Activity::Eating;

        consumed
    }

    /// Attempt to hunt and eat another creature.
    /// `hunter_pack_mul` and `prey_pack_mul` are combat multipliers
    /// computed externally from local same-species adult counts.
    pub fn hunt(
        &mut self,
        prey: &mut Creature,
        hunter_pack_mul: f32,
        prey_pack_mul: f32,
        rng: &mut SmallRng,
    ) -> bool {
        if !self.alive || !prey.alive {
            return false;
        }

        // Juveniles are clumsy hunters.
        if !self.is_mature() {
            return false;
        }
        // Can't hunt if not carnivorous enough.
        if self.phenotype.diet < 0.2 {
            return false;
        }

        // Detection check: prey camouflage vs hunter sense.
        let detection_chance = self.phenotype.sense_range / 8.0
            * (1.0 - prey.phenotype.camouflage * 0.7);
        if !rng.gen_bool(detection_chance.clamp(0.1, 0.95) as f64) {
            return false;
        }

        // Combat: hunter power scales with maturity and pack.
        let hunter_mat = if self.is_mature() { 1.0 } else { 0.5 };
        let prey_mat = if prey.is_mature() { 1.0 } else { 0.5 };
        let hunter_power = self.phenotype.combat_power() * hunter_pack_mul * hunter_mat;
        // Defenders take a disadvantage but armour helps absorb the hit.
        let armor_mul = 1.0 + prey.phenotype.armor * 1.2;
        let prey_power =
            prey.phenotype.combat_power() * 0.6 * prey_pack_mul * prey_mat * armor_mul;

        let success_chance = hunter_power / (hunter_power + prey_power);
        if rng.gen_bool(success_chance.clamp(0.05, 0.95) as f64) {
            // Successful hunt — but venomous prey leaves a parting injury.
            let food = prey.phenotype.food_value() * self.phenotype.diet;
            self.energy += food;
            self.kills += 1;
            self.activity = Activity::Hunting;

            if prey.phenotype.venom > 0.05 {
                self.health -= prey.phenotype.venom * 0.3;
                self.energy -= prey.phenotype.venom * 4.0;
            }
            prey.alive = false;
            true
        } else {
            // Failed hunt, both lose energy. Venom amplifies the
            // counter-injury chance and damage.
            self.energy -= self.phenotype.movement_energy_cost() * 2.0;
            prey.energy -= prey.phenotype.movement_energy_cost();
            let injure_chance = 0.2 + prey.phenotype.venom * 0.5;
            if rng.gen_bool(injure_chance.clamp(0.0, 0.95) as f64) {
                self.health -= 0.1 + prey.phenotype.venom * 0.15;
            }
            false
        }
    }

    /// Check if the creature can reproduce. Maturity is now a real gate.
    pub fn can_reproduce(&self) -> bool {
        self.alive
            && self.energy > self.phenotype.fertility_threshold
            && self.reproduction_cooldown == 0
            && self.is_mature()
    }

    /// Produce offspring with another creature. Sexual reproduction now
    /// requires opposite sexes — same-sex pairs short-circuit out.
    pub fn reproduce(
        &mut self,
        partner: &mut Creature,
        next_id: CreatureId,
        mutation_scale: f32,
        rng: &mut SmallRng,
    ) -> Vec<Creature> {
        if !self.can_reproduce() || !partner.can_reproduce() {
            return Vec::new();
        }
        if self.sex == partner.sex {
            return Vec::new();
        }

        let offspring_count = ((self.phenotype.offspring_count + partner.phenotype.offspring_count) / 2)
            .max(1)
            .min(4);

        let energy_per_offspring = self.phenotype.fertility_threshold * 0.3;

        let mut offspring = Vec::new();
        for i in 0..offspring_count {
            let child_genome =
                Genome::crossover(&self.genome, &partner.genome, mutation_scale, rng);
            let child = Creature::new(
                next_id + i as u64,
                self.species_id,
                child_genome,
                self.x,
                self.y,
                self.generation + 1,
                rng,
            );
            offspring.push(child);
        }

        // Parents lose energy
        let total_cost = energy_per_offspring * offspring_count as f32;
        self.energy -= total_cost * 0.6;
        partner.energy -= total_cost * 0.4;

        self.reproduction_cooldown = 15 + (self.phenotype.max_age / 10);
        partner.reproduction_cooldown = 15 + (partner.phenotype.max_age / 10);

        self.children_produced += offspring_count;
        partner.children_produced += offspring_count;

        self.activity = Activity::Reproducing;
        partner.activity = Activity::Reproducing;

        offspring
    }

    /// Choose movement direction based on surroundings.
    pub fn choose_direction(
        &mut self,
        world_width: usize,
        world_height: usize,
        rng: &mut SmallRng,
    ) {
        if !self.alive {
            return;
        }

        // Random wandering within sense range
        let range = self.phenotype.sense_range as i32;
        let dx = rng.gen_range(-range..=range);
        let dy = rng.gen_range(-range..=range);

        let new_x = ((self.x as i32 + dx).rem_euclid(world_width as i32)) as usize;
        let new_y = (self.y as i32 + dy).clamp(0, world_height as i32 - 1) as usize;

        self.target_x = Some(new_x);
        self.target_y = Some(new_y);
        self.activity = Activity::Wandering;
    }

    /// Move one step towards target.
    pub fn move_towards_target(&mut self, world_width: usize, world_height: usize) {
        if !self.alive {
            return;
        }

        let (tx, ty) = match (self.target_x, self.target_y) {
            (Some(tx), Some(ty)) => (tx, ty),
            _ => return,
        };

        // Calculate shortest path (wrapping horizontally)
        let dx = {
            let d1 = tx as i32 - self.x as i32;
            let d2 = d1 + world_width as i32;
            let d3 = d1 - world_width as i32;
            if d1.abs() <= d2.abs() && d1.abs() <= d3.abs() {
                d1
            } else if d2.abs() <= d3.abs() {
                d2
            } else {
                d3
            }
        };
        let dy = ty as i32 - self.y as i32;

        // Move one step
        let step_x = dx.signum();
        let step_y = dy.signum();

        self.x = ((self.x as i32 + step_x).rem_euclid(world_width as i32)) as usize;
        self.y = (self.y as i32 + step_y).clamp(0, world_height as i32 - 1) as usize;

        // Energy cost for movement
        self.energy -= self.phenotype.movement_energy_cost();

        // Clear target if reached
        if self.x == tx && self.y == ty {
            self.target_x = None;
            self.target_y = None;
            self.activity = Activity::Idle;
        }
    }
}
