use crate::civilization::{NationId, SettlementId};
use crate::culture::{CultureEvent, CultureId};
use crate::military::{BattleId, MilitaryEvent};
use crate::politics::PoliticalEvent;
use crate::religion::{ReligionEvent, ReligionId};
use crate::technology::TechnologyId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of historical events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HistoricalEvent {
    // World events
    WorldCreated { year: i32, seed: u32 },
    YearPassed { year: i32 },
    EraChanged { year: i32, era_name: String },

    // Population events
    PopulationMilestone { year: i32, total_population: u64 },
    FirstSettlement { year: i32, settlement_name: String, location: (u32, u32) },
    SettlementFounded { year: i32, settlement_id: SettlementId, name: String, nation_id: NationId },
    SettlementDestroyed { year: i32, settlement_id: SettlementId, name: String, destroyer: Option<NationId> },
    SettlementGrew { year: i32, settlement_id: SettlementId, name: String, new_type: String },

    // Cultural events
    CultureFounded { year: i32, culture_id: CultureId, name: String, location: (u32, u32) },
    CultureMutation { year: i32, culture_id: CultureId, description: String },
    CultureSpread { year: i32, culture_id: CultureId, to_settlement: SettlementId },
    CultureExtinct { year: i32, culture_id: CultureId, name: String },

    // Religious events
    ReligionFounded { year: i32, religion_id: ReligionId, name: String, founder: Option<String> },
    ReligionSchism { year: i32, parent_id: ReligionId, child_id: ReligionId, child_name: String },
    MassConversion { year: i32, religion_id: ReligionId, nation_id: NationId, population: u32 },
    HolyCityEstablished { year: i32, religion_id: ReligionId, settlement_id: SettlementId },
    ReligiousPersecution { year: i32, nation_id: NationId, religion_id: ReligionId },

    // Political events
    NationFounded { year: i32, nation_id: NationId, name: String, founder: String },
    NationDestroyed { year: i32, nation_id: NationId, name: String, destroyer: Option<NationId> },
    LeaderRise { year: i32, nation_id: NationId, leader_name: String, title: String },
    LeaderDeath { year: i32, nation_id: NationId, leader_name: String, cause: String },
    GovernmentChange { year: i32, nation_id: NationId, old_type: String, new_type: String },
    CapitalMoved { year: i32, nation_id: NationId, new_capital: SettlementId },
    AllianceFormed { year: i32, nation_a: NationId, nation_b: NationId },
    AllianceBroken { year: i32, nation_a: NationId, nation_b: NationId },
    Vassalization { year: i32, overlord: NationId, vassal: NationId },
    Independence { year: i32, nation_id: NationId, from_nation: NationId },
    Revolution { year: i32, nation_id: NationId, rebel_type: String },
    Unification { year: i32, nation_a: NationId, nation_b: NationId, new_nation: NationId },

    // Military events
    WarDeclared { year: i32, attacker: NationId, defender: NationId, casus_belli: String },
    WarEnded { year: i32, winner: Option<NationId>, loser: Option<NationId>, treaty_name: String },
    BattleFought { year: i32, battle_id: BattleId, location: (u32, u32), winner: Option<NationId>, casualties: u32 },
    SiegeBegan { year: i32, settlement_id: SettlementId, attacker: NationId },
    SiegeEnded { year: i32, settlement_id: SettlementId, captured: bool },
    TerritoryConquered { year: i32, conqueror: NationId, from_nation: Option<NationId>, tiles: u32 },
    TerritoryLost { year: i32, nation_id: NationId, tiles: u32 },

    // Technology events
    TechnologyDiscovered { year: i32, nation_id: NationId, tech_id: TechnologyId, tech_name: String },
    TechnologySpread { year: i32, tech_id: TechnologyId, from_nation: NationId, to_nation: NationId },
    WonderBuilt { year: i32, nation_id: NationId, wonder_name: String, settlement_id: SettlementId },

    // Economic events
    TradeRouteEstablished { year: i32, settlement_a: SettlementId, settlement_b: SettlementId },
    GoldDiscovery { year: i32, settlement_id: SettlementId },
    Famine { year: i32, nation_id: NationId, deaths: u32 },
    Plague { year: i32, affected_nations: Vec<NationId>, deaths: u64 },
    EconomicBoom { year: i32, nation_id: NationId },
    EconomicCrisis { year: i32, nation_id: NationId },

    // Natural events
    NaturalDisaster { year: i32, location: (u32, u32), disaster_type: String, deaths: u32 },
    ClimateChange { year: i32, description: String },
}

