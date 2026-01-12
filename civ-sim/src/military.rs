use crate::civilization::{NationId, SettlementId};
use crate::technology::TechEra;
use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ArmyId = u64;
pub type BattleId = u64;

/// Types of military units
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitType {
    // Infantry
    Warriors,
    Spearmen,
    Swordsmen,
    Pikemen,
    Musketeers,
    Riflemen,
    Infantry,

    // Ranged
    Slingers,
    Archers,
    Crossbowmen,
    Skirmishers,

    // Cavalry
    LightCavalry,
    HeavyCavalry,
    Knights,
    HorseArchers,
    Dragoons,

    // Siege
    BatteringRam,
    Catapult,
    Trebuchet,
    Cannon,
    Artillery,

    // Special
    WarElephants,
    Chariots,
    Berserkers,
    Samurai,
    Janissaries,

    // Naval
    Galley,
    Trireme,
    Longship,
    Caravel,
    Galleon,
    FrigateShip,
    Ironclad,
}

impl UnitType {
    pub fn base_strength(&self) -> f32 {
        match self {
            UnitType::Warriors => 5.0,
            UnitType::Spearmen => 8.0,
            UnitType::Swordsmen => 12.0,
            UnitType::Pikemen => 14.0,
            UnitType::Musketeers => 20.0,
            UnitType::Riflemen => 30.0,
            UnitType::Infantry => 40.0,
            UnitType::Slingers => 4.0,
            UnitType::Archers => 7.0,
            UnitType::Crossbowmen => 12.0,
            UnitType::Skirmishers => 15.0,
            UnitType::LightCavalry => 10.0,
            UnitType::HeavyCavalry => 18.0,
            UnitType::Knights => 25.0,
            UnitType::HorseArchers => 14.0,
            UnitType::Dragoons => 28.0,
            UnitType::BatteringRam => 3.0,
            UnitType::Catapult => 8.0,
            UnitType::Trebuchet => 15.0,
            UnitType::Cannon => 25.0,
            UnitType::Artillery => 40.0,
            UnitType::WarElephants => 30.0,
            UnitType::Chariots => 12.0,
            UnitType::Berserkers => 15.0,
            UnitType::Samurai => 22.0,
            UnitType::Janissaries => 24.0,
            UnitType::Galley => 8.0,
            UnitType::Trireme => 12.0,
            UnitType::Longship => 15.0,
            UnitType::Caravel => 18.0,
            UnitType::Galleon => 30.0,
            UnitType::FrigateShip => 40.0,
            UnitType::Ironclad => 60.0,
        }
    }

    pub fn siege_bonus(&self) -> f32 {
        match self {
            UnitType::BatteringRam => 3.0,
            UnitType::Catapult => 2.0,
            UnitType::Trebuchet => 4.0,
            UnitType::Cannon => 5.0,
            UnitType::Artillery => 8.0,
            _ => 0.0,
        }
    }

    pub fn defense_bonus(&self) -> f32 {
        match self {
            UnitType::Spearmen => 0.3,
            UnitType::Pikemen => 0.5,
            UnitType::Swordsmen => 0.2,
            _ => 0.0,
        }
    }

    pub fn vs_cavalry_bonus(&self) -> f32 {
        match self {
            UnitType::Spearmen => 0.5,
            UnitType::Pikemen => 1.0,
            _ => 0.0,
        }
    }

    pub fn maintenance_cost(&self) -> f32 {
        self.base_strength() * 0.1
    }

    pub fn recruitment_cost(&self) -> f32 {
        self.base_strength() * 2.0
    }

    pub fn recruitment_time(&self) -> u32 {
        match self {
            UnitType::Warriors => 1,
            UnitType::Spearmen | UnitType::Archers => 2,
            UnitType::Swordsmen | UnitType::Crossbowmen => 3,
            UnitType::Knights | UnitType::HeavyCavalry => 5,
            UnitType::Cannon | UnitType::Artillery => 6,
            UnitType::Galleon | UnitType::FrigateShip => 8,
            _ => 3,
        }
    }

    pub fn is_naval(&self) -> bool {
        matches!(
            self,
            UnitType::Galley
                | UnitType::Trireme
                | UnitType::Longship
                | UnitType::Caravel
                | UnitType::Galleon
                | UnitType::FrigateShip
                | UnitType::Ironclad
        )
    }

