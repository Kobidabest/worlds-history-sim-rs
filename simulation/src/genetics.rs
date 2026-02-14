use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};

/// Number of genes in the genome.
pub const GENE_COUNT: usize = 14;

/// Gene indices for readability.
pub mod gene {
    pub const BODY_SIZE: usize = 0;
    pub const SPEED: usize = 1;
    pub const SENSE_RANGE: usize = 2;
    pub const DIET: usize = 3; // 0=herbivore, 1=carnivore
    pub const COLD_TOLERANCE: usize = 4;
    pub const HEAT_TOLERANCE: usize = 5;
    pub const CAMOUFLAGE: usize = 6;
    pub const AGGRESSION: usize = 7;
    pub const LONGEVITY: usize = 8;
    pub const FERTILITY: usize = 9;
    pub const OFFSPRING_COUNT: usize = 10;
    pub const ENERGY_EFFICIENCY: usize = 11;
    pub const WATER_NEED: usize = 12; // drought tolerance (inverted)
    pub const LEG_STRENGTH: usize = 13;
}

/// A genome is a fixed-size array of gene values in [0.0, 1.0].
/// Each gene value represents a normalized trait.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub genes: [f32; GENE_COUNT],
}

impl Genome {
    /// Create a random genome.
    pub fn random(rng: &mut SmallRng) -> Self {
        let mut genes = [0.0f32; GENE_COUNT];
        for g in genes.iter_mut() {
            *g = rng.gen_range(0.0..1.0);
        }
        Self { genes }
    }

    /// Create a herbivore-biased starter genome.
    pub fn random_herbivore(rng: &mut SmallRng) -> Self {
        let mut genome = Self::random(rng);
        genome.genes[gene::DIET] = rng.gen_range(0.0..0.25);
        genome.genes[gene::AGGRESSION] = rng.gen_range(0.0..0.3);
        genome.genes[gene::BODY_SIZE] = rng.gen_range(0.2..0.6);
        genome.genes[gene::SPEED] = rng.gen_range(0.3..0.7);
        genome
    }

    /// Create a carnivore-biased starter genome.
    pub fn random_carnivore(rng: &mut SmallRng) -> Self {
        let mut genome = Self::random(rng);
        genome.genes[gene::DIET] = rng.gen_range(0.75..1.0);
        genome.genes[gene::AGGRESSION] = rng.gen_range(0.6..1.0);
        genome.genes[gene::BODY_SIZE] = rng.gen_range(0.4..0.8);
        genome.genes[gene::SPEED] = rng.gen_range(0.5..0.9);
        genome.genes[gene::SENSE_RANGE] = rng.gen_range(0.5..1.0);
        genome
    }

    /// Sexual reproduction: crossover of two parent genomes with mutation.
    pub fn crossover(parent_a: &Genome, parent_b: &Genome, rng: &mut SmallRng) -> Self {
        let mut genes = [0.0f32; GENE_COUNT];
        let crossover_point = rng.gen_range(1..GENE_COUNT);

        for i in 0..GENE_COUNT {
            // Crossover
            let base = if i < crossover_point {
                parent_a.genes[i]
            } else {
                parent_b.genes[i]
            };

            // Blending with small chance
            let blended = if rng.gen_bool(0.3) {
                (parent_a.genes[i] + parent_b.genes[i]) / 2.0
            } else {
                base
            };

            // Mutation
            let mutated = if rng.gen_bool(0.08) {
                // 8% per gene mutation rate
                let delta = rng.gen_range(-0.15..0.15);
                (blended + delta).clamp(0.0, 1.0)
            } else {
                blended
            };

            genes[i] = mutated;
        }

        Self { genes }
    }

