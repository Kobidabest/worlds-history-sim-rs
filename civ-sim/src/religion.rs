use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ReligionId = u64;

/// Types of religious structures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReligionType {
    Animism,        // Spirits in nature
    Polytheism,     // Multiple gods
    Henotheism,     // One main god among many
    Monotheism,     // Single god
    Dualism,        // Two opposing forces
    Pantheism,      // God is everything
    Nontheism,      // No gods (philosophical)
    AncestorWorship, // Veneration of ancestors
}

impl ReligionType {
    pub fn conversion_rate(&self) -> f32 {
        match self {
            ReligionType::Monotheism => 1.3,
            ReligionType::Henotheism => 1.1,
            ReligionType::Polytheism => 0.9,
            ReligionType::Animism => 0.7,
            ReligionType::AncestorWorship => 0.6,
            ReligionType::Dualism => 1.0,
            ReligionType::Pantheism => 0.8,
            ReligionType::Nontheism => 0.5,
        }
    }

    pub fn tolerance(&self) -> f32 {
        match self {
            ReligionType::Polytheism => 0.8,
            ReligionType::Animism => 0.9,
            ReligionType::Pantheism => 0.9,
            ReligionType::Henotheism => 0.5,
            ReligionType::Monotheism => 0.3,
            ReligionType::Dualism => 0.4,
            ReligionType::Nontheism => 0.7,
            ReligionType::AncestorWorship => 0.8,
        }
    }
}

/// Core beliefs and doctrines
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Belief {
    // Afterlife
    Reincarnation,
    Heaven,
    Hell,
    AncestralRealm,
    Annihilation,
    SpiritWorld,

    // Ethics
    NonViolence,
    HolyWar,
    Charity,
    Asceticism,
    Hedonism,
    Justice,
    Mercy,
    Humility,
    Pride,

    // Practices
    Prayer,
    Meditation,
    Sacrifice,
    Pilgrimage,
    Fasting,
    Tithing,
    Monasticism,
    Proselytizing,

    // Nature
    NatureSacred,
    AnimalRespect,
    FertilityWorship,
    SunWorship,
    MoonWorship,
    WaterSacred,

    // Social
    SocialHierarchy,
    Equality,
    Patriarchy,
    GenderEquality,

    // Knowledge
    SacredTexts,
    OralTradition,
    MysticalKnowledge,
    RationalFaith,
}

impl Belief {
    pub fn happiness_effect(&self) -> f32 {
        match self {
            Belief::Heaven => 0.1,
            Belief::Charity => 0.1,
            Belief::Justice => 0.05,
            Belief::Mercy => 0.05,
            Belief::Hell => -0.05,
            Belief::Asceticism => -0.05,
            Belief::Equality => 0.1,
            _ => 0.0,
        }
    }

    pub fn military_effect(&self) -> f32 {
        match self {
            Belief::HolyWar => 0.25,
            Belief::NonViolence => -0.2,
            Belief::Sacrifice => 0.1,
            Belief::Pride => 0.1,
            Belief::Humility => -0.05,
            _ => 0.0,
        }
    }

    pub fn production_effect(&self) -> f32 {
        match self {
            Belief::Asceticism => 0.15,
            Belief::Monasticism => 0.1,
            Belief::Hedonism => -0.1,
            Belief::Tithing => -0.05,
            _ => 0.0,
        }
    }

    pub fn research_effect(&self) -> f32 {
        match self {
            Belief::SacredTexts => 0.15,
            Belief::MysticalKnowledge => 0.1,
            Belief::RationalFaith => 0.2,
            Belief::OralTradition => -0.1,
            _ => 0.0,
        }
    }

    pub fn faith_effect(&self) -> f32 {
        match self {
            Belief::Prayer => 0.2,
            Belief::Meditation => 0.15,
            Belief::Pilgrimage => 0.15,
            Belief::Fasting => 0.1,
            Belief::Monasticism => 0.25,
            Belief::Proselytizing => 0.1,
            _ => 0.0,
        }
    }