    pub fn is_ranged(&self) -> bool {
        matches!(
            self,
            UnitType::Slingers
                | UnitType::Archers
                | UnitType::Crossbowmen
                | UnitType::Skirmishers
                | UnitType::HorseArchers
        )
    }

    pub fn is_cavalry(&self) -> bool {
        matches!(
            self,
            UnitType::LightCavalry
                | UnitType::HeavyCavalry
                | UnitType::Knights
                | UnitType::HorseArchers
                | UnitType::Dragoons
                | UnitType::Chariots
        )
    }

    pub fn is_siege(&self) -> bool {
        matches!(
            self,
            UnitType::BatteringRam
                | UnitType::Catapult
                | UnitType::Trebuchet
                | UnitType::Cannon
                | UnitType::Artillery
        )
    }

    pub fn available_era(&self) -> TechEra {
        match self {
            UnitType::Warriors | UnitType::Slingers => TechEra::StoneAge,
            UnitType::Spearmen | UnitType::Archers | UnitType::Chariots => TechEra::BronzeAge,
            UnitType::Swordsmen
            | UnitType::LightCavalry
            | UnitType::Galley
            | UnitType::Trireme
            | UnitType::Catapult
            | UnitType::BatteringRam => TechEra::IronAge,
            UnitType::HeavyCavalry
            | UnitType::Crossbowmen
            | UnitType::HorseArchers
            | UnitType::WarElephants
            | UnitType::Trebuchet => TechEra::ClassicalAge,
            UnitType::Knights
            | UnitType::Pikemen
            | UnitType::Longship
            | UnitType::Berserkers
            | UnitType::Samurai => TechEra::MedievalAge,
            UnitType::Musketeers
            | UnitType::Cannon
            | UnitType::Caravel
            | UnitType::Galleon
            | UnitType::Janissaries
            | UnitType::Skirmishers => TechEra::RenaissanceAge,
            UnitType::Riflemen
            | UnitType::Dragoons
            | UnitType::Artillery
            | UnitType::FrigateShip => TechEra::IndustrialAge,
            UnitType::Infantry | UnitType::Ironclad => TechEra::ModernAge,
        }
    }
}

/// A group of units of the same type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regiment {
    pub unit_type: UnitType,
    pub count: u32,
    pub experience: f32, // 0-1
    pub morale: f32,     // 0-1
    pub strength: f32,   // Current HP percentage
}

impl Regiment {
    pub fn new(unit_type: UnitType, count: u32) -> Self {
        Regiment {
            unit_type,
            count,
            experience: 0.0,
            morale: 1.0,
            strength: 1.0,
        }
    }

    pub fn effective_strength(&self) -> f32 {
        let base = self.unit_type.base_strength() * self.count as f32;
        let experience_bonus = 1.0 + self.experience * 0.5;
        let morale_factor = 0.5 + self.morale * 0.5;
        base * experience_bonus * morale_factor * self.strength
    }

    pub fn take_casualties(&mut self, percentage: f32) {
        let losses = (self.count as f32 * percentage) as u32;
        self.count = self.count.saturating_sub(losses);
        self.morale -= percentage * 0.3;
        self.morale = self.morale.max(0.0);
    }

    pub fn reinforce(&mut self, amount: u32) {
        self.count += amount;
        // New troops lower average experience
        self.experience *= self.count as f32 / (self.count + amount) as f32;
    }

    pub fn recover(&mut self) {
        self.strength = (self.strength + 0.1).min(1.0);
        self.morale = (self.morale + 0.05).min(1.0);
    }

    pub fn gain_experience(&mut self, amount: f32) {
        self.experience = (self.experience + amount).min(1.0);
    }
}

/// An army or fleet commanded by a general
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Army {
    pub id: ArmyId,
    pub name: String,
    pub owner_nation_id: NationId,
    pub commander_id: Option<u64>,
    pub position: (u32, u32),
    pub regiments: Vec<Regiment>,
    pub is_naval: bool,

    // State
    pub morale: f32,
    pub supply: f32,
    pub movement_points: f32,
    pub is_in_combat: bool,
    pub is_sieging: Option<SettlementId>,

    // Movement
    pub destination: Option<(u32, u32)>,
    pub path: Vec<(u32, u32)>,
}

