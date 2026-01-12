use crate::culture::Culture;
use crate::religion::Religion;
use crate::technology::TechnologyId;
use crate::world::ResourceType;
use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Unique identifier for settlements
pub type SettlementId = u64;
/// Unique identifier for nations
pub type NationId = u64;
/// Unique identifier for population groups
pub type PopulationId = u64;

/// Different types of settlements with varying characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettlementType {
    Camp,           // Nomadic or temporary
    Village,        // Small agricultural community (50-500)
    Town,           // Growing settlement (500-5000)
    City,           // Major urban center (5000-50000)
    Metropolis,     // Great city (50000+)
    Capital,        // National capital (special status)
}

impl SettlementType {
    pub fn from_population(pop: u32, is_capital: bool) -> Self {
        if is_capital {
            SettlementType::Capital
        } else if pop >= 50000 {
            SettlementType::Metropolis
        } else if pop >= 5000 {
            SettlementType::City
        } else if pop >= 500 {
            SettlementType::Town
        } else if pop >= 50 {
            SettlementType::Village
        } else {
            SettlementType::Camp
        }
    }

    pub fn defense_bonus(&self) -> f32 {
        match self {
            SettlementType::Camp => 0.0,
            SettlementType::Village => 0.1,
            SettlementType::Town => 0.25,
            SettlementType::City => 0.5,
            SettlementType::Metropolis => 0.75,
            SettlementType::Capital => 1.0,
        }
    }

    pub fn max_buildings(&self) -> u32 {
        match self {
            SettlementType::Camp => 2,
            SettlementType::Village => 5,
            SettlementType::Town => 15,
            SettlementType::City => 40,
            SettlementType::Metropolis => 80,
            SettlementType::Capital => 100,
        }
    }
}

/// A building that can be constructed in a settlement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingType {
    // Basic
    Farm,
    Mine,
    Workshop,
    Market,
    Granary,
    Well,

    // Military
    Barracks,
    Walls,
    Fortress,
    Arsenal,
    Stables,

    // Religious
    Shrine,
    Temple,
    Cathedral,
    Monastery,

    // Cultural
    Library,
    Theater,
    Arena,
    Monument,
    University,

    // Administrative
    CourtHouse,
    Palace,
    Treasury,

    // Economic
    Harbor,
    TradePost,
    Bank,
    Warehouse,
    Aqueduct,

    // Industrial
    Forge,
    Tannery,
    Mill,
    Quarry,
}

impl BuildingType {
    pub fn production_bonus(&self) -> f32 {
        match self {
            BuildingType::Farm => 0.2,
            BuildingType::Mine => 0.15,
            BuildingType::Workshop => 0.1,
            BuildingType::Mill => 0.15,
            BuildingType::Quarry => 0.1,
            BuildingType::Forge => 0.2,
            _ => 0.0,
        }
    }

    pub fn food_bonus(&self) -> f32 {
        match self {
            BuildingType::Farm => 0.3,
            BuildingType::Granary => 0.2,
            BuildingType::Mill => 0.1,
            BuildingType::Aqueduct => 0.15,
            BuildingType::Well => 0.05,
            _ => 0.0,
        }
    }

    pub fn gold_bonus(&self) -> f32 {
        match self {
            BuildingType::Market => 0.2,
            BuildingType::TradePost => 0.15,
            BuildingType::Bank => 0.3,
            BuildingType::Harbor => 0.25,
            _ => 0.0,
        }
    }

    pub fn research_bonus(&self) -> f32 {
        match self {
            BuildingType::Library => 0.2,
            BuildingType::University => 0.5,
            BuildingType::Monastery => 0.15,
            _ => 0.0,
        }
    }

    pub fn culture_bonus(&self) -> f32 {
        match self {
            BuildingType::Theater => 0.2,
            BuildingType::Arena => 0.15,
            BuildingType::Monument => 0.3,
            BuildingType::Temple => 0.1,
            BuildingType::Cathedral => 0.3,
            _ => 0.0,
        }
    }

    pub fn faith_bonus(&self) -> f32 {
        match self {
            BuildingType::Shrine => 0.1,
            BuildingType::Temple => 0.25,
            BuildingType::Cathedral => 0.5,
            BuildingType::Monastery => 0.3,
            _ => 0.0,
        }
    }

    pub fn defense_bonus(&self) -> f32 {
        match self {
            BuildingType::Walls => 0.3,
            BuildingType::Fortress => 0.5,
            BuildingType::Barracks => 0.1,
            _ => 0.0,
        }
    }