    pub fn conflicts_with(&self, other: &Belief) -> bool {
        matches!(
            (self, other),
            (Belief::NonViolence, Belief::HolyWar)
                | (Belief::HolyWar, Belief::NonViolence)
                | (Belief::Asceticism, Belief::Hedonism)
                | (Belief::Hedonism, Belief::Asceticism)
                | (Belief::SocialHierarchy, Belief::Equality)
                | (Belief::Equality, Belief::SocialHierarchy)
                | (Belief::Patriarchy, Belief::GenderEquality)
                | (Belief::GenderEquality, Belief::Patriarchy)
                | (Belief::Pride, Belief::Humility)
                | (Belief::Humility, Belief::Pride)
                | (Belief::SacredTexts, Belief::OralTradition)
                | (Belief::OralTradition, Belief::SacredTexts)
        )
    }
}

/// A deity in a polytheistic or henotheistic religion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deity {
    pub name: String,
    pub title: String,
    pub domain: DeityDomain,
    pub is_chief: bool,
    pub is_good: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeityDomain {
    Sun,
    Moon,
    Sky,
    Storm,
    War,
    Death,
    Fertility,
    Sea,
    Earth,
    Fire,
    Wisdom,
    Love,
    Justice,
    Harvest,
    Craft,
    Trade,
    Healing,
    Magic,
    Trickster,
    Underworld,
}

impl DeityDomain {
    pub fn typical_name_prefix(&self) -> &str {
        match self {
            DeityDomain::Sun => "Sol",
            DeityDomain::Moon => "Lun",
            DeityDomain::Sky => "Ur",
            DeityDomain::Storm => "Thor",
            DeityDomain::War => "Ar",
            DeityDomain::Death => "Mort",
            DeityDomain::Fertility => "Fer",
            DeityDomain::Sea => "Pos",
            DeityDomain::Earth => "Gai",
            DeityDomain::Fire => "Agn",
            DeityDomain::Wisdom => "Ath",
            DeityDomain::Love => "Ven",
            DeityDomain::Justice => "Jus",
            DeityDomain::Harvest => "Dem",
            DeityDomain::Craft => "Hep",
            DeityDomain::Trade => "Mer",
            DeityDomain::Healing => "Asc",
            DeityDomain::Magic => "Hec",
            DeityDomain::Trickster => "Lok",
            DeityDomain::Underworld => "Had",
        }
    }
}

/// A religion in the simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Religion {
    pub id: ReligionId,
    pub name: String,
    pub religion_type: ReligionType,
    pub founded_year: i32,
    pub founder_name: Option<String>,
    pub holy_city: Option<(u32, u32)>,
    pub origin_culture_id: Option<u64>,

    pub beliefs: Vec<Belief>,
    pub deities: Vec<Deity>,

    // Sacred elements
    pub sacred_animal: Option<String>,
    pub sacred_color: (u8, u8, u8),
    pub holy_symbol: String,

    // Organizational structure
    pub has_clergy: bool,
    pub has_holy_texts: bool,
    pub hierarchy_level: u8, // 0 = decentralized, 5 = highly centralized

    // Statistics
    pub follower_count: u32,
    pub settlement_count: u32,
    pub nation_count: u32,
    pub prestige: f32,
    pub fervor: f32, // 0-1, how actively believers spread the faith

    // Parent religion for schisms
    pub parent_religion_id: Option<ReligionId>,
}

