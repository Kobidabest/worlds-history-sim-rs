use crate::genetics::Genome;
use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tracks species information and handles speciation events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub id: u32,
    pub name: String,
    pub color: [u8; 3],
    pub ancestor_id: Option<u32>,
    pub representative_genome: Genome,
    pub population: u32,
    pub total_born: u64,
    pub total_died: u64,
    pub appeared_tick: u64,
    pub extinct_tick: Option<u64>,
    pub peak_population: u32,
    pub generation_sum: u64,
    pub diet_label: DietLabel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DietLabel {
    Herbivore,
    Omnivore,
    Carnivore,
}

impl DietLabel {
    pub fn from_diet(diet: f32) -> Self {
        if diet < 0.35 {
            DietLabel::Herbivore
        } else if diet > 0.65 {
            DietLabel::Carnivore
        } else {
            DietLabel::Omnivore
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            DietLabel::Herbivore => "Herbivore",
            DietLabel::Omnivore => "Omnivore",
            DietLabel::Carnivore => "Carnivore",
        }
    }
}

/// Name parts for procedural species name generation.
const PREFIXES: &[&str] = &[
    "Gor", "Vel", "Zar", "Mox", "Kri", "Plu", "Dra", "Fen", "Xyl", "Qua",
    "Bri", "Nym", "Tho", "Sil", "Rek", "Onu", "Pal", "Vix", "Hep", "Jor",
    "Cal", "Wyr", "Ath", "Bol", "Eri", "Nex", "Sar", "Tig", "Uma", "Vul",
];

const MIDDLES: &[&str] = &[
    "ath", "eon", "ux", "is", "an", "or", "el", "um", "ix", "os",
    "ar", "en", "il", "op", "ur", "ast", "eth", "in", "ov", "ul",
];

const SUFFIXES: &[&str] = &[
    "us", "a", "ix", "on", "um", "is", "ax", "or", "id", "al",
    "en", "yx", "os", "ia", "ur", "ek", "an", "es", "ot", "il",
];

impl Species {
    pub fn new(
        id: u32,
        ancestor_id: Option<u32>,
        representative_genome: Genome,
        tick: u64,
        rng: &mut SmallRng,
    ) -> Self {
        let diet = representative_genome.genes[crate::genetics::gene::DIET];
        Self {
            id,
            name: generate_species_name(rng),
            color: generate_species_color(rng),
            ancestor_id,
            representative_genome,
            population: 0,
            total_born: 0,
            total_died: 0,
            appeared_tick: tick,
            extinct_tick: None,
            peak_population: 0,
            generation_sum: 0,
            diet_label: DietLabel::from_diet(diet),
        }
    }

    pub fn is_extinct(&self) -> bool {
        self.extinct_tick.is_some()
    }

    pub fn average_generation(&self) -> f64 {
        if self.total_born == 0 {
            0.0
        } else {
            self.generation_sum as f64 / self.total_born as f64
        }
    }
}

fn generate_species_name(rng: &mut SmallRng) -> String {
    let prefix = PREFIXES[rng.gen_range(0..PREFIXES.len())];
    let middle = MIDDLES[rng.gen_range(0..MIDDLES.len())];
    let suffix = SUFFIXES[rng.gen_range(0..SUFFIXES.len())];
    format!("{}{}{}", prefix, middle, suffix)
}

fn generate_species_color(rng: &mut SmallRng) -> [u8; 3] {
    // Generate saturated, visible colors
    let hue: f32 = rng.gen_range(0.0..360.0);
    let sat: f32 = rng.gen_range(0.5..1.0);
    let val: f32 = rng.gen_range(0.5..0.95);

    hsv_to_rgb(hue, sat, val)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}

/// Manages all species in the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesRegistry {
    pub species: HashMap<u32, Species>,
    pub next_id: u32,
}

impl SpeciesRegistry {
    pub fn new() -> Self {
        Self {
            species: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_species(
        &mut self,
        ancestor_id: Option<u32>,
        representative_genome: Genome,
        tick: u64,
        rng: &mut SmallRng,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let species = Species::new(id, ancestor_id, representative_genome, tick, rng);
        self.species.insert(id, species);
        id
    }

    pub fn record_birth(&mut self, species_id: u32, generation: u32) {
        if let Some(sp) = self.species.get_mut(&species_id) {
            sp.population += 1;
            sp.total_born += 1;
            sp.generation_sum += generation as u64;
            if sp.population > sp.peak_population {
                sp.peak_population = sp.population;
            }
        }
    }

    pub fn record_death(&mut self, species_id: u32, tick: u64) {
        if let Some(sp) = self.species.get_mut(&species_id) {
            sp.population = sp.population.saturating_sub(1);
            sp.total_died += 1;
            if sp.population == 0 && sp.extinct_tick.is_none() {
                sp.extinct_tick = Some(tick);
            }
        }
    }

    pub fn living_species(&self) -> Vec<&Species> {
        self.species.values().filter(|s| !s.is_extinct()).collect()
    }

    pub fn total_population(&self) -> u32 {
        self.species.values().map(|s| s.population).sum()
    }

    /// Check if a creature's genome has diverged enough from its species
    /// to warrant creating a new species.
    pub fn check_speciation(
        &self,
        genome: &Genome,
        species_id: u32,
        threshold: f32,
    ) -> bool {
        if let Some(sp) = self.species.get(&species_id) {
            genome.distance(&sp.representative_genome) > threshold
        } else {
            false
        }
    }
}