impl HistoricalEvent {
    pub fn year(&self) -> i32 {
        match self {
            HistoricalEvent::WorldCreated { year, .. } => *year,
            HistoricalEvent::YearPassed { year } => *year,
            HistoricalEvent::EraChanged { year, .. } => *year,
            HistoricalEvent::PopulationMilestone { year, .. } => *year,
            HistoricalEvent::FirstSettlement { year, .. } => *year,
            HistoricalEvent::SettlementFounded { year, .. } => *year,
            HistoricalEvent::SettlementDestroyed { year, .. } => *year,
            HistoricalEvent::SettlementGrew { year, .. } => *year,
            HistoricalEvent::CultureFounded { year, .. } => *year,
            HistoricalEvent::CultureMutation { year, .. } => *year,
            HistoricalEvent::CultureSpread { year, .. } => *year,
            HistoricalEvent::CultureExtinct { year, .. } => *year,
            HistoricalEvent::ReligionFounded { year, .. } => *year,
            HistoricalEvent::ReligionSchism { year, .. } => *year,
            HistoricalEvent::MassConversion { year, .. } => *year,
            HistoricalEvent::HolyCityEstablished { year, .. } => *year,
            HistoricalEvent::ReligiousPersecution { year, .. } => *year,
            HistoricalEvent::NationFounded { year, .. } => *year,
            HistoricalEvent::NationDestroyed { year, .. } => *year,
            HistoricalEvent::LeaderRise { year, .. } => *year,
            HistoricalEvent::LeaderDeath { year, .. } => *year,
            HistoricalEvent::GovernmentChange { year, .. } => *year,
            HistoricalEvent::CapitalMoved { year, .. } => *year,
            HistoricalEvent::AllianceFormed { year, .. } => *year,
            HistoricalEvent::AllianceBroken { year, .. } => *year,
            HistoricalEvent::Vassalization { year, .. } => *year,
            HistoricalEvent::Independence { year, .. } => *year,
            HistoricalEvent::Revolution { year, .. } => *year,
            HistoricalEvent::Unification { year, .. } => *year,
            HistoricalEvent::WarDeclared { year, .. } => *year,
            HistoricalEvent::WarEnded { year, .. } => *year,
            HistoricalEvent::BattleFought { year, .. } => *year,
            HistoricalEvent::SiegeBegan { year, .. } => *year,
            HistoricalEvent::SiegeEnded { year, .. } => *year,
            HistoricalEvent::TerritoryConquered { year, .. } => *year,
            HistoricalEvent::TerritoryLost { year, .. } => *year,
            HistoricalEvent::TechnologyDiscovered { year, .. } => *year,
            HistoricalEvent::TechnologySpread { year, .. } => *year,
            HistoricalEvent::WonderBuilt { year, .. } => *year,
            HistoricalEvent::TradeRouteEstablished { year, .. } => *year,
            HistoricalEvent::GoldDiscovery { year, .. } => *year,
            HistoricalEvent::Famine { year, .. } => *year,
            HistoricalEvent::Plague { year, .. } => *year,
            HistoricalEvent::EconomicBoom { year, .. } => *year,
            HistoricalEvent::EconomicCrisis { year, .. } => *year,
            HistoricalEvent::NaturalDisaster { year, .. } => *year,
            HistoricalEvent::ClimateChange { year, .. } => *year,
        }
    }