impl Religion {
    pub fn new(
        id: ReligionId,
        name: String,
        religion_type: ReligionType,
        founded_year: i32,
        rng: &mut SmallRng,
    ) -> Self {
        let sacred_color = (rng.gen(), rng.gen(), rng.gen());

        let mut beliefs = Vec::new();
        let all_beliefs = [
            Belief::Prayer,
            Belief::Charity,
            Belief::Justice,
            Belief::Pilgrimage,
            Belief::SacredTexts,
            Belief::NatureSacred,
            Belief::Heaven,
            Belief::Mercy,
            Belief::Humility,
        ];

        let num_beliefs = rng.gen_range(3..7);
        for _ in 0..num_beliefs {
            let idx = rng.gen_range(0..all_beliefs.len());
            let b = all_beliefs[idx];
            if !beliefs.iter().any(|existing| b.conflicts_with(existing)) {
                beliefs.push(b);
            }
        }

        let mut deities = Vec::new();
        match religion_type {
            ReligionType::Polytheism | ReligionType::Henotheism => {
                let num_gods = match religion_type {
                    ReligionType::Polytheism => rng.gen_range(5..12),
                    ReligionType::Henotheism => rng.gen_range(3..7),
                    _ => 0,
                };

                let domains = [
                    DeityDomain::Sun,
                    DeityDomain::Moon,
                    DeityDomain::Sky,
                    DeityDomain::Storm,
                    DeityDomain::War,
                    DeityDomain::Death,
                    DeityDomain::Fertility,
                    DeityDomain::Sea,
                    DeityDomain::Wisdom,
                    DeityDomain::Love,
                ];

                for i in 0..num_gods {
                    let domain = domains[i % domains.len()];
                    let deity_name = format!("{}us", domain.typical_name_prefix());
                    deities.push(Deity {
                        name: deity_name,
                        title: format!("God of {:?}", domain),
                        domain,
                        is_chief: i == 0,
                        is_good: rng.gen_bool(0.8),
                    });
                }
            }
            ReligionType::Monotheism => {
                deities.push(Deity {
                    name: crate::names::generate_deity_name(rng),
                    title: "The One True God".to_string(),
                    domain: DeityDomain::Sky,
                    is_chief: true,
                    is_good: true,
                });
            }
            ReligionType::Dualism => {
                deities.push(Deity {
                    name: "Ahura".to_string(),
                    title: "Lord of Light".to_string(),
                    domain: DeityDomain::Sun,
                    is_chief: true,
                    is_good: true,
                });
                deities.push(Deity {
                    name: "Angra".to_string(),
                    title: "Lord of Darkness".to_string(),
                    domain: DeityDomain::Death,
                    is_chief: true,
                    is_good: false,
                });
            }
            _ => {}
        }

        let has_holy_texts = beliefs.contains(&Belief::SacredTexts);

        Religion {
            id,
            name,
            religion_type,
            founded_year,
            founder_name: None,
            holy_city: None,
            origin_culture_id: None,
            beliefs,
            deities,
            sacred_animal: None,
            sacred_color,
            holy_symbol: "Star".to_string(),
            has_clergy: rng.gen_bool(0.6),
            has_holy_texts,
            hierarchy_level: rng.gen_range(0..6),
            follower_count: 0,
            settlement_count: 0,
            nation_count: 0,
            prestige: 10.0,
            fervor: rng.gen_range(0.3..0.8),
            parent_religion_id: None,
        }
    }

    /// Calculate effects on a settlement
    pub fn calculate_effects(&self) -> ReligionEffects {
        let mut effects = ReligionEffects::default();

        for belief in &self.beliefs {
            effects.happiness += belief.happiness_effect();
            effects.military += belief.military_effect();
            effects.production += belief.production_effect();
            effects.research += belief.research_effect();
            effects.faith += belief.faith_effect();
        }

        // Religion type modifiers
        effects.conversion_rate = self.religion_type.conversion_rate();
        effects.tolerance = self.religion_type.tolerance();

        // Fervor affects spread
        effects.spread_rate = self.fervor * effects.conversion_rate;

        // Centralized religions are more effective at maintaining unity
        effects.stability = self.hierarchy_level as f32 * 0.05;

        effects
    }

    /// Create a schism (heretic branch)
    pub fn create_schism(
        &self,
        new_id: ReligionId,
        new_name: String,
        current_year: i32,
        rng: &mut SmallRng,
    ) -> Religion {
        let mut heresy = self.clone();
        heresy.id = new_id;
        heresy.name = new_name;
        heresy.founded_year = current_year;
        heresy.parent_religion_id = Some(self.id);
        heresy.follower_count = 0;
        heresy.settlement_count = 0;
        heresy.nation_count = 0;
        heresy.prestige = self.prestige * 0.3;

        // Modify some beliefs
        if rng.gen_bool(0.5) && heresy.beliefs.len() > 2 {
            let idx = rng.gen_range(0..heresy.beliefs.len());
            heresy.beliefs.remove(idx);
        }

        // Add a new belief
        let new_beliefs = [
            Belief::Asceticism,
            Belief::Proselytizing,
            Belief::RationalFaith,
            Belief::MysticalKnowledge,
            Belief::Equality,
        ];
        let new_belief = new_beliefs[rng.gen_range(0..new_beliefs.len())];
        if !heresy
            .beliefs
            .iter()
            .any(|b| new_belief.conflicts_with(b))
        {
            heresy.beliefs.push(new_belief);
        }

        // Heresies often have higher fervor
        heresy.fervor = (heresy.fervor + 0.2).min(1.0);

        heresy
    }