    pub fn build_cost(&self) -> u32 {
        match self {
            BuildingType::Farm => 50,
            BuildingType::Mine => 80,
            BuildingType::Workshop => 60,
            BuildingType::Market => 100,
            BuildingType::Granary => 70,
            BuildingType::Well => 30,
            BuildingType::Barracks => 120,
            BuildingType::Walls => 200,
            BuildingType::Fortress => 400,
            BuildingType::Arsenal => 250,
            BuildingType::Stables => 150,
            BuildingType::Shrine => 40,
            BuildingType::Temple => 150,
            BuildingType::Cathedral => 500,
            BuildingType::Monastery => 200,
            BuildingType::Library => 180,
            BuildingType::Theater => 200,
            BuildingType::Arena => 300,
            BuildingType::Monument => 400,
            BuildingType::University => 500,
            BuildingType::CourtHouse => 150,
            BuildingType::Palace => 800,
            BuildingType::Treasury => 300,
            BuildingType::Harbor => 250,
            BuildingType::TradePost => 100,
            BuildingType::Bank => 400,
            BuildingType::Warehouse => 120,
            BuildingType::Aqueduct => 300,
            BuildingType::Forge => 150,
            BuildingType::Tannery => 80,
            BuildingType::Mill => 100,
            BuildingType::Quarry => 90,
        }
    }
}

/// A settlement (city, town, village, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub id: SettlementId,
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub population: u32,
    pub settlement_type: SettlementType,
    pub owner_nation_id: Option<NationId>,
    pub founded_year: i32,

    // Resources and production
    pub food_stored: f32,
    pub gold: f32,
    pub production_points: f32,
    pub research_points: f32,
    pub culture_points: f32,
    pub faith_points: f32,

    // Buildings
    pub buildings: Vec<BuildingType>,
    pub building_in_progress: Option<BuildingType>,
    pub building_progress: f32,

    // Culture and religion
    pub primary_culture_id: Option<u64>,
    pub primary_religion_id: Option<u64>,
    pub culture_influence: HashMap<u64, f32>,
    pub religion_influence: HashMap<u64, f32>,

    // Military
    pub garrison_size: u32,
    pub fortification_level: f32,
    pub is_under_siege: bool,

    // Trade
    pub trade_routes: Vec<SettlementId>,
    pub available_resources: HashMap<ResourceType, u32>,

    // Growth
    pub growth_rate: f32,
    pub happiness: f32,
    pub health: f32,
}

impl Settlement {
    pub fn new(
        id: SettlementId,
        name: String,
        x: u32,
        y: u32,
        founded_year: i32,
    ) -> Self {
        Settlement {
            id,
            name,
            x,
            y,
            population: 100,
            settlement_type: SettlementType::Village,
            owner_nation_id: None,
            founded_year,
            food_stored: 100.0,
            gold: 0.0,
            production_points: 0.0,
            research_points: 0.0,
            culture_points: 0.0,
            faith_points: 0.0,
            buildings: Vec::new(),
            building_in_progress: None,
            building_progress: 0.0,
            primary_culture_id: None,
            primary_religion_id: None,
            culture_influence: HashMap::new(),
            religion_influence: HashMap::new(),
            garrison_size: 0,
            fortification_level: 0.0,
            is_under_siege: false,
            trade_routes: Vec::new(),
            available_resources: HashMap::new(),
            growth_rate: 0.01,
            happiness: 0.7,
            health: 0.8,
        }
    }

    pub fn update_type(&mut self, is_capital: bool) {
        self.settlement_type = SettlementType::from_population(self.population, is_capital);
    }

    pub fn calculate_food_production(&self, base_fertility: f32) -> f32 {
        let mut food = base_fertility * 10.0;
        for building in &self.buildings {
            food *= 1.0 + building.food_bonus();
        }
        food
    }

    pub fn calculate_gold_production(&self) -> f32 {
        let base = (self.population as f32 / 100.0) * 0.5;
        let mut gold = base;
        for building in &self.buildings {
            gold *= 1.0 + building.gold_bonus();
        }
        gold
    }

    pub fn calculate_production(&self) -> f32 {
        let base = (self.population as f32 / 200.0) * 0.5;
        let mut prod = base;
        for building in &self.buildings {
            prod *= 1.0 + building.production_bonus();
        }
        prod
    }

    pub fn calculate_research(&self) -> f32 {
        let base = (self.population as f32 / 500.0) * 0.2;
        let mut research = base;
        for building in &self.buildings {
            research *= 1.0 + building.research_bonus();
        }
        research
    }

