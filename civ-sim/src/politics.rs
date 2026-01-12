use crate::civilization::{Leader, NationId, Settlement, SettlementId};
use crate::culture::CultureId;
use crate::religion::ReligionId;
use crate::technology::{ResearchState, TechTree, TechnologyId};
use crate::world::ResourceType;
use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Types of government systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GovernmentType {
    // Early governments
    Tribal,         // Chieftain-led
    Chiefdom,       // Hereditary chief

    // Monarchies
    Monarchy,       // King with nobles
    Despotism,      // Absolute ruler
    Theocracy,      // Religious rule

    // Republics
    Republic,       // Elected leaders
    Oligarchy,      // Rule by few elites
    Democracy,      // Direct democracy

    // Special
    Feudal,         // Decentralized lords
    Empire,         // Multi-ethnic conquest state
    MerchantRepublic, // Trade-focused
    Nomadic,        // Tribal confederation
    CityState,      // Independent city
}

impl GovernmentType {
    pub fn stability_modifier(&self) -> f32 {
        match self {
            GovernmentType::Despotism => 0.8,
            GovernmentType::Theocracy => 1.1,
            GovernmentType::Monarchy => 1.0,
            GovernmentType::Republic => 0.9,
            GovernmentType::Democracy => 0.85,
            GovernmentType::Oligarchy => 0.95,
            GovernmentType::Feudal => 0.75,
            GovernmentType::Empire => 0.7,
            GovernmentType::Tribal => 1.0,
            GovernmentType::Chiefdom => 0.95,
            GovernmentType::MerchantRepublic => 0.9,
            GovernmentType::Nomadic => 0.85,
            GovernmentType::CityState => 1.1,
        }
    }

    pub fn admin_efficiency(&self) -> f32 {
        match self {
            GovernmentType::Despotism => 1.2,
            GovernmentType::Theocracy => 1.0,
            GovernmentType::Monarchy => 1.1,
            GovernmentType::Republic => 1.15,
            GovernmentType::Democracy => 1.0,
            GovernmentType::Oligarchy => 1.1,
            GovernmentType::Feudal => 0.7,
            GovernmentType::Empire => 0.9,
            GovernmentType::Tribal => 0.5,
            GovernmentType::Chiefdom => 0.6,
            GovernmentType::MerchantRepublic => 1.2,
            GovernmentType::Nomadic => 0.4,
            GovernmentType::CityState => 1.3,
        }
    }

    pub fn military_modifier(&self) -> f32 {
        match self {
            GovernmentType::Despotism => 1.2,
            GovernmentType::Theocracy => 0.9,
            GovernmentType::Monarchy => 1.1,
            GovernmentType::Republic => 1.0,
            GovernmentType::Democracy => 0.9,
            GovernmentType::Oligarchy => 1.0,
            GovernmentType::Feudal => 1.1,
            GovernmentType::Empire => 1.15,
            GovernmentType::Tribal => 0.8,
            GovernmentType::Chiefdom => 0.85,
            GovernmentType::MerchantRepublic => 0.8,
            GovernmentType::Nomadic => 1.2,
            GovernmentType::CityState => 0.9,
        }
    }

    pub fn tax_efficiency(&self) -> f32 {
        match self {
            GovernmentType::Despotism => 1.3,
            GovernmentType::Theocracy => 1.1,
            GovernmentType::Monarchy => 1.15,
            GovernmentType::Republic => 1.1,
            GovernmentType::Democracy => 1.0,
            GovernmentType::Oligarchy => 1.2,
            GovernmentType::Feudal => 0.7,
            GovernmentType::Empire => 1.1,
            GovernmentType::Tribal => 0.4,
            GovernmentType::Chiefdom => 0.5,
            GovernmentType::MerchantRepublic => 1.25,
            GovernmentType::Nomadic => 0.3,
            GovernmentType::CityState => 1.2,
        }
    }

    pub fn research_modifier(&self) -> f32 {
        match self {
            GovernmentType::Republic => 1.15,
            GovernmentType::Democracy => 1.1,
            GovernmentType::Theocracy => 0.8,
            GovernmentType::MerchantRepublic => 1.1,
            GovernmentType::CityState => 1.2,
            GovernmentType::Tribal => 0.6,
            GovernmentType::Nomadic => 0.5,
            _ => 1.0,
        }
    }