    /// Calculate conversion chance to this religion
    pub fn conversion_chance(&self, target_religion: Option<&Religion>, target_piety: f32) -> f32 {
        let base = 0.02 * self.fervor * self.religion_type.conversion_rate();

        let prestige_factor = if let Some(other) = target_religion {
            (self.prestige / (other.prestige + 1.0)).min(2.0)
        } else {
            1.5 // Easier to convert from no religion
        };

        let piety_resistance = 1.0 - (target_piety * 0.3);

        base * prestige_factor * piety_resistance
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReligionEffects {
    pub happiness: f32,
    pub military: f32,
    pub production: f32,
    pub research: f32,
    pub faith: f32,
    pub conversion_rate: f32,
    pub tolerance: f32,
    pub spread_rate: f32,
    pub stability: f32,
}

/// Manages all religions in the simulation
pub struct ReligionManager {
    pub religions: HashMap<ReligionId, Religion>,
    pub next_id: ReligionId,
}

impl ReligionManager {
    pub fn new() -> Self {
        ReligionManager {
            religions: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_religion(
        &mut self,
        name: String,
        religion_type: ReligionType,
        founded_year: i32,
        rng: &mut SmallRng,
    ) -> ReligionId {
        let id = self.next_id;
        self.next_id += 1;

        let religion = Religion::new(id, name, religion_type, founded_year, rng);
        self.religions.insert(id, religion);
        id
    }

    pub fn get(&self, id: ReligionId) -> Option<&Religion> {
        self.religions.get(&id)
    }

    pub fn get_mut(&mut self, id: ReligionId) -> Option<&mut Religion> {
        self.religions.get_mut(&id)
    }

    /// Process religious changes for a tick
    pub fn tick(&mut self, current_year: i32, rng: &mut SmallRng) -> Vec<ReligionEvent> {
        let mut events = Vec::new();

        // Check for schisms in large religions
        let religion_ids: Vec<ReligionId> = self.religions.keys().cloned().collect();

        for id in religion_ids {
            let should_schism = {
                if let Some(religion) = self.religions.get(&id) {
                    religion.follower_count > 100000 && rng.gen_bool(0.001)
                } else {
                    false
                }
            };

            if should_schism {
                let (new_name, new_id) = {
                    let religion = self.religions.get(&id).unwrap();
                    let new_name = format!("Reformed {}", religion.name);
                    (new_name, self.next_id)
                };
                self.next_id += 1;

                if let Some(parent) = self.religions.get(&id) {
                    let heresy = parent.create_schism(new_id, new_name.clone(), current_year, rng);
                    events.push(ReligionEvent::Schism {
                        parent_id: id,
                        child_id: new_id,
                        child_name: new_name,
                    });
                    self.religions.insert(new_id, heresy);
                }
            }
        }

        // Update prestige based on followers
        for religion in self.religions.values_mut() {
            religion.prestige =
                (religion.prestige * 0.99 + religion.follower_count as f32 * 0.0001)
                    .clamp(1.0, 1000.0);
        }

        events
    }

    /// Get religions that might spread to a location
    pub fn get_spreading_religions(&self, location: (u32, u32)) -> Vec<(ReligionId, f32)> {
        let mut influences: Vec<(ReligionId, f32)> = self
            .religions
            .iter()
            .map(|(id, r)| (*id, r.prestige * r.fervor * r.religion_type.conversion_rate()))
            .filter(|(_, influence)| *influence > 1.0)
            .collect();

        influences.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        influences
    }
}

/// Events that occur during religious simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReligionEvent {
    Founded {
        religion_id: ReligionId,
        religion_name: String,
        founder_name: String,
    },
    Schism {
        parent_id: ReligionId,
        child_id: ReligionId,
        child_name: String,
    },
    HolyCityEstablished {
        religion_id: ReligionId,
        location: (u32, u32),
    },
    MassConversion {
        religion_id: ReligionId,
        nation_name: String,
        population: u32,
    },
    Persecution {
        persecuting_religion_id: ReligionId,
        persecuted_religion_id: ReligionId,
        nation_name: String,
    },
    ReligiousWar {
        religion_a: ReligionId,
        religion_b: ReligionId,
    },
    Reformation {
        religion_id: ReligionId,
        changed_beliefs: Vec<Belief>,
    },
}