    pub fn importance(&self) -> u8 {
        match self {
            HistoricalEvent::WorldCreated { .. } => 10,
            HistoricalEvent::YearPassed { .. } => 0,
            HistoricalEvent::EraChanged { .. } => 9,
            HistoricalEvent::PopulationMilestone { .. } => 5,
            HistoricalEvent::FirstSettlement { .. } => 10,
            HistoricalEvent::SettlementFounded { .. } => 3,
            HistoricalEvent::SettlementDestroyed { .. } => 6,
            HistoricalEvent::SettlementGrew { .. } => 2,
            HistoricalEvent::CultureFounded { .. } => 7,
            HistoricalEvent::CultureMutation { .. } => 4,
            HistoricalEvent::CultureSpread { .. } => 2,
            HistoricalEvent::CultureExtinct { .. } => 6,
            HistoricalEvent::ReligionFounded { .. } => 8,
            HistoricalEvent::ReligionSchism { .. } => 7,
            HistoricalEvent::MassConversion { .. } => 6,
            HistoricalEvent::HolyCityEstablished { .. } => 5,
            HistoricalEvent::ReligiousPersecution { .. } => 5,
            HistoricalEvent::NationFounded { .. } => 8,
            HistoricalEvent::NationDestroyed { .. } => 8,
            HistoricalEvent::LeaderRise { .. } => 5,
            HistoricalEvent::LeaderDeath { .. } => 5,
            HistoricalEvent::GovernmentChange { .. } => 7,
            HistoricalEvent::CapitalMoved { .. } => 4,
            HistoricalEvent::AllianceFormed { .. } => 4,
            HistoricalEvent::AllianceBroken { .. } => 4,
            HistoricalEvent::Vassalization { .. } => 6,
            HistoricalEvent::Independence { .. } => 7,
            HistoricalEvent::Revolution { .. } => 8,
            HistoricalEvent::Unification { .. } => 8,
            HistoricalEvent::WarDeclared { .. } => 7,
            HistoricalEvent::WarEnded { .. } => 7,
            HistoricalEvent::BattleFought { .. } => 5,
            HistoricalEvent::SiegeBegan { .. } => 4,
            HistoricalEvent::SiegeEnded { .. } => 5,
            HistoricalEvent::TerritoryConquered { .. } => 5,
            HistoricalEvent::TerritoryLost { .. } => 5,
            HistoricalEvent::TechnologyDiscovered { .. } => 6,
            HistoricalEvent::TechnologySpread { .. } => 3,
            HistoricalEvent::WonderBuilt { .. } => 7,
            HistoricalEvent::TradeRouteEstablished { .. } => 3,
            HistoricalEvent::GoldDiscovery { .. } => 5,
            HistoricalEvent::Famine { .. } => 6,
            HistoricalEvent::Plague { .. } => 9,
            HistoricalEvent::EconomicBoom { .. } => 4,
            HistoricalEvent::EconomicCrisis { .. } => 5,
            HistoricalEvent::NaturalDisaster { .. } => 6,
            HistoricalEvent::ClimateChange { .. } => 7,
        }
    }

    pub fn description(&self) -> String {
        match self {
            HistoricalEvent::WorldCreated { year, seed } => {
                format!("The world was created in year {} (seed: {})", year, seed)
            }
            HistoricalEvent::YearPassed { year } => format!("Year {} passed", year),
            HistoricalEvent::EraChanged { year, era_name } => {
                format!("The {} began in year {}", era_name, year)
            }
            HistoricalEvent::PopulationMilestone { year, total_population } => {
                format!("World population reached {} in year {}", total_population, year)
            }
            HistoricalEvent::FirstSettlement { year, settlement_name, .. } => {
                format!("The first settlement, {}, was founded in year {}", settlement_name, year)
            }
            HistoricalEvent::SettlementFounded { year, name, .. } => {
                format!("{} was founded in year {}", name, year)
            }
            HistoricalEvent::SettlementDestroyed { year, name, .. } => {
                format!("{} was destroyed in year {}", name, year)
            }
            HistoricalEvent::CultureFounded { year, name, .. } => {
                format!("The {} culture emerged in year {}", name, year)
            }
            HistoricalEvent::ReligionFounded { year, name, founder, .. } => {
                if let Some(f) = founder {
                    format!("{} founded {} in year {}", f, name, year)
                } else {
                    format!("{} emerged in year {}", name, year)
                }
            }
            HistoricalEvent::NationFounded { year, name, founder, .. } => {
                format!("{} founded {} in year {}", founder, name, year)
            }
            HistoricalEvent::NationDestroyed { year, name, .. } => {
                format!("{} fell in year {}", name, year)
            }
            HistoricalEvent::WarDeclared { year, .. } => {
                format!("War was declared in year {}", year)
            }
            HistoricalEvent::WarEnded { year, treaty_name, .. } => {
                format!("The {} was signed in year {}", treaty_name, year)
            }
            HistoricalEvent::BattleFought { year, casualties, .. } => {
                format!("A battle was fought in year {} with {} casualties", year, casualties)
            }
            HistoricalEvent::TechnologyDiscovered { year, tech_name, .. } => {
                format!("{} was discovered in year {}", tech_name, year)
            }
            HistoricalEvent::Plague { year, deaths, .. } => {
                format!("A great plague killed {} people in year {}", deaths, year)
            }
            _ => format!("Event in year {}", self.year()),
        }
    }
}

/// Manages the historical record
pub struct HistoryManager {
    pub events: Vec<HistoricalEvent>,
    pub events_by_year: HashMap<i32, Vec<usize>>,
    pub events_by_nation: HashMap<NationId, Vec<usize>>,
    pub events_by_culture: HashMap<CultureId, Vec<usize>>,
    pub events_by_religion: HashMap<ReligionId, Vec<usize>>,
    pub total_events: usize,
    pub max_events: usize,
}

impl HistoryManager {
    pub fn new() -> Self {
        HistoryManager {
            events: Vec::new(),
            events_by_year: HashMap::new(),
            events_by_nation: HashMap::new(),
            events_by_culture: HashMap::new(),
            events_by_religion: HashMap::new(),
            total_events: 0,
            max_events: 100000,
        }
    }