    pub fn happiness_modifier(&self) -> f32 {
        match self {
            GovernmentType::Democracy => 1.2,
            GovernmentType::Republic => 1.1,
            GovernmentType::Despotism => 0.7,
            GovernmentType::Theocracy => 1.0,
            GovernmentType::CityState => 1.1,
            GovernmentType::Tribal => 1.0,
            GovernmentType::Nomadic => 1.0,
            _ => 0.95,
        }
    }

    pub fn max_territory(&self) -> u32 {
        match self {
            GovernmentType::CityState => 50,
            GovernmentType::Tribal => 100,
            GovernmentType::Chiefdom => 200,
            GovernmentType::Monarchy => 500,
            GovernmentType::Republic => 400,
            GovernmentType::Feudal => 600,
            GovernmentType::Empire => 2000,
            GovernmentType::Despotism => 800,
            GovernmentType::Nomadic => 300,
            _ => 400,
        }
    }

    pub fn succession_type(&self) -> SuccessionType {
        match self {
            GovernmentType::Tribal => SuccessionType::Election,
            GovernmentType::Chiefdom => SuccessionType::Hereditary,
            GovernmentType::Monarchy => SuccessionType::Hereditary,
            GovernmentType::Despotism => SuccessionType::Hereditary,
            GovernmentType::Theocracy => SuccessionType::Election,
            GovernmentType::Republic => SuccessionType::Election,
            GovernmentType::Oligarchy => SuccessionType::Election,
            GovernmentType::Democracy => SuccessionType::Election,
            GovernmentType::Feudal => SuccessionType::Hereditary,
            GovernmentType::Empire => SuccessionType::Hereditary,
            GovernmentType::MerchantRepublic => SuccessionType::Election,
            GovernmentType::Nomadic => SuccessionType::Election,
            GovernmentType::CityState => SuccessionType::Election,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SuccessionType {
    Hereditary,     // Eldest child/relative
    Election,       // Elected by nobles/citizens
    Appointment,    // Appointed by predecessor
}

/// Diplomatic relations between nations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiplomaticStatus {
    Unknown,        // No contact
    Peace,          // Neutral
    NonAggression,  // Agreed not to fight
    OpenBorders,    // Can move through territory
    TradeAgreement, // Trading
    Alliance,       // Military alliance
    DefensivePact,  // Will defend if attacked
    War,            // At war
    Vassalage,      // Subject to another
    Federation,     // Closely allied states
    Truce,          // Temporary peace after war
}

/// Diplomatic relationship between two nations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticRelation {
    pub nation_a: NationId,
    pub nation_b: NationId,
    pub status: DiplomaticStatus,
    pub opinion: i32,           // -100 to 100
    pub trust: i32,             // -100 to 100
    pub treaties: Vec<Treaty>,
    pub started_year: i32,
    pub war_exhaustion_a: f32,
    pub war_exhaustion_b: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Treaty {
    pub treaty_type: TreatyType,
    pub signed_year: i32,
    pub expires_year: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreatyType {
    Peace,
    TradeAgreement,
    MilitaryAccess,
    DefensiveAlliance,
    Marriage,
    Tribute,
    Vassalage,
}

/// A nation/country/state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nation {
    pub id: NationId,
    pub name: String,
    pub adjective: String,
    pub founded_year: i32,
    pub government: GovernmentType,

    // Leadership
    pub current_leader_id: Option<u64>,
    pub ruling_dynasty: Option<String>,
    pub leaders_history: Vec<u64>,

    // Territory
    pub capital_id: Option<SettlementId>,
    pub controlled_tiles: HashSet<(u32, u32)>,
    pub settlements: Vec<SettlementId>,

    // Demographics
    pub total_population: u32,
    pub primary_culture_id: Option<CultureId>,
    pub cultures: HashMap<CultureId, u32>, // Culture -> population
    pub primary_religion_id: Option<ReligionId>,
    pub religions: HashMap<ReligionId, u32>, // Religion -> population

    // Technology
    pub research_state: ResearchState,

    // Economy
    pub treasury: f32,
    pub income: f32,
    pub expenses: f32,
    pub trade_income: f32,
    pub tax_rate: f32,
    pub resources: HashMap<ResourceType, u32>,

    // Military
    pub army_size: u32,
    pub navy_size: u32,
    pub military_strength: f32,
    pub war_exhaustion: f32,
    pub manpower: u32,
    pub manpower_pool: u32,

    // Diplomatic
    pub relations: HashMap<NationId, DiplomaticRelation>,
    pub at_war_with: HashSet<NationId>,
    pub allies: HashSet<NationId>,
    pub vassals: HashSet<NationId>,
    pub overlord: Option<NationId>,

    // Internal
    pub stability: f32,         // 0-1
    pub legitimacy: f32,        // 0-1
    pub corruption: f32,        // 0-1
    pub revolt_risk: f32,       // 0-1
    pub happiness: f32,         // 0-1

    // Culture/Faith
    pub national_ideas: Vec<NationalIdea>,
    pub culture_points: f32,
    pub faith_points: f32,

    // Visual
    pub primary_color: (u8, u8, u8),
    pub secondary_color: (u8, u8, u8),
    pub flag_symbol: String,

    // Flags
    pub is_player: bool,
    pub is_alive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NationalIdea {
    // Military
    MilitaryTradition,
    DefensiveWars,
    Expansionism,
    NavalPower,

    // Economy
    FreeTrade,
    Mercantilism,
    IndustrialFocus,
    AgrarianEconomy,

    // Culture
    PatronOfArts,
    ReligiousTolerance,
    StateReligion,
    NationalUnity,

    // Governance
    Bureaucracy,
    LocalAutonomy,
    Centralization,

    // Diplomacy
    DiplomaticCorps,
    Isolationism,
    Imperialism,
}

impl NationalIdea {
    pub fn effects(&self) -> Vec<(String, f32)> {
        match self {
            NationalIdea::MilitaryTradition => vec![
                ("combat".to_string(), 0.15),
                ("army_maintenance".to_string(), -0.1),
            ],
            NationalIdea::NavalPower => vec![
                ("naval_combat".to_string(), 0.2),
                ("trade".to_string(), 0.1),
            ],
            NationalIdea::FreeTrade => vec![
                ("trade".to_string(), 0.25),
                ("production".to_string(), 0.1),
            ],
            NationalIdea::PatronOfArts => vec![
                ("culture".to_string(), 0.3),
                ("prestige".to_string(), 0.2),
            ],
            NationalIdea::Bureaucracy => vec![
                ("admin_efficiency".to_string(), 0.2),
                ("corruption".to_string(), -0.1),
            ],
            _ => vec![],
        }
    }
}

impl Nation {
    pub fn new(
        id: NationId,
        name: String,
        founded_year: i32,
        capital_location: (u32, u32),
        rng: &mut SmallRng,
    ) -> Self {
        let primary_color = (rng.gen(), rng.gen(), rng.gen());
        let secondary_color = (rng.gen(), rng.gen(), rng.gen());

        let mut controlled_tiles = HashSet::new();
        controlled_tiles.insert(capital_location);

        Nation {
            id,
            name: name.clone(),
            adjective: format!("{}n", name),
            founded_year,
            government: GovernmentType::Tribal,
            current_leader_id: None,
            ruling_dynasty: None,
            leaders_history: Vec::new(),
            capital_id: None,
            controlled_tiles,
            settlements: Vec::new(),
            total_population: 0,
            primary_culture_id: None,
            cultures: HashMap::new(),
            primary_religion_id: None,
            religions: HashMap::new(),
            research_state: ResearchState::new(),
            treasury: 100.0,
            income: 0.0,
            expenses: 0.0,
            trade_income: 0.0,
            tax_rate: 0.2,
            resources: HashMap::new(),
            army_size: 0,
            navy_size: 0,
            military_strength: 0.0,
            war_exhaustion: 0.0,
            manpower: 0,
            manpower_pool: 0,
            relations: HashMap::new(),
            at_war_with: HashSet::new(),
            allies: HashSet::new(),
            vassals: HashSet::new(),
            overlord: None,
            stability: 0.7,
            legitimacy: 0.8,
            corruption: 0.1,
            revolt_risk: 0.0,
            happiness: 0.7,
            national_ideas: Vec::new(),
            culture_points: 0.0,
            faith_points: 0.0,
            primary_color,
            secondary_color,
            flag_symbol: "Shield".to_string(),
            is_player: false,
            is_alive: true,
        }
    }

    pub fn calculate_income(&self) -> f32 {
        let base_tax = self.total_population as f32 * 0.01 * self.tax_rate;
        let tax_efficiency = self.government.tax_efficiency();
        let admin = self.government.admin_efficiency();
        let corruption_penalty = 1.0 - self.corruption * 0.5;

        base_tax * tax_efficiency * admin * corruption_penalty + self.trade_income
    }

    pub fn calculate_expenses(&self) -> f32 {
        let army_cost = self.army_size as f32 * 0.5;
        let navy_cost = self.navy_size as f32 * 1.0;
        let admin_cost = self.settlements.len() as f32 * 2.0;

        army_cost + navy_cost + admin_cost
    }

    pub fn calculate_military_strength(&self) -> f32 {
        let base = self.army_size as f32 + self.navy_size as f32 * 0.5;
        let gov_modifier = self.government.military_modifier();
        let tech_modifier = 1.0 + self.research_state.calculate_bonuses(&TechTree::new()).combat;

        base * gov_modifier * tech_modifier * (1.0 - self.war_exhaustion)
    }

    pub fn calculate_revolt_risk(&self) -> f32 {
        let stability_factor = (1.0 - self.stability) * 0.3;
        let happiness_factor = (1.0 - self.happiness) * 0.3;
        let war_exhaustion_factor = self.war_exhaustion * 0.2;
        let corruption_factor = self.corruption * 0.1;
        let legitimacy_factor = (1.0 - self.legitimacy) * 0.2;

        (stability_factor + happiness_factor + war_exhaustion_factor +
         corruption_factor + legitimacy_factor).min(1.0)
    }

    pub fn can_declare_war(&self, target: &Nation) -> bool {
        if self.at_war_with.contains(&target.id) {
            return false;
        }
        if self.allies.contains(&target.id) {
            return false;
        }
        if self.stability < 0.3 {
            return false;
        }
        if self.war_exhaustion > 0.8 {
            return false;
        }
        true
    }

    pub fn declare_war(&mut self, target_id: NationId, current_year: i32) {
        self.at_war_with.insert(target_id);
        self.stability -= 0.1;

        // Update relations
        if let Some(relation) = self.relations.get_mut(&target_id) {
            relation.status = DiplomaticStatus::War;
            relation.opinion = -100;
            relation.started_year = current_year;
        } else {
            self.relations.insert(target_id, DiplomaticRelation {
                nation_a: self.id,
                nation_b: target_id,
                status: DiplomaticStatus::War,
                opinion: -100,
                trust: -100,
                treaties: Vec::new(),
                started_year: current_year,
                war_exhaustion_a: 0.0,
                war_exhaustion_b: 0.0,
            });
        }
    }

    pub fn make_peace(&mut self, target_id: NationId, current_year: i32) {
        self.at_war_with.remove(&target_id);

        if let Some(relation) = self.relations.get_mut(&target_id) {
            relation.status = DiplomaticStatus::Truce;
            relation.treaties.push(Treaty {
                treaty_type: TreatyType::Peace,
                signed_year: current_year,
                expires_year: Some(current_year + 10),
            });
        }
    }

    pub fn get_opinion(&self, other_id: NationId) -> i32 {
        self.relations.get(&other_id).map(|r| r.opinion).unwrap_or(0)
    }

    pub fn improve_relations(&mut self, other_id: NationId, amount: i32) {
        if let Some(relation) = self.relations.get_mut(&other_id) {
            relation.opinion = (relation.opinion + amount).clamp(-100, 100);
            if amount > 0 {
                relation.trust = (relation.trust + amount / 2).clamp(-100, 100);
            }
        }
    }

    pub fn tick(&mut self, tech_tree: &TechTree) {
        // Update economy
        self.income = self.calculate_income();
        self.expenses = self.calculate_expenses();
        self.treasury += self.income - self.expenses;

        // Update military
        self.military_strength = self.calculate_military_strength();
        self.manpower_pool = (self.total_population as f32 * 0.05) as u32;

        // Update stability
        if self.treasury < 0.0 {
            self.stability -= 0.02;
            self.happiness -= 0.01;
        }

        // Slow corruption growth/decay
        self.corruption = (self.corruption + 0.001 - self.government.admin_efficiency() * 0.002)
            .clamp(0.0, 1.0);

        // War exhaustion decay
        if self.at_war_with.is_empty() {
            self.war_exhaustion = (self.war_exhaustion - 0.02).max(0.0);
        }

        // Calculate revolt risk
        self.revolt_risk = self.calculate_revolt_risk();

        // Stability recovery
        if self.stability < 0.5 {
            self.stability = (self.stability + 0.01).min(1.0);
        }

        // Update happiness based on various factors
        let gov_happiness = self.government.happiness_modifier();
        let target_happiness = 0.5 * gov_happiness
            + 0.2 * (1.0 - self.corruption)
            + 0.2 * self.stability
            + 0.1 * self.legitimacy;
        self.happiness = self.happiness * 0.95 + target_happiness * 0.05;
    }

    pub fn add_territory(&mut self, tiles: &[(u32, u32)]) {
        for tile in tiles {
            self.controlled_tiles.insert(*tile);
        }
    }

    pub fn remove_territory(&mut self, tiles: &[(u32, u32)]) {
        for tile in tiles {
            self.controlled_tiles.remove(tile);
        }
    }

    pub fn change_government(&mut self, new_gov: GovernmentType) {
        self.government = new_gov;
        self.stability -= 0.3;
        self.legitimacy = 0.5;
    }

    pub fn succession_crisis(&mut self, rng: &mut SmallRng) {
        self.stability -= 0.2;
        self.legitimacy = 0.3;

        // Chance of civil war or government change
        if rng.gen_bool(0.3) {
            match self.government {
                GovernmentType::Monarchy => {
                    if rng.gen_bool(0.5) {
                        self.government = GovernmentType::Feudal;
                    }
                }
                GovernmentType::Empire => {
                    self.government = GovernmentType::Despotism;
                }
                _ => {}
            }
        }
    }
}

/// Manages all nations in the simulation
pub struct NationManager {
    pub nations: HashMap<NationId, Nation>,
    pub next_id: NationId,
}

impl NationManager {
    pub fn new() -> Self {
        NationManager {
            nations: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_nation(
        &mut self,
        name: String,
        founded_year: i32,
        capital_location: (u32, u32),
        rng: &mut SmallRng,
    ) -> NationId {
        let id = self.next_id;
        self.next_id += 1;

        let nation = Nation::new(id, name, founded_year, capital_location, rng);
        self.nations.insert(id, nation);
        id
    }

    pub fn get(&self, id: NationId) -> Option<&Nation> {
        self.nations.get(&id)
    }

    pub fn get_mut(&mut self, id: NationId) -> Option<&mut Nation> {
        self.nations.get_mut(&id)
    }

    pub fn get_all_alive(&self) -> Vec<&Nation> {
        self.nations.values().filter(|n| n.is_alive).collect()
    }

    pub fn tick(&mut self, tech_tree: &TechTree) {
        let ids: Vec<NationId> = self.nations.keys().cloned().collect();
        for id in ids {
            if let Some(nation) = self.nations.get_mut(&id) {
                if nation.is_alive {
                    nation.tick(tech_tree);

                    // Check for nation death
                    if nation.total_population == 0 || nation.controlled_tiles.is_empty() {
                        nation.is_alive = false;
                    }
                }
            }
        }
    }

    /// Get nations at war with each other
    pub fn get_wars(&self) -> Vec<(NationId, NationId)> {
        let mut wars = Vec::new();
        let mut seen = HashSet::new();

        for nation in self.nations.values() {
            for war_target in &nation.at_war_with {
                let pair = if nation.id < *war_target {
                    (nation.id, *war_target)
                } else {
                    (*war_target, nation.id)
                };

                if !seen.contains(&pair) {
                    seen.insert(pair);
                    wars.push(pair);
                }
            }
        }

        wars
    }
}

/// Events that occur in political simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoliticalEvent {
    NationFounded {
        nation_id: NationId,
        nation_name: String,
        founder_name: String,
    },
    NationDestroyed {
        nation_id: NationId,
        nation_name: String,
        destroyer_id: Option<NationId>,
    },
    WarDeclared {
        attacker_id: NationId,
        defender_id: NationId,
        casus_belli: String,
    },
    PeaceSigned {
        nation_a: NationId,
        nation_b: NationId,
        terms: String,
    },
    GovernmentChange {
        nation_id: NationId,
        old_government: GovernmentType,
        new_government: GovernmentType,
    },
    LeaderSuccession {
        nation_id: NationId,
        old_leader: String,
        new_leader: String,
    },
    Revolt {
        nation_id: NationId,
        rebel_type: String,
        severity: f32,
    },
    AllianceFormed {
        nation_a: NationId,
        nation_b: NationId,
    },
    TreatyBroken {
        nation_id: NationId,
        treaty_type: TreatyType,
        victim_id: NationId,
    },
    TerritoryGained {
        nation_id: NationId,
        tiles_gained: u32,
        from_nation: Option<NationId>,
    },
    TerritoryLost {
        nation_id: NationId,
        tiles_lost: u32,
        to_nation: Option<NationId>,
    },
}
