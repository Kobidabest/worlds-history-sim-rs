use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type CultureId = u64;

/// Fundamental cultural traits that affect behavior and bonuses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CulturalTrait {
    // Social organization
    Individualistic,
    Collectivist,
    Hierarchical,
    Egalitarian,
    Patriarchal,
    Matriarchal,

    // Economic tendencies
    Agrarian,
    Pastoral,
    Maritime,
    Mercantile,
    Industrious,
    Artistic,

    // Military tendencies
    Warlike,
    Peaceful,
    Defensive,
    Expansionist,
    Isolationist,
    Diplomatic,

    // Values
    Traditional,
    Progressive,
    Religious,
    Secular,
    Honorbound,
    Pragmatic,

    // Lifestyle
    Nomadic,
    Sedentary,
    Urban,
    Rural,

    // Special
    Seafaring,
    MountainDwelling,
    DesertAdapted,
    ForestDwelling,
    SteppePeople,
}

impl CulturalTrait {
    pub fn combat_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Warlike => 0.2,
            CulturalTrait::Peaceful => -0.1,
            CulturalTrait::Defensive => 0.1,
            CulturalTrait::Honorbound => 0.1,
            CulturalTrait::SteppePeople => 0.15,
            _ => 0.0,
        }
    }

    pub fn defense_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Defensive => 0.25,
            CulturalTrait::MountainDwelling => 0.2,
            CulturalTrait::Isolationist => 0.1,
            _ => 0.0,
        }
    }

    pub fn production_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Industrious => 0.2,
            CulturalTrait::Collectivist => 0.1,
            CulturalTrait::Urban => 0.15,
            _ => 0.0,
        }
    }

    pub fn food_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Agrarian => 0.25,
            CulturalTrait::Pastoral => 0.15,
            CulturalTrait::Rural => 0.1,
            _ => 0.0,
        }
    }

    pub fn trade_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Mercantile => 0.3,
            CulturalTrait::Maritime => 0.2,
            CulturalTrait::Seafaring => 0.15,
            CulturalTrait::Urban => 0.1,
            CulturalTrait::Isolationist => -0.2,
            _ => 0.0,
        }
    }

    pub fn culture_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Artistic => 0.3,
            CulturalTrait::Traditional => 0.15,
            CulturalTrait::Urban => 0.1,
            CulturalTrait::Progressive => 0.1,
            _ => 0.0,
        }
    }

    pub fn research_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Progressive => 0.2,
            CulturalTrait::Urban => 0.15,
            CulturalTrait::Secular => 0.1,
            CulturalTrait::Traditional => -0.1,
            _ => 0.0,
        }
    }

    pub fn faith_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Religious => 0.3,
            CulturalTrait::Traditional => 0.1,
            CulturalTrait::Secular => -0.2,
            _ => 0.0,
        }
    }

    pub fn expansion_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Expansionist => 0.3,
            CulturalTrait::Nomadic => 0.2,
            CulturalTrait::Seafaring => 0.15,
            CulturalTrait::SteppePeople => 0.2,
            CulturalTrait::Isolationist => -0.3,
            CulturalTrait::Sedentary => -0.1,
            _ => 0.0,
        }
    }

    pub fn diplomacy_modifier(&self) -> f32 {
        match self {
            CulturalTrait::Diplomatic => 0.3,
            CulturalTrait::Mercantile => 0.15,
            CulturalTrait::Honorbound => 0.1,
            CulturalTrait::Warlike => -0.15,
            CulturalTrait::Isolationist => -0.2,
            _ => 0.0,
        }
    }

    /// Traits that tend to mutate into this trait
    pub fn can_evolve_from(&self) -> Vec<CulturalTrait> {
        match self {
            CulturalTrait::Urban => vec![CulturalTrait::Sedentary, CulturalTrait::Mercantile],
            CulturalTrait::Sedentary => vec![CulturalTrait::Nomadic, CulturalTrait::Agrarian],
            CulturalTrait::Mercantile => {
                vec![CulturalTrait::Maritime, CulturalTrait::Urban]
            }
            CulturalTrait::Progressive => vec![CulturalTrait::Urban, CulturalTrait::Mercantile],
            CulturalTrait::Secular => vec![CulturalTrait::Progressive, CulturalTrait::Urban],
            CulturalTrait::Diplomatic => vec![CulturalTrait::Mercantile, CulturalTrait::Peaceful],
            _ => vec![],
        }
    }

    /// Check if two traits are incompatible
    pub fn conflicts_with(&self, other: &CulturalTrait) -> bool {
        matches!(
            (self, other),
            (CulturalTrait::Warlike, CulturalTrait::Peaceful)
                | (CulturalTrait::Peaceful, CulturalTrait::Warlike)
                | (CulturalTrait::Nomadic, CulturalTrait::Sedentary)
                | (CulturalTrait::Sedentary, CulturalTrait::Nomadic)
                | (CulturalTrait::Traditional, CulturalTrait::Progressive)
                | (CulturalTrait::Progressive, CulturalTrait::Traditional)
                | (CulturalTrait::Religious, CulturalTrait::Secular)
                | (CulturalTrait::Secular, CulturalTrait::Religious)
                | (CulturalTrait::Individualistic, CulturalTrait::Collectivist)
                | (CulturalTrait::Collectivist, CulturalTrait::Individualistic)
                | (CulturalTrait::Expansionist, CulturalTrait::Isolationist)
                | (CulturalTrait::Isolationist, CulturalTrait::Expansionist)
                | (CulturalTrait::Urban, CulturalTrait::Rural)
                | (CulturalTrait::Rural, CulturalTrait::Urban)
        )
    }
}