    pub fn record(&mut self, event: HistoricalEvent) {
        if self.events.len() >= self.max_events {
            // Remove low-importance events to make room
            self.prune_events();
        }

        let index = self.events.len();
        let year = event.year();

        // Index by year
        self.events_by_year
            .entry(year)
            .or_insert_with(Vec::new)
            .push(index);

        // Index by nation if applicable
        match &event {
            HistoricalEvent::NationFounded { nation_id, .. }
            | HistoricalEvent::NationDestroyed { nation_id, .. }
            | HistoricalEvent::LeaderRise { nation_id, .. }
            | HistoricalEvent::LeaderDeath { nation_id, .. }
            | HistoricalEvent::GovernmentChange { nation_id, .. }
            | HistoricalEvent::TechnologyDiscovered { nation_id, .. }
            | HistoricalEvent::Famine { nation_id, .. }
            | HistoricalEvent::EconomicBoom { nation_id, .. }
            | HistoricalEvent::EconomicCrisis { nation_id, .. } => {
                self.events_by_nation
                    .entry(*nation_id)
                    .or_insert_with(Vec::new)
                    .push(index);
            }
            HistoricalEvent::WarDeclared {
                attacker, defender, ..
            } => {
                self.events_by_nation
                    .entry(*attacker)
                    .or_insert_with(Vec::new)
                    .push(index);
                self.events_by_nation
                    .entry(*defender)
                    .or_insert_with(Vec::new)
                    .push(index);
            }
            _ => {}
        }

        // Index by culture if applicable
        match &event {
            HistoricalEvent::CultureFounded { culture_id, .. }
            | HistoricalEvent::CultureMutation { culture_id, .. }
            | HistoricalEvent::CultureSpread { culture_id, .. }
            | HistoricalEvent::CultureExtinct { culture_id, .. } => {
                self.events_by_culture
                    .entry(*culture_id)
                    .or_insert_with(Vec::new)
                    .push(index);
            }
            _ => {}
        }

        // Index by religion if applicable
        match &event {
            HistoricalEvent::ReligionFounded { religion_id, .. }
            | HistoricalEvent::ReligionSchism { parent_id: religion_id, .. }
            | HistoricalEvent::MassConversion { religion_id, .. }
            | HistoricalEvent::HolyCityEstablished { religion_id, .. }
            | HistoricalEvent::ReligiousPersecution { religion_id, .. } => {
                self.events_by_religion
                    .entry(*religion_id)
                    .or_insert_with(Vec::new)
                    .push(index);
            }
            _ => {}
        }

        self.events.push(event);
        self.total_events += 1;
    }

    fn prune_events(&mut self) {
        // Remove events with importance <= 2
        let mut kept_events = Vec::new();
        let mut index_map: HashMap<usize, usize> = HashMap::new();

        for (old_idx, event) in self.events.iter().enumerate() {
            if event.importance() > 2 {
                let new_idx = kept_events.len();
                index_map.insert(old_idx, new_idx);
                kept_events.push(event.clone());
            }
        }

        // Rebuild indices
        self.events = kept_events;
        self.events_by_year.clear();
        self.events_by_nation.clear();
        self.events_by_culture.clear();
        self.events_by_religion.clear();

        for (idx, event) in self.events.iter().enumerate() {
            let year = event.year();
            self.events_by_year
                .entry(year)
                .or_insert_with(Vec::new)
                .push(idx);
        }
    }

    pub fn get_events_in_year(&self, year: i32) -> Vec<&HistoricalEvent> {
        self.events_by_year
            .get(&year)
            .map(|indices| indices.iter().map(|&i| &self.events[i]).collect())
            .unwrap_or_default()
    }

    pub fn get_events_for_nation(&self, nation_id: NationId) -> Vec<&HistoricalEvent> {
        self.events_by_nation
            .get(&nation_id)
            .map(|indices| indices.iter().map(|&i| &self.events[i]).collect())
            .unwrap_or_default()
    }

    pub fn get_recent_events(&self, count: usize) -> Vec<&HistoricalEvent> {
        self.events.iter().rev().take(count).collect()
    }

    pub fn get_important_events(&self, min_importance: u8) -> Vec<&HistoricalEvent> {
        self.events
            .iter()
            .filter(|e| e.importance() >= min_importance)
            .collect()
    }

    pub fn get_timeline(&self, start_year: i32, end_year: i32) -> Vec<&HistoricalEvent> {
        let mut timeline = Vec::new();
        for year in start_year..=end_year {
            if let Some(indices) = self.events_by_year.get(&year) {
                for &idx in indices {
                    timeline.push(&self.events[idx]);
                }
            }
        }
        timeline
    }
}