    pub fn calculate_culture(&self) -> f32 {
        let base = (self.population as f32 / 300.0) * 0.3;
        let mut culture = base;
        for building in &self.buildings {
            culture *= 1.0 + building.culture_bonus();
        }
        culture
    }

    pub fn calculate_faith(&self) -> f32 {
        let base = (self.population as f32 / 400.0) * 0.2;
        let mut faith = base;
        for building in &self.buildings {
            faith *= 1.0 + building.faith_bonus();
        }
        faith
    }

    pub fn calculate_defense(&self) -> f32 {
        let base = self.settlement_type.defense_bonus();
        let mut defense = base;
        for building in &self.buildings {
            defense += building.defense_bonus();
        }
        defense += self.fortification_level * 0.5;
        defense += (self.garrison_size as f32 / 100.0) * 0.2;
        defense.min(3.0)
    }

    pub fn can_build(&self, building: BuildingType) -> bool {
        !self.buildings.contains(&building)
            && self.buildings.len() < self.settlement_type.max_buildings() as usize
    }

    pub fn tick(&mut self, base_fertility: f32) {
        // Food production and consumption
        let food_produced = self.calculate_food_production(base_fertility);
        let food_consumed = self.population as f32 * 0.1;
        self.food_stored += food_produced - food_consumed;

        // Population growth/decline
        if self.food_stored > 0.0 {
            let growth = self.population as f32 * self.growth_rate * self.happiness * self.health;
            self.population = (self.population as f32 + growth).max(10.0) as u32;
        } else {
            // Starvation
            let deaths = (self.population as f32 * 0.05) as u32;
            self.population = self.population.saturating_sub(deaths);
            self.happiness *= 0.9;
        }

        // Cap food storage
        let max_food = self.population as f32 * 2.0;
        self.food_stored = self.food_stored.clamp(-max_food, max_food * 5.0);

        // Resource production
        self.gold += self.calculate_gold_production();
        self.production_points += self.calculate_production();
        self.research_points += self.calculate_research();
        self.culture_points += self.calculate_culture();
        self.faith_points += self.calculate_faith();

        // Building progress
        if let Some(building) = self.building_in_progress {
            self.building_progress += self.production_points * 0.5;
            let cost = building.build_cost() as f32;
            if self.building_progress >= cost {
                self.buildings.push(building);
                self.building_in_progress = None;
                self.building_progress = 0.0;
            }
        }

        // Update settlement type
        self.update_type(self.settlement_type == SettlementType::Capital);

        // Slowly recover happiness
        self.happiness = (self.happiness + 0.01).min(1.0);
    }
}

/// Ethnic or tribal population group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub id: PopulationId,
    pub name: String,
    pub size: u32,
    pub location: (u32, u32),
    pub culture_id: u64,
    pub religion_id: Option<u64>,
    pub is_nomadic: bool,
    pub migration_target: Option<(u32, u32)>,
    pub skills: PopulationSkills,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PopulationSkills {
    pub farming: f32,
    pub herding: f32,
    pub hunting: f32,
    pub fishing: f32,
    pub mining: f32,
    pub crafting: f32,
    pub trading: f32,
    pub warfare: f32,
    pub sailing: f32,
}

/// Social classes within a nation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SocialClass {
    Slaves,
    Serfs,
    Peasants,
    Artisans,
    Merchants,
    Soldiers,
    Priests,
    Nobles,
    Royalty,
}

impl SocialClass {
    pub fn tax_rate(&self) -> f32 {
        match self {
            SocialClass::Slaves => 0.0,
            SocialClass::Serfs => 0.6,
            SocialClass::Peasants => 0.3,
            SocialClass::Artisans => 0.2,
            SocialClass::Merchants => 0.15,
            SocialClass::Soldiers => 0.0,
            SocialClass::Priests => 0.05,
            SocialClass::Nobles => 0.1,
            SocialClass::Royalty => 0.0,
        }
    }

    pub fn revolt_tendency(&self) -> f32 {
        match self {
            SocialClass::Slaves => 0.8,
            SocialClass::Serfs => 0.6,
            SocialClass::Peasants => 0.4,
            SocialClass::Artisans => 0.3,
            SocialClass::Merchants => 0.2,
            SocialClass::Soldiers => 0.3,
            SocialClass::Priests => 0.1,
            SocialClass::Nobles => 0.4,
            SocialClass::Royalty => 0.5,
        }
    }
}