/// Cultural practices and customs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CulturalPractice {
    // Social customs
    MonogamousMarriage,
    PolygamousMarriage,
    AncestorWorship,
    ElderVeneration,
    YouthCelebration,

    // Burial practices
    Burial,
    Cremation,
    SkyBurial,
    SeaBurial,
    Mummification,

    // Food customs
    Vegetarianism,
    RitualFeasting,
    FastingTraditions,
    FoodTaboos,

    // Arts
    OralTradition,
    WrittenLiterature,
    MonumentalArchitecture,
    FineArts,
    MusicTradition,
    DanceTradition,

    // Social
    HospitalityCode,
    CastSystem,
    MeritocraticIdeal,
    MilitaryService,

    // Economic
    Bartering,
    Guilds,
    Apprenticeship,
}

/// A distinct cultural group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    pub id: CultureId,
    pub name: String,
    pub parent_culture_id: Option<CultureId>,
    pub origin_year: i32,
    pub origin_location: (u32, u32),

    pub traits: Vec<CulturalTrait>,
    pub practices: Vec<CulturalPractice>,

    // Language family tracking
    pub language_family: String,
    pub language_name: String,

    // Aesthetic preferences
    pub primary_color: (u8, u8, u8),
    pub secondary_color: (u8, u8, u8),
    pub architectural_style: String,
    pub art_style: String,
    pub music_style: String,

    // Cultural values (0-100)
    pub valor: u8,
    pub piety: u8,
    pub learning: u8,
    pub honor: u8,
    pub wealth: u8,
    pub tradition: u8,

    // Spread and influence
    pub prestige: f32,
    pub assimilation_resistance: f32,
    pub spread_rate: f32,

    // Statistics
    pub population: u32,
    pub settlements: u32,
}

impl Culture {
    pub fn new(
        id: CultureId,
        name: String,
        origin_year: i32,
        origin_location: (u32, u32),
        rng: &mut SmallRng,
    ) -> Self {
        let mut traits = Vec::new();
        let primary_color = (rng.gen(), rng.gen(), rng.gen());
        let secondary_color = (rng.gen(), rng.gen(), rng.gen());

        // Generate random traits
        let all_traits = [
            CulturalTrait::Individualistic,
            CulturalTrait::Collectivist,
            CulturalTrait::Agrarian,
            CulturalTrait::Pastoral,
            CulturalTrait::Maritime,
            CulturalTrait::Mercantile,
            CulturalTrait::Warlike,
            CulturalTrait::Peaceful,
            CulturalTrait::Traditional,
            CulturalTrait::Religious,
            CulturalTrait::Sedentary,
        ];

        let num_traits = rng.gen_range(3..6);
        for _ in 0..num_traits {
            let trait_idx = rng.gen_range(0..all_traits.len());
            let t = all_traits[trait_idx];
            if !traits.iter().any(|existing| t.conflicts_with(existing)) {
                traits.push(t);
            }
        }

        // Generate practices
        let all_practices = [
            CulturalPractice::MonogamousMarriage,
            CulturalPractice::AncestorWorship,
            CulturalPractice::Burial,
            CulturalPractice::OralTradition,
            CulturalPractice::HospitalityCode,
            CulturalPractice::RitualFeasting,
        ];
        let mut practices = Vec::new();
        for practice in &all_practices {
            if rng.gen_bool(0.4) {
                practices.push(*practice);
            }
        }

        Culture {
            id,
            name: name.clone(),
            parent_culture_id: None,
            origin_year,
            origin_location,
            traits,
            practices,
            language_family: name.clone(),
            language_name: name.clone(),
            primary_color,
            secondary_color,
            architectural_style: "Basic".to_string(),
            art_style: "Primitive".to_string(),
            music_style: "Folk".to_string(),
            valor: rng.gen_range(30..70),
            piety: rng.gen_range(30..70),
            learning: rng.gen_range(30..70),
            honor: rng.gen_range(30..70),
            wealth: rng.gen_range(30..70),
            tradition: rng.gen_range(40..80),
            prestige: 10.0,
            assimilation_resistance: 0.5,
            spread_rate: 1.0,
            population: 0,
            settlements: 0,
        }
    }