impl Army {
    pub fn new(
        id: ArmyId,
        name: String,
        owner_nation_id: NationId,
        position: (u32, u32),
        is_naval: bool,
    ) -> Self {
        Army {
            id,
            name,
            owner_nation_id,
            commander_id: None,
            position,
            regiments: Vec::new(),
            is_naval,
            morale: 1.0,
            supply: 1.0,
            movement_points: 3.0,
            is_in_combat: false,
            is_sieging: None,
            destination: None,
            path: Vec::new(),
        }
    }

    pub fn total_strength(&self) -> f32 {
        self.regiments.iter().map(|r| r.effective_strength()).sum()
    }

    pub fn total_units(&self) -> u32 {
        self.regiments.iter().map(|r| r.count).sum()
    }

    pub fn add_regiment(&mut self, regiment: Regiment) {
        // Try to merge with existing regiment of same type
        for existing in &mut self.regiments {
            if existing.unit_type == regiment.unit_type {
                existing.count += regiment.count;
                existing.experience =
                    (existing.experience + regiment.experience) / 2.0;
                return;
            }
        }
        self.regiments.push(regiment);
    }

    pub fn siege_strength(&self) -> f32 {
        self.regiments
            .iter()
            .map(|r| r.unit_type.siege_bonus() * r.count as f32)
            .sum()
    }

    pub fn maintenance_cost(&self) -> f32 {
        self.regiments
            .iter()
            .map(|r| r.unit_type.maintenance_cost() * r.count as f32)
            .sum()
    }

    pub fn take_attrition(&mut self, rate: f32) {
        for regiment in &mut self.regiments {
            regiment.take_casualties(rate);
        }
        self.supply -= rate;
        self.supply = self.supply.max(0.0);
    }

    pub fn recover(&mut self) {
        for regiment in &mut self.regiments {
            regiment.recover();
        }
        self.supply = (self.supply + 0.1).min(1.0);
        self.morale = (self.morale + 0.05).min(1.0);
    }

    pub fn can_move(&self) -> bool {
        self.movement_points > 0.0 && !self.is_in_combat && self.total_units() > 0
    }

    pub fn move_to(&mut self, new_position: (u32, u32), cost: f32) {
        self.position = new_position;
        self.movement_points -= cost;
        self.movement_points = self.movement_points.max(0.0);
    }

    pub fn reset_movement(&mut self) {
        self.movement_points = 3.0;
    }

    pub fn remove_empty_regiments(&mut self) {
        self.regiments.retain(|r| r.count > 0);
    }
}

/// Result of a battle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleResult {
    pub id: BattleId,
    pub location: (u32, u32),
    pub year: i32,

    pub attacker_nation_id: NationId,
    pub defender_nation_id: NationId,

    pub attacker_army_ids: Vec<ArmyId>,
    pub defender_army_ids: Vec<ArmyId>,

    pub attacker_initial_strength: f32,
    pub defender_initial_strength: f32,

    pub attacker_casualties: u32,
    pub defender_casualties: u32,

    pub winner: Option<NationId>,
    pub is_decisive: bool,
}

/// Combat resolution system
pub struct CombatSystem;