/// A notable figure (ruler, general, prophet, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leader {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub birth_year: i32,
    pub death_year: Option<i32>,
    pub nation_id: Option<NationId>,

    // Attributes (0-100)
    pub martial: u8,
    pub diplomacy: u8,
    pub stewardship: u8,
    pub intrigue: u8,
    pub learning: u8,
    pub piety: u8,

    // Traits
    pub traits: Vec<LeaderTrait>,

    // Dynasty
    pub dynasty_name: String,
    pub parent_id: Option<u64>,
    pub spouse_id: Option<u64>,
    pub children_ids: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaderTrait {
    Ambitious,
    Content,
    Brave,
    Craven,
    Cruel,
    Kind,
    Genius,
    Imbecile,
    Strong,
    Weak,
    Charismatic,
    Shy,
    Just,
    Arbitrary,
    Zealous,
    Cynical,
    Diligent,
    Slothful,
    Greedy,
    Charitable,
    Wrathful,
    Patient,
    Proud,
    Humble,
    Deceitful,
    Honest,
}

impl LeaderTrait {
    pub fn martial_modifier(&self) -> i8 {
        match self {
            LeaderTrait::Brave => 2,
            LeaderTrait::Craven => -2,
            LeaderTrait::Strong => 2,
            LeaderTrait::Weak => -2,
            LeaderTrait::Wrathful => 1,
            _ => 0,
        }
    }

    pub fn diplomacy_modifier(&self) -> i8 {
        match self {
            LeaderTrait::Charismatic => 3,
            LeaderTrait::Shy => -2,
            LeaderTrait::Kind => 2,
            LeaderTrait::Cruel => -2,
            LeaderTrait::Honest => 1,
            LeaderTrait::Deceitful => -1,
            _ => 0,
        }
    }

    pub fn stewardship_modifier(&self) -> i8 {
        match self {
            LeaderTrait::Diligent => 2,
            LeaderTrait::Slothful => -3,
            LeaderTrait::Greedy => 1,
            LeaderTrait::Genius => 2,
            LeaderTrait::Imbecile => -3,
            _ => 0,
        }
    }
}

impl Leader {
    pub fn generate(rng: &mut SmallRng, id: u64, birth_year: i32, culture_name: &str) -> Self {
        let name = crate::names::generate_leader_name(rng, culture_name);
        let dynasty = crate::names::generate_dynasty_name(rng, culture_name);

        let mut traits = Vec::new();
        let all_traits = [
            LeaderTrait::Ambitious,
            LeaderTrait::Brave,
            LeaderTrait::Cruel,
            LeaderTrait::Kind,
            LeaderTrait::Genius,
            LeaderTrait::Strong,
            LeaderTrait::Charismatic,
            LeaderTrait::Just,
            LeaderTrait::Zealous,
            LeaderTrait::Diligent,
            LeaderTrait::Greedy,
            LeaderTrait::Wrathful,
            LeaderTrait::Proud,
            LeaderTrait::Honest,
        ];

        let num_traits = rng.gen_range(2..5);
        for _ in 0..num_traits {
            let trait_idx = rng.gen_range(0..all_traits.len());
            let t = all_traits[trait_idx];
            if !traits.contains(&t) {
                traits.push(t);
            }
        }

        Leader {
            id,
            name,
            title: "Chief".to_string(),
            birth_year,
            death_year: None,
            nation_id: None,
            martial: rng.gen_range(20..80),
            diplomacy: rng.gen_range(20..80),
            stewardship: rng.gen_range(20..80),
            intrigue: rng.gen_range(20..80),
            learning: rng.gen_range(20..80),
            piety: rng.gen_range(20..80),
            traits,
            dynasty_name: dynasty,
            parent_id: None,
            spouse_id: None,
            children_ids: Vec::new(),
        }
    }

    pub fn effective_martial(&self) -> u8 {
        let modifier: i8 = self.traits.iter().map(|t| t.martial_modifier()).sum();
        (self.martial as i16 + modifier as i16).clamp(0, 100) as u8
    }

    pub fn effective_diplomacy(&self) -> u8 {
        let modifier: i8 = self.traits.iter().map(|t| t.diplomacy_modifier()).sum();
        (self.diplomacy as i16 + modifier as i16).clamp(0, 100) as u8
    }

    pub fn effective_stewardship(&self) -> u8 {
        let modifier: i8 = self.traits.iter().map(|t| t.stewardship_modifier()).sum();
        (self.stewardship as i16 + modifier as i16).clamp(0, 100) as u8
    }

    pub fn is_alive(&self, current_year: i32) -> bool {
        self.death_year.is_none() || self.death_year.unwrap() > current_year
    }

    pub fn age(&self, current_year: i32) -> i32 {
        current_year - self.birth_year
    }
}