    pub fn calculate_total_combat_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.combat_modifier()).sum()
    }

    pub fn calculate_total_defense_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.defense_modifier()).sum()
    }

    pub fn calculate_total_production_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.production_modifier()).sum()
    }

    pub fn calculate_total_food_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.food_modifier()).sum()
    }

    pub fn calculate_total_trade_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.trade_modifier()).sum()
    }

    pub fn calculate_total_culture_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.culture_modifier()).sum()
    }

    pub fn calculate_total_research_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.research_modifier()).sum()
    }

    pub fn calculate_total_faith_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.faith_modifier()).sum()
    }

    pub fn calculate_total_expansion_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.expansion_modifier()).sum()
    }

    pub fn calculate_total_diplomacy_modifier(&self) -> f32 {
        self.traits.iter().map(|t| t.diplomacy_modifier()).sum()
    }

    /// Attempt to mutate a trait based on circumstances
    pub fn try_mutate_trait(&mut self, rng: &mut SmallRng) -> Option<(CulturalTrait, CulturalTrait)>
    {
        if rng.gen_bool(0.02) {
            // 2% chance per tick
            // Pick a random trait to potentially evolve
            let all_evolvable = [
                CulturalTrait::Urban,
                CulturalTrait::Sedentary,
                CulturalTrait::Mercantile,
                CulturalTrait::Progressive,
                CulturalTrait::Secular,
                CulturalTrait::Diplomatic,
            ];

            for new_trait in &all_evolvable {
                if self.traits.contains(new_trait) {
                    continue;
                }

                let prereqs = new_trait.can_evolve_from();
                for prereq in &prereqs {
                    if self.traits.contains(prereq)
                        && !self.traits.iter().any(|t| new_trait.conflicts_with(t))
                    {
                        if rng.gen_bool(0.3) {
                            let old = *prereq;
                            self.traits.retain(|t| t != prereq);
                            self.traits.push(*new_trait);
                            return Some((old, *new_trait));
                        }
                    }
                }
            }
        }
        None
    }

    /// Create a divergent child culture
    pub fn create_branch(
        &self,
        new_id: CultureId,
        new_name: String,
        current_year: i32,
        new_location: (u32, u32),
        rng: &mut SmallRng,
    ) -> Culture {
        let mut child = self.clone();
        child.id = new_id;
        child.name = new_name;
        child.parent_culture_id = Some(self.id);
        child.origin_year = current_year;
        child.origin_location = new_location;
        child.prestige = self.prestige * 0.5;
        child.population = 0;
        child.settlements = 0;

        // Mutate some traits
        if rng.gen_bool(0.5) && child.traits.len() > 1 {
            let idx = rng.gen_range(0..child.traits.len());
            child.traits.remove(idx);
        }

        // Slightly modify colors
        child.primary_color = (
            child.primary_color.0.wrapping_add(rng.gen_range(0..30)),
            child.primary_color.1.wrapping_add(rng.gen_range(0..30)),
            child.primary_color.2.wrapping_add(rng.gen_range(0..30)),
        );

        // Modify values slightly
        child.valor = (child.valor as i16 + rng.gen_range(-10..10)).clamp(0, 100) as u8;
        child.piety = (child.piety as i16 + rng.gen_range(-10..10)).clamp(0, 100) as u8;
        child.learning = (child.learning as i16 + rng.gen_range(-10..10)).clamp(0, 100) as u8;
        child.honor = (child.honor as i16 + rng.gen_range(-10..10)).clamp(0, 100) as u8;

        child
    }

    /// Calculate cultural similarity (0.0 to 1.0)
    pub fn similarity_to(&self, other: &Culture) -> f32 {
        let mut score: f32 = 0.0;
        let max_score: f32 = 10.0;

        // Shared traits
        for trait1 in &self.traits {
            if other.traits.contains(trait1) {
                score += 1.5;
            }
        }

        // Shared practices
        for practice in &self.practices {
            if other.practices.contains(practice) {
                score += 0.5;
            }
        }

        // Same language family
        if self.language_family == other.language_family {
            score += 2.0;
        }

        // Similar values
        let valor_diff = (self.valor as i16 - other.valor as i16).abs();
        let piety_diff = (self.piety as i16 - other.piety as i16).abs();
        if valor_diff < 20 {
            score += 0.5;
        }
        if piety_diff < 20 {
            score += 0.5;
        }

        // Direct lineage
        if self.parent_culture_id == Some(other.id) || other.parent_culture_id == Some(self.id) {
            score += 2.0;
        }

        (score / max_score).min(1.0)
    }
}