    /// Compute genetic distance between two genomes (Euclidean distance normalized).
    pub fn distance(&self, other: &Genome) -> f32 {
        let sum: f32 = self
            .genes
            .iter()
            .zip(other.genes.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum();
        (sum / GENE_COUNT as f32).sqrt()
    }
}

/// Expressed phenotype derived from genome. These are the actual values
/// used in simulation mechanics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phenotype {
    /// Body size: affects energy consumption, combat, predation. Range: 0.2 - 5.0 kg equivalent.
    pub body_size: f32,
    /// Movement speed: tiles per tick. Range: 0.3 - 3.0.
    pub speed: f32,
    /// How far the creature can detect food/threats. Range: 1.0 - 8.0 tiles.
    pub sense_range: f32,
    /// Diet preference: 0.0 = pure herbivore, 1.0 = pure carnivore.
    pub diet: f32,
    /// Minimum tolerable temperature.
    pub cold_tolerance: f32,
    /// Maximum tolerable temperature.
    pub heat_tolerance: f32,
    /// Camouflage effectiveness: 0.0 - 1.0.
    pub camouflage: f32,
    /// Aggression: probability of hunting vs fleeing. 0.0 - 1.0.
    pub aggression: f32,
    /// Maximum lifespan in ticks. Range: 50 - 500.
    pub max_age: u32,
    /// Energy threshold for reproduction. Lower = breeds more often.
    pub fertility_threshold: f32,
    /// Number of offspring per reproduction event. 1 - 4.
    pub offspring_count: u32,
    /// Metabolic efficiency: lower means less energy burned per tick. 0.5 - 2.0.
    pub metabolic_rate: f32,
    /// Drought tolerance: 0.0 = needs lots of water, 1.0 = desert-adapted.
    pub drought_tolerance: f32,
    /// Movement cost reduction in rough terrain.
    pub terrain_mobility: f32,
}

impl Phenotype {
    /// Express a genome into a phenotype.
    pub fn from_genome(genome: &Genome) -> Self {
        let g = &genome.genes;

        Self {
            body_size: 0.2 + g[gene::BODY_SIZE] * 4.8,
            speed: 0.3 + g[gene::SPEED] * 2.7,
            sense_range: 1.0 + g[gene::SENSE_RANGE] * 7.0,
            diet: g[gene::DIET],
            cold_tolerance: -35.0 + g[gene::COLD_TOLERANCE] * 40.0,
            heat_tolerance: -5.0 + g[gene::HEAT_TOLERANCE] * 40.0,
            camouflage: g[gene::CAMOUFLAGE],
            aggression: g[gene::AGGRESSION],
            max_age: 50 + (g[gene::LONGEVITY] * 450.0) as u32,
            fertility_threshold: 30.0 + (1.0 - g[gene::FERTILITY]) * 70.0,
            offspring_count: 1 + (g[gene::OFFSPRING_COUNT] * 3.0) as u32,
            metabolic_rate: 0.5 + (1.0 - g[gene::ENERGY_EFFICIENCY]) * 1.5,
            drought_tolerance: g[gene::WATER_NEED],
            terrain_mobility: g[gene::LEG_STRENGTH],
        }
    }

    /// Check if this creature can survive in the given temperature.
    pub fn can_tolerate_temperature(&self, temp: f32) -> bool {
        temp >= self.cold_tolerance && temp <= self.heat_tolerance
    }

    /// Energy cost per tick for basic metabolism.
    pub fn base_energy_cost(&self) -> f32 {
        self.body_size * self.metabolic_rate * 0.3
    }

    /// Energy cost for moving one tile.
    pub fn movement_energy_cost(&self) -> f32 {
        self.body_size * self.speed * 0.15
    }

    /// How much energy this creature provides when eaten.
    pub fn food_value(&self) -> f32 {
        self.body_size * 12.0
    }

    /// Combat strength for hunting/defense.
    pub fn combat_power(&self) -> f32 {
        self.body_size * self.speed * 0.5 + self.aggression * 2.0
    }

    /// Is this creature primarily a herbivore?
    pub fn is_herbivore(&self) -> bool {
        self.diet < 0.4
    }

    /// Is this creature primarily a carnivore?
    pub fn is_carnivore(&self) -> bool {
        self.diet > 0.6
    }
}