impl CombatSystem {
    /// Resolve a battle between two forces
    pub fn resolve_battle(
        attackers: &mut [&mut Army],
        defenders: &mut [&mut Army],
        terrain_modifier: f32,
        fortification_level: f32,
        rng: &mut SmallRng,
    ) -> BattleResult {
        let attacker_nation = attackers[0].owner_nation_id;
        let defender_nation = defenders[0].owner_nation_id;

        let attacker_ids: Vec<ArmyId> = attackers.iter().map(|a| a.id).collect();
        let defender_ids: Vec<ArmyId> = defenders.iter().map(|a| a.id).collect();

        let attacker_initial: f32 = attackers.iter().map(|a| a.total_strength()).sum();
        let defender_initial: f32 = defenders.iter().map(|a| a.total_strength()).sum();

        // Calculate combat bonuses
        let terrain_bonus = terrain_modifier;
        let fort_bonus = 1.0 + fortification_level * 0.5;

        // Defender gets bonuses
        let effective_defender_strength = defender_initial * terrain_bonus * fort_bonus;

        // Calculate combat ratio
        let ratio = attacker_initial / effective_defender_strength.max(1.0);

        // Multiple combat rounds
        let mut attacker_remaining = attacker_initial;
        let mut defender_remaining = defender_initial;

        for _ in 0..5 {
            // Random factor
            let roll = rng.gen_range(0.8..1.2);

            let attacker_damage = (attacker_remaining * 0.15 * roll * ratio.sqrt())
                .min(defender_remaining * 0.3);
            let defender_damage = (defender_remaining * 0.15 * roll / ratio.sqrt() * fort_bonus)
                .min(attacker_remaining * 0.3);

            defender_remaining -= attacker_damage;
            attacker_remaining -= defender_damage;

            if defender_remaining <= 0.0 || attacker_remaining <= 0.0 {
                break;
            }
        }

        let attacker_losses = attacker_initial - attacker_remaining.max(0.0);
        let defender_losses = defender_initial - defender_remaining.max(0.0);

        // Apply casualties to armies
        let attacker_loss_percent = attacker_losses / attacker_initial.max(1.0);
        let defender_loss_percent = defender_losses / defender_initial.max(1.0);

        for army in attackers.iter_mut() {
            for regiment in &mut army.regiments {
                regiment.take_casualties(attacker_loss_percent);
                regiment.gain_experience(0.05);
            }
            army.morale -= attacker_loss_percent * 0.3;
            army.morale = army.morale.max(0.0);
        }

        for army in defenders.iter_mut() {
            for regiment in &mut army.regiments {
                regiment.take_casualties(defender_loss_percent);
                regiment.gain_experience(0.05);
            }
            army.morale -= defender_loss_percent * 0.3;
            army.morale = army.morale.max(0.0);
        }

        // Determine winner
        let winner = if defender_remaining <= 0.0 && attacker_remaining > 0.0 {
            Some(attacker_nation)
        } else if attacker_remaining <= 0.0 && defender_remaining > 0.0 {
            Some(defender_nation)
        } else if attacker_losses < defender_losses {
            Some(attacker_nation)
        } else if defender_losses < attacker_losses {
            Some(defender_nation)
        } else {
            None
        };

        let is_decisive = attacker_loss_percent > 0.5 || defender_loss_percent > 0.5;

        BattleResult {
            id: rng.gen(),
            location: attackers[0].position,
            year: 0, // Will be set by caller
            attacker_nation_id: attacker_nation,
            defender_nation_id: defender_nation,
            attacker_army_ids: attacker_ids,
            defender_army_ids: defender_ids,
            attacker_initial_strength: attacker_initial,
            defender_initial_strength: defender_initial,
            attacker_casualties: attacker_losses as u32,
            defender_casualties: defender_losses as u32,
            winner,
            is_decisive,
        }
    }

