use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type TechnologyId = u64;

/// Technology eras that represent major periods of advancement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum TechEra {
    StoneAge,
    BronzeAge,
    IronAge,
    ClassicalAge,
    MedievalAge,
    RenaissanceAge,
    IndustrialAge,
    ModernAge,
}

impl TechEra {
    pub fn research_modifier(&self) -> f32 {
        match self {
            TechEra::StoneAge => 1.0,
            TechEra::BronzeAge => 1.5,
            TechEra::IronAge => 2.0,
            TechEra::ClassicalAge => 2.5,
            TechEra::MedievalAge => 3.0,
            TechEra::RenaissanceAge => 4.0,
            TechEra::IndustrialAge => 6.0,
            TechEra::ModernAge => 10.0,
        }
    }

    pub fn era_start_year(&self) -> i32 {
        match self {
            TechEra::StoneAge => -50000,
            TechEra::BronzeAge => -3300,
            TechEra::IronAge => -1200,
            TechEra::ClassicalAge => -500,
            TechEra::MedievalAge => 500,
            TechEra::RenaissanceAge => 1300,
            TechEra::IndustrialAge => 1760,
            TechEra::ModernAge => 1900,
        }
    }
}

/// Categories of technology
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TechCategory {
    Military,
    Agriculture,
    Infrastructure,
    Maritime,
    Science,
    Culture,
    Religion,
    Government,
    Economics,
    Industry,
}

/// A single technology that can be researched
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technology {
    pub id: TechnologyId,
    pub name: String,
    pub description: String,
    pub era: TechEra,
    pub category: TechCategory,
    pub research_cost: f32,
    pub prerequisites: Vec<TechnologyId>,
    pub effects: Vec<TechEffect>,
    pub discovery_year: Option<i32>,
}

/// Effects that technologies can have
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechEffect {
    // Military
    UnlockUnit(String),
    CombatBonus(f32),
    DefenseBonus(f32),
    SiegeBonus(f32),

    // Economy
    GoldBonus(f32),
    TradeRangeBonus(f32),
    ProductionBonus(f32),

    // Agriculture
    FoodBonus(f32),
    FarmingEfficiency(f32),
    PopulationGrowthBonus(f32),

    // Infrastructure
    UnlockBuilding(String),
    MovementSpeedBonus(f32),
    ConstructionSpeedBonus(f32),

    // Maritime
    UnlockShip(String),
    NavalCombatBonus(f32),
    ExplorationRange(f32),

    // Culture/Religion
    CultureBonus(f32),
    FaithBonus(f32),
    HappinessBonus(f32),

    // Science
    ResearchBonus(f32),

    // Government
    UnlockGovernment(String),
    AdministrativeEfficiency(f32),
    DiplomacyBonus(f32),

    // Special
    EnablesFeature(String),
}

/// The complete technology tree
pub struct TechTree {
    pub technologies: HashMap<TechnologyId, Technology>,
    pub by_era: HashMap<TechEra, Vec<TechnologyId>>,
    pub by_category: HashMap<TechCategory, Vec<TechnologyId>>,
}

impl TechTree {
    pub fn new() -> Self {
        let mut tree = TechTree {
            technologies: HashMap::new(),
            by_era: HashMap::new(),
            by_category: HashMap::new(),
        };

        tree.initialize_technologies();
        tree
    }

