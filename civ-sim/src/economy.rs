use crate::civilization::{NationId, SettlementId};
use crate::world::ResourceType;
use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type TradeRouteId = u64;
pub type MarketId = u64;

/// Types of economic activities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EconomicActivity {
    Agriculture,
    Mining,
    Fishing,
    Forestry,
    Herding,
    Crafting,
    Trading,
    Banking,
    Manufacturing,
}

/// Trade goods with price and demand characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeGood {
    pub resource_type: ResourceType,
    pub base_price: f32,
    pub current_price: f32,
    pub supply: u32,
    pub demand: u32,
    pub is_luxury: bool,
    pub is_strategic: bool,
}

impl TradeGood {
    pub fn new(resource_type: ResourceType) -> Self {
        let base_price = resource_type.base_value() as f32;
        TradeGood {
            resource_type,
            base_price,
            current_price: base_price,
            supply: 0,
            demand: 0,
            is_luxury: resource_type.is_luxury(),
            is_strategic: resource_type.is_strategic(),
        }
    }

    pub fn update_price(&mut self) {
        // Price based on supply/demand
        if self.supply > 0 && self.demand > 0 {
            let ratio = self.demand as f32 / self.supply as f32;
            self.current_price = self.base_price * ratio.clamp(0.5, 3.0);
        } else if self.supply == 0 {
            self.current_price = self.base_price * 3.0;
        } else {
            self.current_price = self.base_price * 0.5;
        }
    }
}

/// A trade route between two settlements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRoute {
    pub id: TradeRouteId,
    pub source_id: SettlementId,
    pub target_id: SettlementId,
    pub source_nation: NationId,
    pub target_nation: NationId,
    pub is_maritime: bool,
    pub distance: f32,
    pub established_year: i32,

    pub goods_traded: Vec<(ResourceType, u32)>,
    pub gold_flow: f32, // Positive = into source
    pub trade_value: f32,
    pub efficiency: f32,
    pub is_active: bool,
}

impl TradeRoute {
    pub fn new(
        id: TradeRouteId,
        source_id: SettlementId,
        target_id: SettlementId,
        source_nation: NationId,
        target_nation: NationId,
        is_maritime: bool,
        distance: f32,
        established_year: i32,
    ) -> Self {
        TradeRoute {
            id,
            source_id,
            target_id,
            source_nation,
            target_nation,
            is_maritime,
            distance,
            established_year,
            goods_traded: Vec::new(),
            gold_flow: 0.0,
            trade_value: 0.0,
            efficiency: 1.0,
            is_active: true,
        }
    }

    pub fn calculate_trade_value(&self) -> f32 {
        let base_value: f32 = self.goods_traded.iter().map(|(_, q)| *q as f32).sum();
        let distance_penalty = 1.0 / (1.0 + self.distance * 0.01);
        base_value * distance_penalty * self.efficiency
    }
}

/// Market in a settlement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: MarketId,
    pub settlement_id: SettlementId,
    pub goods: HashMap<ResourceType, TradeGood>,
    pub trade_routes: Vec<TradeRouteId>,
    pub gold_reserve: f32,
    pub trade_volume: f32,
    pub market_reach: f32, // How far trade influence extends
}

impl Market {
    pub fn new(id: MarketId, settlement_id: SettlementId) -> Self {
        Market {
            id,
            settlement_id,
            goods: HashMap::new(),
            trade_routes: Vec::new(),
            gold_reserve: 0.0,
            trade_volume: 0.0,
            market_reach: 10.0,
        }
    }

    pub fn add_good(&mut self, resource_type: ResourceType, amount: u32) {
        let good = self
            .goods
            .entry(resource_type)
            .or_insert_with(|| TradeGood::new(resource_type));
        good.supply += amount;
    }

    pub fn consume_good(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        if let Some(good) = self.goods.get_mut(&resource_type) {
            let consumed = amount.min(good.supply);
            good.supply -= consumed;
            good.demand += amount;
            consumed
        } else {
            0
        }
    }