/// Manages all cultures in the simulation
pub struct CultureManager {
    pub cultures: HashMap<CultureId, Culture>,
    pub next_id: CultureId,
}

impl CultureManager {
    pub fn new() -> Self {
        CultureManager {
            cultures: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_culture(
        &mut self,
        name: String,
        origin_year: i32,
        origin_location: (u32, u32),
        rng: &mut SmallRng,
    ) -> CultureId {
        let id = self.next_id;
        self.next_id += 1;

        let culture = Culture::new(id, name, origin_year, origin_location, rng);
        self.cultures.insert(id, culture);
        id
    }

    pub fn get(&self, id: CultureId) -> Option<&Culture> {
        self.cultures.get(&id)
    }

    pub fn get_mut(&mut self, id: CultureId) -> Option<&mut Culture> {
        self.cultures.get_mut(&id)
    }

    /// Process cultural changes for a tick
    pub fn tick(&mut self, rng: &mut SmallRng) -> Vec<CultureEvent> {
        let mut events = Vec::new();
        let culture_ids: Vec<CultureId> = self.cultures.keys().cloned().collect();

        for id in culture_ids {
            if let Some(culture) = self.cultures.get_mut(&id) {
                // Try trait mutations
                if let Some((old, new)) = culture.try_mutate_trait(rng) {
                    events.push(CultureEvent::TraitMutation {
                        culture_id: id,
                        old_trait: old,
                        new_trait: new,
                    });
                }

                // Adjust prestige based on population
                culture.prestige = (culture.prestige * 0.99 + culture.population as f32 * 0.001)
                    .clamp(1.0, 1000.0);
            }
        }

        events
    }

    /// Get cultures that can spread to a location based on distance and influence
    pub fn get_influential_cultures(
        &self,
        location: (u32, u32),
        max_distance: f32,
    ) -> Vec<(CultureId, f32)> {
        let mut influences = Vec::new();

        for (id, culture) in &self.cultures {
            let dx = (culture.origin_location.0 as f32 - location.0 as f32).abs();
            let dy = (culture.origin_location.1 as f32 - location.1 as f32).abs();
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < max_distance {
                let influence =
                    (1.0 - distance / max_distance) * culture.prestige * culture.spread_rate;
                if influence > 0.1 {
                    influences.push((*id, influence));
                }
            }
        }

        influences.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        influences
    }
}

/// Events that occur during cultural simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CultureEvent {
    TraitMutation {
        culture_id: CultureId,
        old_trait: CulturalTrait,
        new_trait: CulturalTrait,
    },
    CultureBranch {
        parent_id: CultureId,
        child_id: CultureId,
        child_name: String,
    },
    CultureExtinct {
        culture_id: CultureId,
        culture_name: String,
    },
    Assimilation {
        absorbed_id: CultureId,
        absorbing_id: CultureId,
        population: u32,
    },
}