    fn add_tech(&mut self, tech: Technology) {
        let id = tech.id;
        let era = tech.era;
        let category = tech.category;

        self.by_era.entry(era).or_insert_with(Vec::new).push(id);
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id);
        self.technologies.insert(id, tech);
    }

    fn initialize_technologies(&mut self) {
        // =============== STONE AGE ===============
        self.add_tech(Technology {
            id: 1,
            name: "Fire Making".to_string(),
            description: "Mastery of fire for warmth, cooking, and protection".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Science,
            research_cost: 10.0,
            prerequisites: vec![],
            effects: vec![
                TechEffect::FoodBonus(0.1),
                TechEffect::PopulationGrowthBonus(0.05),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 2,
            name: "Tool Making".to_string(),
            description: "Creating basic stone tools for various purposes".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Industry,
            research_cost: 15.0,
            prerequisites: vec![],
            effects: vec![
                TechEffect::ProductionBonus(0.1),
                TechEffect::CombatBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 3,
            name: "Hunting".to_string(),
            description: "Organized hunting techniques".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Agriculture,
            research_cost: 20.0,
            prerequisites: vec![2],
            effects: vec![TechEffect::FoodBonus(0.15)],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 4,
            name: "Gathering".to_string(),
            description: "Knowledge of edible plants and foraging".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Agriculture,
            research_cost: 15.0,
            prerequisites: vec![],
            effects: vec![TechEffect::FoodBonus(0.1)],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 5,
            name: "Primitive Shelter".to_string(),
            description: "Building basic shelters from natural materials".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Infrastructure,
            research_cost: 20.0,
            prerequisites: vec![2],
            effects: vec![TechEffect::DefenseBonus(0.1)],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 6,
            name: "Language".to_string(),
            description: "Development of complex spoken language".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Culture,
            research_cost: 25.0,
            prerequisites: vec![],
            effects: vec![
                TechEffect::ResearchBonus(0.1),
                TechEffect::CultureBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 7,
            name: "Tribal Organization".to_string(),
            description: "Basic social structure and leadership".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Government,
            research_cost: 30.0,
            prerequisites: vec![6],
            effects: vec![
                TechEffect::AdministrativeEfficiency(0.1),
                TechEffect::UnlockGovernment("Tribal".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 8,
            name: "Animal Domestication".to_string(),
            description: "Taming animals for labor and food".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Agriculture,
            research_cost: 40.0,
            prerequisites: vec![3],
            effects: vec![
                TechEffect::FoodBonus(0.2),
                TechEffect::MovementSpeedBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 9,
            name: "Agriculture".to_string(),
            description: "Systematic cultivation of crops".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Agriculture,
            research_cost: 50.0,
            prerequisites: vec![4, 2],
            effects: vec![
                TechEffect::FarmingEfficiency(0.3),
                TechEffect::FoodBonus(0.3),
                TechEffect::PopulationGrowthBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 10,
            name: "Pottery".to_string(),
            description: "Creating vessels for storage and cooking".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Industry,
            research_cost: 35.0,
            prerequisites: vec![1],
            effects: vec![
                TechEffect::FoodBonus(0.1),
                TechEffect::TradeRangeBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 11,
            name: "Weaving".to_string(),
            description: "Creating cloth from plant fibers and wool".to_string(),
            era: TechEra::StoneAge,
            category: TechCategory::Industry,
            research_cost: 30.0,
            prerequisites: vec![8],
            effects: vec![
                TechEffect::GoldBonus(0.1),
                TechEffect::HappinessBonus(0.05),
            ],
            discovery_year: None,
        });

        // =============== BRONZE AGE ===============
        self.add_tech(Technology {
            id: 100,
            name: "Bronze Working".to_string(),
            description: "Smelting copper and tin to create bronze".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Industry,
            research_cost: 80.0,
            prerequisites: vec![2, 1],
            effects: vec![
                TechEffect::CombatBonus(0.2),
                TechEffect::ProductionBonus(0.15),
                TechEffect::UnlockUnit("Bronze Warriors".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 101,
            name: "Writing".to_string(),
            description: "Recording information through symbols".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Culture,
            research_cost: 100.0,
            prerequisites: vec![6],
            effects: vec![
                TechEffect::ResearchBonus(0.2),
                TechEffect::CultureBonus(0.15),
                TechEffect::AdministrativeEfficiency(0.15),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 102,
            name: "The Wheel".to_string(),
            description: "Revolutionary transportation technology".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Infrastructure,
            research_cost: 70.0,
            prerequisites: vec![8],
            effects: vec![
                TechEffect::MovementSpeedBonus(0.3),
                TechEffect::TradeRangeBonus(0.2),
                TechEffect::ProductionBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 103,
            name: "Sailing".to_string(),
            description: "Harnessing wind for water travel".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Maritime,
            research_cost: 90.0,
            prerequisites: vec![11],
            effects: vec![
                TechEffect::UnlockShip("Galley".to_string()),
                TechEffect::TradeRangeBonus(0.3),
                TechEffect::ExplorationRange(0.5),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 104,
            name: "Irrigation".to_string(),
            description: "Controlled water distribution for farming".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Agriculture,
            research_cost: 75.0,
            prerequisites: vec![9],
            effects: vec![
                TechEffect::FarmingEfficiency(0.3),
                TechEffect::FoodBonus(0.25),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 105,
            name: "Masonry".to_string(),
            description: "Building with shaped stone".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Infrastructure,
            research_cost: 80.0,
            prerequisites: vec![5, 2],
            effects: vec![
                TechEffect::DefenseBonus(0.2),
                TechEffect::UnlockBuilding("Stone Walls".to_string()),
                TechEffect::ConstructionSpeedBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 106,
            name: "Calendar".to_string(),
            description: "Tracking seasons and astronomical events".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Science,
            research_cost: 60.0,
            prerequisites: vec![9, 101],
            effects: vec![
                TechEffect::FarmingEfficiency(0.15),
                TechEffect::FaithBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 107,
            name: "Mathematics".to_string(),
            description: "Basic numerical systems and geometry".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Science,
            research_cost: 85.0,
            prerequisites: vec![101],
            effects: vec![
                TechEffect::ResearchBonus(0.15),
                TechEffect::ConstructionSpeedBonus(0.15),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 108,
            name: "Code of Laws".to_string(),
            description: "Formal legal systems".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Government,
            research_cost: 100.0,
            prerequisites: vec![101, 7],
            effects: vec![
                TechEffect::AdministrativeEfficiency(0.2),
                TechEffect::HappinessBonus(0.1),
                TechEffect::UnlockGovernment("Monarchy".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 109,
            name: "Currency".to_string(),
            description: "Standardized medium of exchange".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Economics,
            research_cost: 70.0,
            prerequisites: vec![100, 101],
            effects: vec![
                TechEffect::GoldBonus(0.25),
                TechEffect::TradeRangeBonus(0.2),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 110,
            name: "Archery".to_string(),
            description: "Bow and arrow warfare".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Military,
            research_cost: 55.0,
            prerequisites: vec![3],
            effects: vec![
                TechEffect::CombatBonus(0.15),
                TechEffect::UnlockUnit("Archers".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 111,
            name: "Horseback Riding".to_string(),
            description: "Using horses for mounted warfare".to_string(),
            era: TechEra::BronzeAge,
            category: TechCategory::Military,
            research_cost: 90.0,
            prerequisites: vec![8],
            effects: vec![
                TechEffect::MovementSpeedBonus(0.4),
                TechEffect::CombatBonus(0.2),
                TechEffect::UnlockUnit("Light Cavalry".to_string()),
            ],
            discovery_year: None,
        });

        // =============== IRON AGE ===============
        self.add_tech(Technology {
            id: 200,
            name: "Iron Working".to_string(),
            description: "Smelting and forging iron".to_string(),
            era: TechEra::IronAge,
            category: TechCategory::Industry,
            research_cost: 150.0,
            prerequisites: vec![100],
            effects: vec![
                TechEffect::CombatBonus(0.25),
                TechEffect::ProductionBonus(0.2),
                TechEffect::UnlockUnit("Iron Swordsmen".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 201,
            name: "Alphabet".to_string(),
            description: "Phonetic writing system".to_string(),
            era: TechEra::IronAge,
            category: TechCategory::Culture,
            research_cost: 120.0,
            prerequisites: vec![101],
            effects: vec![
                TechEffect::ResearchBonus(0.2),
                TechEffect::CultureBonus(0.2),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 202,
            name: "Construction".to_string(),
            description: "Advanced building techniques".to_string(),
            era: TechEra::IronAge,
            category: TechCategory::Infrastructure,
            research_cost: 130.0,
            prerequisites: vec![105, 107],
            effects: vec![
                TechEffect::ConstructionSpeedBonus(0.25),
                TechEffect::UnlockBuilding("Aqueduct".to_string()),
                TechEffect::UnlockBuilding("Colosseum".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 203,
            name: "Military Tactics".to_string(),
            description: "Organized battlefield formations".to_string(),
            era: TechEra::IronAge,
            category: TechCategory::Military,
            research_cost: 140.0,
            prerequisites: vec![108, 200],
            effects: vec![
                TechEffect::CombatBonus(0.3),
                TechEffect::UnlockUnit("Phalanx".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 204,
            name: "Philosophy".to_string(),
            description: "Systematic inquiry into existence and ethics".to_string(),
            era: TechEra::IronAge,
            category: TechCategory::Culture,
            research_cost: 160.0,
            prerequisites: vec![201, 107],
            effects: vec![
                TechEffect::ResearchBonus(0.25),
                TechEffect::CultureBonus(0.2),
                TechEffect::HappinessBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 205,
            name: "Drama and Poetry".to_string(),
            description: "Theatrical and literary arts".to_string(),
            era: TechEra::IronAge,
            category: TechCategory::Culture,
            research_cost: 100.0,
            prerequisites: vec![201],
            effects: vec![
                TechEffect::CultureBonus(0.3),
                TechEffect::HappinessBonus(0.15),
                TechEffect::UnlockBuilding("Theater".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 206,
            name: "Triremes".to_string(),
            description: "Advanced warships with three rows of oars".to_string(),
            era: TechEra::IronAge,
            category: TechCategory::Maritime,
            research_cost: 140.0,
            prerequisites: vec![103, 200],
            effects: vec![
                TechEffect::NavalCombatBonus(0.3),
                TechEffect::UnlockShip("Trireme".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 207,
            name: "Democracy".to_string(),
            description: "Government by citizen participation".to_string(),
            era: TechEra::IronAge,
            category: TechCategory::Government,
            research_cost: 180.0,
            prerequisites: vec![108, 204],
            effects: vec![
                TechEffect::UnlockGovernment("Republic".to_string()),
                TechEffect::HappinessBonus(0.2),
                TechEffect::ResearchBonus(0.1),
            ],
            discovery_year: None,
        });

        // =============== CLASSICAL AGE ===============
        self.add_tech(Technology {
            id: 300,
            name: "Engineering".to_string(),
            description: "Systematic application of scientific principles to construction"
                .to_string(),
            era: TechEra::ClassicalAge,
            category: TechCategory::Infrastructure,
            research_cost: 200.0,
            prerequisites: vec![202, 107],
            effects: vec![
                TechEffect::ConstructionSpeedBonus(0.3),
                TechEffect::UnlockBuilding("Roads".to_string()),
                TechEffect::MovementSpeedBonus(0.2),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 301,
            name: "Steel".to_string(),
            description: "Advanced iron alloy production".to_string(),
            era: TechEra::ClassicalAge,
            category: TechCategory::Industry,
            research_cost: 220.0,
            prerequisites: vec![200],
            effects: vec![
                TechEffect::CombatBonus(0.25),
                TechEffect::ProductionBonus(0.2),
                TechEffect::UnlockUnit("Legionaries".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 302,
            name: "Medicine".to_string(),
            description: "Systematic treatment of disease".to_string(),
            era: TechEra::ClassicalAge,
            category: TechCategory::Science,
            research_cost: 180.0,
            prerequisites: vec![204],
            effects: vec![
                TechEffect::PopulationGrowthBonus(0.15),
                TechEffect::UnlockBuilding("Hospital".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 303,
            name: "Theology".to_string(),
            description: "Systematic study of religion and divinity".to_string(),
            era: TechEra::ClassicalAge,
            category: TechCategory::Religion,
            research_cost: 170.0,
            prerequisites: vec![204],
            effects: vec![
                TechEffect::FaithBonus(0.3),
                TechEffect::UnlockBuilding("Cathedral".to_string()),
            ],
            discovery_year: None,
        });

        // =============== MEDIEVAL AGE ===============
        self.add_tech(Technology {
            id: 400,
            name: "Feudalism".to_string(),
            description: "Hierarchical land-based social system".to_string(),
            era: TechEra::MedievalAge,
            category: TechCategory::Government,
            research_cost: 250.0,
            prerequisites: vec![108],
            effects: vec![
                TechEffect::UnlockGovernment("Feudal".to_string()),
                TechEffect::DefenseBonus(0.2),
                TechEffect::UnlockUnit("Knights".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 401,
            name: "Heavy Plow".to_string(),
            description: "Iron-tipped plow for heavy soils".to_string(),
            era: TechEra::MedievalAge,
            category: TechCategory::Agriculture,
            research_cost: 200.0,
            prerequisites: vec![200, 104],
            effects: vec![
                TechEffect::FarmingEfficiency(0.35),
                TechEffect::FoodBonus(0.25),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 402,
            name: "Castle Building".to_string(),
            description: "Fortified noble residences".to_string(),
            era: TechEra::MedievalAge,
            category: TechCategory::Military,
            research_cost: 280.0,
            prerequisites: vec![202, 400],
            effects: vec![
                TechEffect::DefenseBonus(0.4),
                TechEffect::UnlockBuilding("Castle".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 403,
            name: "Guilds".to_string(),
            description: "Organized trade associations".to_string(),
            era: TechEra::MedievalAge,
            category: TechCategory::Economics,
            research_cost: 220.0,
            prerequisites: vec![109],
            effects: vec![
                TechEffect::ProductionBonus(0.25),
                TechEffect::GoldBonus(0.2),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 404,
            name: "Gunpowder".to_string(),
            description: "Explosive black powder".to_string(),
            era: TechEra::MedievalAge,
            category: TechCategory::Military,
            research_cost: 350.0,
            prerequisites: vec![200],
            effects: vec![
                TechEffect::CombatBonus(0.35),
                TechEffect::SiegeBonus(0.5),
                TechEffect::UnlockUnit("Musketeers".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 405,
            name: "Compass".to_string(),
            description: "Magnetic navigation device".to_string(),
            era: TechEra::MedievalAge,
            category: TechCategory::Maritime,
            research_cost: 200.0,
            prerequisites: vec![103, 200],
            effects: vec![
                TechEffect::ExplorationRange(0.5),
                TechEffect::NavalCombatBonus(0.1),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 406,
            name: "Universities".to_string(),
            description: "Institutions of higher learning".to_string(),
            era: TechEra::MedievalAge,
            category: TechCategory::Science,
            research_cost: 300.0,
            prerequisites: vec![204, 303],
            effects: vec![
                TechEffect::ResearchBonus(0.4),
                TechEffect::UnlockBuilding("University".to_string()),
            ],
            discovery_year: None,
        });

        // =============== RENAISSANCE AGE ===============
        self.add_tech(Technology {
            id: 500,
            name: "Printing Press".to_string(),
            description: "Movable type printing".to_string(),
            era: TechEra::RenaissanceAge,
            category: TechCategory::Culture,
            research_cost: 400.0,
            prerequisites: vec![201, 403],
            effects: vec![
                TechEffect::ResearchBonus(0.35),
                TechEffect::CultureBonus(0.3),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 501,
            name: "Banking".to_string(),
            description: "Modern financial institutions".to_string(),
            era: TechEra::RenaissanceAge,
            category: TechCategory::Economics,
            research_cost: 380.0,
            prerequisites: vec![109, 403],
            effects: vec![
                TechEffect::GoldBonus(0.4),
                TechEffect::UnlockBuilding("Bank".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 502,
            name: "Astronomy".to_string(),
            description: "Scientific study of celestial bodies".to_string(),
            era: TechEra::RenaissanceAge,
            category: TechCategory::Science,
            research_cost: 420.0,
            prerequisites: vec![107, 406],
            effects: vec![
                TechEffect::ResearchBonus(0.25),
                TechEffect::ExplorationRange(0.3),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 503,
            name: "Galleons".to_string(),
            description: "Large ocean-going sailing ships".to_string(),
            era: TechEra::RenaissanceAge,
            category: TechCategory::Maritime,
            research_cost: 450.0,
            prerequisites: vec![405, 404],
            effects: vec![
                TechEffect::UnlockShip("Galleon".to_string()),
                TechEffect::NavalCombatBonus(0.3),
                TechEffect::TradeRangeBonus(0.4),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 504,
            name: "Nationalism".to_string(),
            description: "Strong identification with one's nation".to_string(),
            era: TechEra::RenaissanceAge,
            category: TechCategory::Government,
            research_cost: 400.0,
            prerequisites: vec![207, 500],
            effects: vec![
                TechEffect::CombatBonus(0.2),
                TechEffect::CultureBonus(0.2),
                TechEffect::UnlockGovernment("Nation-State".to_string()),
            ],
            discovery_year: None,
        });

        // =============== INDUSTRIAL AGE ===============
        self.add_tech(Technology {
            id: 600,
            name: "Steam Power".to_string(),
            description: "Harnessing steam for mechanical power".to_string(),
            era: TechEra::IndustrialAge,
            category: TechCategory::Industry,
            research_cost: 600.0,
            prerequisites: vec![301],
            effects: vec![
                TechEffect::ProductionBonus(0.5),
                TechEffect::MovementSpeedBonus(0.3),
                TechEffect::EnablesFeature("Factories".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 601,
            name: "Railroads".to_string(),
            description: "Steam-powered rail transport".to_string(),
            era: TechEra::IndustrialAge,
            category: TechCategory::Infrastructure,
            research_cost: 650.0,
            prerequisites: vec![600, 300],
            effects: vec![
                TechEffect::MovementSpeedBonus(0.5),
                TechEffect::TradeRangeBonus(0.4),
                TechEffect::ProductionBonus(0.2),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 602,
            name: "Rifling".to_string(),
            description: "Grooved gun barrels for accuracy".to_string(),
            era: TechEra::IndustrialAge,
            category: TechCategory::Military,
            research_cost: 550.0,
            prerequisites: vec![404],
            effects: vec![
                TechEffect::CombatBonus(0.4),
                TechEffect::UnlockUnit("Riflemen".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 603,
            name: "Electricity".to_string(),
            description: "Harnessing electrical power".to_string(),
            era: TechEra::IndustrialAge,
            category: TechCategory::Science,
            research_cost: 700.0,
            prerequisites: vec![600],
            effects: vec![
                TechEffect::ProductionBonus(0.4),
                TechEffect::ResearchBonus(0.3),
            ],
            discovery_year: None,
        });

        // =============== MODERN AGE ===============
        self.add_tech(Technology {
            id: 700,
            name: "Combustion Engine".to_string(),
            description: "Internal combustion for vehicles".to_string(),
            era: TechEra::ModernAge,
            category: TechCategory::Industry,
            research_cost: 800.0,
            prerequisites: vec![600],
            effects: vec![
                TechEffect::MovementSpeedBonus(0.6),
                TechEffect::UnlockUnit("Tanks".to_string()),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 701,
            name: "Flight".to_string(),
            description: "Powered heavier-than-air flight".to_string(),
            era: TechEra::ModernAge,
            category: TechCategory::Science,
            research_cost: 850.0,
            prerequisites: vec![700],
            effects: vec![
                TechEffect::MovementSpeedBonus(0.8),
                TechEffect::UnlockUnit("Aircraft".to_string()),
                TechEffect::CombatBonus(0.3),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 702,
            name: "Mass Media".to_string(),
            description: "Radio and television broadcasting".to_string(),
            era: TechEra::ModernAge,
            category: TechCategory::Culture,
            research_cost: 750.0,
            prerequisites: vec![603, 500],
            effects: vec![
                TechEffect::CultureBonus(0.5),
                TechEffect::DiplomacyBonus(0.3),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 703,
            name: "Computers".to_string(),
            description: "Electronic computation devices".to_string(),
            era: TechEra::ModernAge,
            category: TechCategory::Science,
            research_cost: 1000.0,
            prerequisites: vec![603, 502],
            effects: vec![
                TechEffect::ResearchBonus(0.6),
                TechEffect::ProductionBonus(0.3),
            ],
            discovery_year: None,
        });

        self.add_tech(Technology {
            id: 704,
            name: "Nuclear Fission".to_string(),
            description: "Splitting atoms for energy and weapons".to_string(),
            era: TechEra::ModernAge,
            category: TechCategory::Science,
            research_cost: 1200.0,
            prerequisites: vec![703],
            effects: vec![
                TechEffect::ProductionBonus(0.5),
                TechEffect::CombatBonus(0.5),
            ],
            discovery_year: None,
        });
    }

    pub fn get(&self, id: TechnologyId) -> Option<&Technology> {
        self.technologies.get(&id)
    }

    pub fn get_available(&self, researched: &HashSet<TechnologyId>) -> Vec<&Technology> {
        self.technologies
            .values()
            .filter(|tech| {
                !researched.contains(&tech.id)
                    && tech.prerequisites.iter().all(|pre| researched.contains(pre))
            })
            .collect()
    }

    pub fn get_starting_techs(&self) -> Vec<TechnologyId> {
        self.technologies
            .values()
            .filter(|tech| tech.prerequisites.is_empty())
            .map(|tech| tech.id)
            .collect()
    }
}

/// Nation's research state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchState {
    pub researched_technologies: HashSet<TechnologyId>,
    pub current_research: Option<TechnologyId>,
    pub research_progress: f32,
    pub accumulated_research_points: f32,
    pub current_era: TechEra,
}

impl ResearchState {
    pub fn new() -> Self {
        ResearchState {
            researched_technologies: HashSet::new(),
            current_research: None,
            research_progress: 0.0,
            accumulated_research_points: 0.0,
            current_era: TechEra::StoneAge,
        }
    }

    pub fn has_tech(&self, id: TechnologyId) -> bool {
        self.researched_technologies.contains(&id)
    }

    pub fn add_research_points(&mut self, points: f32, tech_tree: &TechTree) -> Vec<TechnologyId> {
        let mut completed = Vec::new();
        self.accumulated_research_points += points;

        if let Some(tech_id) = self.current_research {
            if let Some(tech) = tech_tree.get(tech_id) {
                self.research_progress += points;
                let cost = tech.research_cost * tech.era.research_modifier();

                if self.research_progress >= cost {
                    self.researched_technologies.insert(tech_id);
                    completed.push(tech_id);
                    self.current_research = None;
                    self.research_progress = 0.0;

                    // Update era
                    if tech.era > self.current_era {
                        self.current_era = tech.era;
                    }
                }
            }
        }

        completed
    }

    pub fn start_research(&mut self, tech_id: TechnologyId, tech_tree: &TechTree) -> bool {
        if let Some(tech) = tech_tree.get(tech_id) {
            if !self.researched_technologies.contains(&tech_id)
                && tech
                    .prerequisites
                    .iter()
                    .all(|pre| self.researched_technologies.contains(pre))
            {
                self.current_research = Some(tech_id);
                self.research_progress = 0.0;
                return true;
            }
        }
        false
    }

    pub fn calculate_bonuses(&self, tech_tree: &TechTree) -> TechBonuses {
        let mut bonuses = TechBonuses::default();

        for tech_id in &self.researched_technologies {
            if let Some(tech) = tech_tree.get(*tech_id) {
                for effect in &tech.effects {
                    match effect {
                        TechEffect::CombatBonus(v) => bonuses.combat += v,
                        TechEffect::DefenseBonus(v) => bonuses.defense += v,
                        TechEffect::GoldBonus(v) => bonuses.gold += v,
                        TechEffect::ProductionBonus(v) => bonuses.production += v,
                        TechEffect::FoodBonus(v) => bonuses.food += v,
                        TechEffect::FarmingEfficiency(v) => bonuses.farming += v,
                        TechEffect::PopulationGrowthBonus(v) => bonuses.pop_growth += v,
                        TechEffect::MovementSpeedBonus(v) => bonuses.movement += v,
                        TechEffect::TradeRangeBonus(v) => bonuses.trade_range += v,
                        TechEffect::ResearchBonus(v) => bonuses.research += v,
                        TechEffect::CultureBonus(v) => bonuses.culture += v,
                        TechEffect::FaithBonus(v) => bonuses.faith += v,
                        TechEffect::HappinessBonus(v) => bonuses.happiness += v,
                        TechEffect::NavalCombatBonus(v) => bonuses.naval_combat += v,
                        TechEffect::ExplorationRange(v) => bonuses.exploration += v,
                        TechEffect::SiegeBonus(v) => bonuses.siege += v,
                        TechEffect::AdministrativeEfficiency(v) => bonuses.admin += v,
                        TechEffect::DiplomacyBonus(v) => bonuses.diplomacy += v,
                        TechEffect::ConstructionSpeedBonus(v) => bonuses.construction += v,
                        _ => {}
                    }
                }
            }
        }

        bonuses
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TechBonuses {
    pub combat: f32,
    pub defense: f32,
    pub gold: f32,
    pub production: f32,
    pub food: f32,
    pub farming: f32,
    pub pop_growth: f32,
    pub movement: f32,
    pub trade_range: f32,
    pub research: f32,
    pub culture: f32,
    pub faith: f32,
    pub happiness: f32,
    pub naval_combat: f32,
    pub exploration: f32,
    pub siege: f32,
    pub admin: f32,
    pub diplomacy: f32,
    pub construction: f32,
}