    pub fn get_price(&self, resource_type: ResourceType) -> f32 {
        self.goods
            .get(&resource_type)
            .map(|g| g.current_price)
            .unwrap_or(resource_type.base_value() as f32)
    }

    pub fn update_prices(&mut self) {
        for good in self.goods.values_mut() {
            good.update_price();
        }
    }

    pub fn tick(&mut self) {
        self.update_prices();
        self.trade_volume = self
            .goods
            .values()
            .map(|g| g.supply as f32 * g.current_price)
            .sum();

        // Decay demand
        for good in self.goods.values_mut() {
            good.demand = (good.demand as f32 * 0.9) as u32;
        }
    }
}

/// Economic policy of a nation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EconomicPolicy {
    FreeTrade,
    Mercantilism,
    Protectionism,
    StateControl,
    LaissezFaire,
}

impl EconomicPolicy {
    pub fn trade_modifier(&self) -> f32 {
        match self {
            EconomicPolicy::FreeTrade => 1.3,
            EconomicPolicy::Mercantilism => 1.0,
            EconomicPolicy::Protectionism => 0.7,
            EconomicPolicy::StateControl => 0.8,
            EconomicPolicy::LaissezFaire => 1.2,
        }
    }

    pub fn production_modifier(&self) -> f32 {
        match self {
            EconomicPolicy::FreeTrade => 1.0,
            EconomicPolicy::Mercantilism => 1.1,
            EconomicPolicy::Protectionism => 1.2,
            EconomicPolicy::StateControl => 1.15,
            EconomicPolicy::LaissezFaire => 1.0,
        }
    }

    pub fn tariff_rate(&self) -> f32 {
        match self {
            EconomicPolicy::FreeTrade => 0.05,
            EconomicPolicy::Mercantilism => 0.2,
            EconomicPolicy::Protectionism => 0.4,
            EconomicPolicy::StateControl => 0.3,
            EconomicPolicy::LaissezFaire => 0.1,
        }
    }
}

/// National economy statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NationalEconomy {
    pub gdp: f32,
    pub gdp_per_capita: f32,
    pub inflation: f32,
    pub trade_balance: f32,
    pub debt: f32,
    pub policy: Option<EconomicPolicy>,

    pub total_production: f32,
    pub total_trade_income: f32,
    pub total_tax_income: f32,
    pub total_expenses: f32,

    pub resource_production: HashMap<ResourceType, u32>,
    pub resource_consumption: HashMap<ResourceType, u32>,
    pub resource_surplus: HashMap<ResourceType, i32>,
}

impl NationalEconomy {
    pub fn new() -> Self {
        NationalEconomy {
            policy: Some(EconomicPolicy::Mercantilism),
            ..Default::default()
        }
    }

    pub fn calculate_gdp(&mut self, population: u32) {
        self.gdp = self.total_production + self.total_trade_income;
        self.gdp_per_capita = if population > 0 {
            self.gdp / population as f32
        } else {
            0.0
        };
    }

    pub fn calculate_balance(&mut self) {
        self.trade_balance = self.total_trade_income - self.total_expenses;
    }

    pub fn update_inflation(&mut self, money_supply_change: f32) {
        self.inflation = (self.inflation * 0.95 + money_supply_change * 0.1).clamp(-0.1, 0.5);
    }
}

/// Trade manager for the simulation
pub struct EconomyManager {
    pub markets: HashMap<MarketId, Market>,
    pub trade_routes: HashMap<TradeRouteId, TradeRoute>,
    pub national_economies: HashMap<NationId, NationalEconomy>,
    pub next_market_id: MarketId,
    pub next_route_id: TradeRouteId,
    pub global_trade_volume: f32,
}

impl EconomyManager {
    pub fn new() -> Self {
        EconomyManager {
            markets: HashMap::new(),
            trade_routes: HashMap::new(),
            national_economies: HashMap::new(),
            next_market_id: 1,
            next_route_id: 1,
            global_trade_volume: 0.0,
        }
    }