    /// Resolve a siege
    pub fn resolve_siege_tick(
        besiegers: &[&Army],
        garrison: u32,
        fortification: f32,
        supplies: f32,
        rng: &mut SmallRng,
    ) -> SiegeResult {
        let siege_strength: f32 = besiegers.iter().map(|a| a.siege_strength()).sum();
        let total_strength: f32 = besiegers.iter().map(|a| a.total_strength()).sum();

        let garrison_strength = garrison as f32 * (1.0 + fortification);

        // Siege progress
        let progress = (siege_strength / garrison_strength.max(10.0)) * 0.1;

        // Assault option
        let assault_success = if siege_strength > garrison_strength * 0.5 {
            let roll = rng.gen_range(0.0..1.0);
            roll < (total_strength / garrison_strength / 3.0).min(0.5)
        } else {
            false
        };

        // Starvation
        let starvation_damage = if supplies <= 0.0 { 0.05 } else { 0.0 };

        SiegeResult {
            progress,
            garrison_losses: (starvation_damage * garrison as f32) as u32,
            besieger_losses: 0,
            assault_possible: siege_strength > garrison_strength * 0.3,
            assault_success,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiegeResult {
    pub progress: f32,
    pub garrison_losses: u32,
    pub besieger_losses: u32,
    pub assault_possible: bool,
    pub assault_success: bool,
}

/// A war between nations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct War {
    pub id: u64,
    pub name: String,
    pub start_year: i32,
    pub end_year: Option<i32>,

    pub aggressor_id: NationId,
    pub defender_id: NationId,

    pub aggressor_allies: Vec<NationId>,
    pub defender_allies: Vec<NationId>,

    pub casus_belli: CasusBelli,

    pub battles: Vec<BattleResult>,
    pub territory_changes: Vec<(u32, u32, NationId)>, // location, new owner

    pub war_score: i32, // -100 to 100, positive = aggressor winning
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CasusBelli {
    Conquest,
    Liberation,
    HolyWar,
    Succession,
    Rebellion,
    Reconquest,
    Subjugation,
    TradeDispute,
    BorderIncident,
    Humiliation,
}

impl War {
    pub fn new(
        id: u64,
        aggressor_id: NationId,
        defender_id: NationId,
        casus_belli: CasusBelli,
        start_year: i32,
    ) -> Self {
        War {
            id,
            name: format!("War of {}", start_year),
            start_year,
            end_year: None,
            aggressor_id,
            defender_id,
            aggressor_allies: Vec::new(),
            defender_allies: Vec::new(),
            casus_belli,
            battles: Vec::new(),
            territory_changes: Vec::new(),
            war_score: 0,
        }
    }

    pub fn add_battle(&mut self, result: BattleResult) {
        // Update war score
        if result.winner == Some(self.aggressor_id) {
            self.war_score += if result.is_decisive { 10 } else { 5 };
        } else if result.winner == Some(self.defender_id) {
            self.war_score -= if result.is_decisive { 10 } else { 5 };
        }
        self.war_score = self.war_score.clamp(-100, 100);
        self.battles.push(result);
    }

    pub fn is_finished(&self) -> bool {
        self.end_year.is_some() || self.war_score.abs() >= 100
    }
}

/// Manages all armies in the simulation
pub struct MilitaryManager {
    pub armies: HashMap<ArmyId, Army>,
    pub wars: HashMap<u64, War>,
    pub next_army_id: ArmyId,
    pub next_war_id: u64,
}

impl MilitaryManager {
    pub fn new() -> Self {
        MilitaryManager {
            armies: HashMap::new(),
            wars: HashMap::new(),
            next_army_id: 1,
            next_war_id: 1,
        }
    }

    pub fn create_army(
        &mut self,
        name: String,
        owner_nation_id: NationId,
        position: (u32, u32),
        is_naval: bool,
    ) -> ArmyId {
        let id = self.next_army_id;
        self.next_army_id += 1;

        let army = Army::new(id, name, owner_nation_id, position, is_naval);
        self.armies.insert(id, army);
        id
    }

    pub fn get(&self, id: ArmyId) -> Option<&Army> {
        self.armies.get(&id)
    }

    pub fn get_mut(&mut self, id: ArmyId) -> Option<&mut Army> {
        self.armies.get_mut(&id)
    }

    pub fn get_armies_at(&self, position: (u32, u32)) -> Vec<&Army> {
        self.armies
            .values()
            .filter(|a| a.position == position)
            .collect()
    }

    pub fn get_armies_of_nation(&self, nation_id: NationId) -> Vec<&Army> {
        self.armies
            .values()
            .filter(|a| a.owner_nation_id == nation_id)
            .collect()
    }

    pub fn start_war(
        &mut self,
        aggressor_id: NationId,
        defender_id: NationId,
        casus_belli: CasusBelli,
        start_year: i32,
    ) -> u64 {
        let war_id = self.next_war_id;
        self.next_war_id += 1;

        let war = War::new(war_id, aggressor_id, defender_id, casus_belli, start_year);
        self.wars.insert(war_id, war);
        war_id
    }

    pub fn get_active_wars(&self) -> Vec<&War> {
        self.wars.values().filter(|w| !w.is_finished()).collect()
    }

    pub fn tick(&mut self) {
        // Remove empty armies
        self.armies.retain(|_, a| a.total_units() > 0);

        // Reset movement and recover armies not in combat
        for army in self.armies.values_mut() {
            army.reset_movement();
            if !army.is_in_combat {
                army.recover();
            }
            army.remove_empty_regiments();
        }
    }
}

/// Events that occur during military simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilitaryEvent {
    BattleFought {
        location: (u32, u32),
        attacker: NationId,
        defender: NationId,
        winner: Option<NationId>,
        attacker_losses: u32,
        defender_losses: u32,
    },
    SiegeStarted {
        settlement_id: SettlementId,
        attacker: NationId,
        defender: NationId,
    },
    SiegeEnded {
        settlement_id: SettlementId,
        winner: NationId,
    },
    ArmyCreated {
        army_id: ArmyId,
        nation_id: NationId,
        name: String,
    },
    ArmyDestroyed {
        army_id: ArmyId,
        nation_id: NationId,
        destroyer: Option<NationId>,
    },
    WarStarted {
        war_id: u64,
        aggressor: NationId,
        defender: NationId,
        casus_belli: CasusBelli,
    },
    WarEnded {
        war_id: u64,
        victor: Option<NationId>,
        duration_years: i32,
    },
}