    pub fn create_market(&mut self, settlement_id: SettlementId) -> MarketId {
        let id = self.next_market_id;
        self.next_market_id += 1;

        let market = Market::new(id, settlement_id);
        self.markets.insert(id, market);
        id
    }

    pub fn create_trade_route(
        &mut self,
        source_id: SettlementId,
        target_id: SettlementId,
        source_nation: NationId,
        target_nation: NationId,
        is_maritime: bool,
        distance: f32,
        current_year: i32,
    ) -> TradeRouteId {
        let id = self.next_route_id;
        self.next_route_id += 1;

        let route = TradeRoute::new(
            id,
            source_id,
            target_id,
            source_nation,
            target_nation,
            is_maritime,
            distance,
            current_year,
        );
        self.trade_routes.insert(id, route);
        id
    }

    pub fn get_or_create_national_economy(&mut self, nation_id: NationId) -> &mut NationalEconomy {
        self.national_economies
            .entry(nation_id)
            .or_insert_with(NationalEconomy::new)
    }

    pub fn process_trade_route(&mut self, route_id: TradeRouteId) {
        if let Some(route) = self.trade_routes.get(&route_id) {
            let trade_value = route.calculate_trade_value();

            // Update national economies
            if let Some(source_econ) = self.national_economies.get_mut(&route.source_nation) {
                source_econ.total_trade_income += trade_value * 0.5;
            }
            if let Some(target_econ) = self.national_economies.get_mut(&route.target_nation) {
                target_econ.total_trade_income += trade_value * 0.5;
            }
        }
    }

    pub fn tick(&mut self) {
        // Update all markets
        for market in self.markets.values_mut() {
            market.tick();
        }

        // Process trade routes
        let route_ids: Vec<TradeRouteId> = self.trade_routes.keys().cloned().collect();
        for route_id in route_ids {
            self.process_trade_route(route_id);
        }

        // Calculate global trade volume
        self.global_trade_volume = self.markets.values().map(|m| m.trade_volume).sum();

        // Update national economies
        for economy in self.national_economies.values_mut() {
            economy.calculate_balance();
        }
    }

    /// Find potential trade partners for a settlement
    pub fn find_trade_partners(
        &self,
        settlement_id: SettlementId,
        nation_id: NationId,
        max_distance: f32,
    ) -> Vec<(SettlementId, f32)> {
        // This would be implemented with actual settlement positions
        Vec::new()
    }
}

/// Economic events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EconomicEvent {
    TradeRouteEstablished {
        source_id: SettlementId,
        target_id: SettlementId,
        route_id: TradeRouteId,
    },
    TradeRouteClosed {
        route_id: TradeRouteId,
        reason: String,
    },
    MarketCreated {
        settlement_id: SettlementId,
        market_id: MarketId,
    },
    PriceChange {
        resource: ResourceType,
        old_price: f32,
        new_price: f32,
        settlement_id: SettlementId,
    },
    TradeAgreement {
        nation_a: NationId,
        nation_b: NationId,
    },
    Embargo {
        imposer: NationId,
        target: NationId,
    },
    EconomicCrisis {
        nation_id: NationId,
        severity: f32,
    },
    GoldDiscovery {
        settlement_id: SettlementId,
        amount: u32,
    },
}

/// Calculate distance between two points for trade
pub fn calculate_trade_distance(
    pos1: (u32, u32),
    pos2: (u32, u32),
    world_width: u32,
    is_land: impl Fn(u32, u32) -> bool,
) -> f32 {
    let dx = {
        let direct = (pos1.0 as f32 - pos2.0 as f32).abs();
        let wrapped = world_width as f32 - direct;
        direct.min(wrapped)
    };
    let dy = (pos1.1 as f32 - pos2.1 as f32).abs();
    (dx * dx + dy * dy).sqrt()
}
