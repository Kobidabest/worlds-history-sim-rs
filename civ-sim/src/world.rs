use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f32::consts::{PI, TAU};

/// Biome types with habitability and resource characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiomeType {
    Ocean,
    DeepOcean,
    IceCap,
    Tundra,
    Taiga,
    Grassland,
    Savanna,
    Desert,
    Forest,
    Rainforest,
    Mountains,
    Hills,
    Wetlands,
    CoastalWaters,
}

impl BiomeType {
    pub fn habitability(&self) -> f32 {
        match self {
            BiomeType::Ocean => 0.0,
            BiomeType::DeepOcean => 0.0,
            BiomeType::IceCap => 0.02,
            BiomeType::Tundra => 0.15,
            BiomeType::Taiga => 0.35,
            BiomeType::Grassland => 0.9,
            BiomeType::Savanna => 0.7,
            BiomeType::Desert => 0.1,
            BiomeType::Forest => 0.75,
            BiomeType::Rainforest => 0.5,
            BiomeType::Mountains => 0.2,
            BiomeType::Hills => 0.6,
            BiomeType::Wetlands => 0.4,
            BiomeType::CoastalWaters => 0.0,
        }
    }

    pub fn agricultural_potential(&self) -> f32 {
        match self {
            BiomeType::Ocean => 0.0,
            BiomeType::DeepOcean => 0.0,
            BiomeType::IceCap => 0.0,
            BiomeType::Tundra => 0.05,
            BiomeType::Taiga => 0.2,
            BiomeType::Grassland => 1.0,
            BiomeType::Savanna => 0.6,
            BiomeType::Desert => 0.05,
            BiomeType::Forest => 0.5,
            BiomeType::Rainforest => 0.4,
            BiomeType::Mountains => 0.1,
            BiomeType::Hills => 0.4,
            BiomeType::Wetlands => 0.7,
            BiomeType::CoastalWaters => 0.0,
        }
    }

    pub fn resource_richness(&self) -> f32 {
        match self {
            BiomeType::Ocean => 0.5,
            BiomeType::DeepOcean => 0.3,
            BiomeType::IceCap => 0.1,
            BiomeType::Tundra => 0.3,
            BiomeType::Taiga => 0.6,
            BiomeType::Grassland => 0.7,
            BiomeType::Savanna => 0.5,
            BiomeType::Desert => 0.2,
            BiomeType::Forest => 0.8,
            BiomeType::Rainforest => 0.9,
            BiomeType::Mountains => 0.9,
            BiomeType::Hills => 0.7,
            BiomeType::Wetlands => 0.6,
            BiomeType::CoastalWaters => 0.7,
        }
    }

    pub fn movement_cost(&self) -> f32 {
        match self {
            BiomeType::Ocean => 1.0,
            BiomeType::DeepOcean => 1.0,
            BiomeType::IceCap => 4.0,
            BiomeType::Tundra => 2.0,
            BiomeType::Taiga => 1.8,
            BiomeType::Grassland => 1.0,
            BiomeType::Savanna => 1.1,
            BiomeType::Desert => 2.5,
            BiomeType::Forest => 1.5,
            BiomeType::Rainforest => 2.5,
            BiomeType::Mountains => 4.0,
            BiomeType::Hills => 1.8,
            BiomeType::Wetlands => 2.0,
            BiomeType::CoastalWaters => 1.0,
        }
    }

    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            BiomeType::Ocean => (28, 66, 120),
            BiomeType::DeepOcean => (15, 40, 90),
            BiomeType::IceCap => (240, 248, 255),
            BiomeType::Tundra => (139, 139, 128),
            BiomeType::Taiga => (43, 63, 40),
            BiomeType::Grassland => (154, 205, 50),
            BiomeType::Savanna => (210, 180, 90),
            BiomeType::Desert => (253, 225, 171),
            BiomeType::Forest => (34, 139, 34),
            BiomeType::Rainforest => (0, 100, 0),
            BiomeType::Mountains => (139, 137, 137),
            BiomeType::Hills => (107, 142, 35),
            BiomeType::Wetlands => (47, 79, 79),
            BiomeType::CoastalWaters => (64, 164, 223),
        }
    }
}

/// Natural resources that can be found on tiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    // Food resources
    Grain,
    Fish,
    Game,
    Cattle,
    Fruit,

    // Strategic resources
    Iron,
    Copper,
    Tin,
    Gold,
    Silver,
    Coal,
    Horses,
    Elephants,

    // Luxury resources
    Silk,
    Spices,
    Gems,
    Ivory,
    Incense,
    Dyes,
    Wine,
    Salt,
    Furs,
    Marble,

    // Building materials
    Stone,
    Wood,
    Clay,
}

impl ResourceType {
    pub fn base_value(&self) -> u32 {
        match self {
            ResourceType::Grain => 10,
            ResourceType::Fish => 8,
            ResourceType::Game => 6,
            ResourceType::Cattle => 12,
            ResourceType::Fruit => 8,
            ResourceType::Iron => 30,
            ResourceType::Copper => 20,
            ResourceType::Tin => 25,
            ResourceType::Gold => 100,
            ResourceType::Silver => 60,
            ResourceType::Coal => 25,
            ResourceType::Horses => 40,
            ResourceType::Elephants => 50,
            ResourceType::Silk => 80,
            ResourceType::Spices => 70,
            ResourceType::Gems => 90,
            ResourceType::Ivory => 60,
            ResourceType::Incense => 50,
            ResourceType::Dyes => 40,
            ResourceType::Wine => 35,
            ResourceType::Salt => 30,
            ResourceType::Furs => 35,
            ResourceType::Marble => 25,
            ResourceType::Stone => 10,
            ResourceType::Wood => 12,
            ResourceType::Clay => 8,
        }
    }

    pub fn is_strategic(&self) -> bool {
        matches!(
            self,
            ResourceType::Iron
                | ResourceType::Copper
                | ResourceType::Tin
                | ResourceType::Coal
                | ResourceType::Horses
                | ResourceType::Elephants
        )
    }

    pub fn is_luxury(&self) -> bool {
        matches!(
            self,
            ResourceType::Gold
                | ResourceType::Silver
                | ResourceType::Silk
                | ResourceType::Spices
                | ResourceType::Gems
                | ResourceType::Ivory
                | ResourceType::Incense
                | ResourceType::Dyes
                | ResourceType::Wine
                | ResourceType::Furs
                | ResourceType::Marble
        )
    }
}

/// A single terrain tile in the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub altitude: f32,
    pub temperature: f32,
    pub rainfall: f32,
    pub biome: BiomeType,
    pub resources: Vec<ResourceType>,
    pub fertility: f32,
    pub is_river: bool,
    pub is_coastal: bool,

    // Ownership and development
    pub owner_nation_id: Option<u64>,
    pub settlement_id: Option<u64>,
    pub population: u32,
    pub development: f32,
    pub infrastructure: f32,
}

impl Tile {
    pub fn new(x: u32, y: u32) -> Self {
        Tile {
            x,
            y,
            altitude: 0.0,
            temperature: 0.0,
            rainfall: 0.0,
            biome: BiomeType::Ocean,
            resources: Vec::new(),
            fertility: 0.0,
            is_river: false,
            is_coastal: false,
            owner_nation_id: None,
            settlement_id: None,
            population: 0,
            development: 0.0,
            infrastructure: 0.0,
        }
    }

    pub fn is_land(&self) -> bool {
        self.altitude > 0.0
    }

    pub fn carrying_capacity(&self) -> u32 {
        if !self.is_land() {
            return 0;
        }

        let base = self.biome.habitability() * 1000.0;
        let fertility_bonus = self.fertility * 500.0;
        let river_bonus = if self.is_river { 300.0 } else { 0.0 };
        let coastal_bonus = if self.is_coastal { 200.0 } else { 0.0 };
        let development_bonus = self.development * 2000.0;

        (base + fertility_bonus + river_bonus + coastal_bonus + development_bonus) as u32
    }

    pub fn food_production(&self) -> f32 {
        if !self.is_land() {
            return 0.0;
        }

        let base = self.biome.agricultural_potential() * 10.0;
        let fertility_bonus = self.fertility * 5.0;
        let river_bonus = if self.is_river { 3.0 } else { 0.0 };
        let development_bonus = self.development * 5.0;

        base + fertility_bonus + river_bonus + development_bonus
    }
}

/// The world map containing all terrain data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub width: u32,
    pub height: u32,
    pub seed: u32,
    pub tiles: Vec<Vec<Tile>>,
    pub land_tile_count: u32,
    pub total_land_area: f32,
}

impl World {
    pub fn new(width: u32, height: u32, seed: u32) -> Self {
        let mut world = World {
            width,
            height,
            seed,
            tiles: Vec::new(),
            land_tile_count: 0,
            total_land_area: 0.0,
        };

        // Initialize tiles
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(Tile::new(x, y));
            }
            world.tiles.push(row);
        }

        // Generate world
        let mut rng = SmallRng::seed_from_u64(seed as u64);
        world.generate_terrain(&mut rng);
        world.generate_climate(&mut rng);
        world.generate_biomes();
        world.generate_rivers(&mut rng);
        world.mark_coastal_tiles();
        world.generate_resources(&mut rng);
        world.calculate_fertility();
        world.count_land_tiles();

        world
    }

    fn generate_terrain(&mut self, rng: &mut SmallRng) {
        // Generate continents using multiple octaves of noise
        let num_continents = 6 + rng.gen_range(0..5);
        let mut continent_centers: Vec<(f32, f32, f32)> = Vec::new();

        for _ in 0..num_continents {
            let cx = rng.gen_range(0.0..self.width as f32);
            let cy = rng.gen_range(self.height as f32 * 0.15..self.height as f32 * 0.85);
            let size = rng.gen_range(0.08..0.2);
            continent_centers.push((cx, cy, size));
        }

        // Generate altitude using Perlin-like noise simulation
        for y in 0..self.height {
            for x in 0..self.width {
                let mut altitude: f32 = -1.0;

                // Continental influence
                for (cx, cy, size) in &continent_centers {
                    let dx = self.wrap_distance_x(x as f32, *cx);
                    let dy = (y as f32 - cy).abs();
                    let dist = (dx * dx + dy * dy).sqrt() / (self.width as f32 * size);
                    let influence = (1.0 - dist).max(0.0).powf(1.5);
                    altitude = altitude.max(-1.0 + influence * 2.5);
                }

                // Add noise octaves
                altitude += self.noise(x as f32, y as f32, 0.02, rng) * 0.5;
                altitude += self.noise(x as f32, y as f32, 0.05, rng) * 0.25;
                altitude += self.noise(x as f32, y as f32, 0.1, rng) * 0.15;
                altitude += self.noise(x as f32, y as f32, 0.2, rng) * 0.1;

                // Mountain ranges
                let mountain_noise = self.ridge_noise(x as f32, y as f32, 0.03, rng);
                if altitude > 0.1 && mountain_noise > 0.6 {
                    altitude += (mountain_noise - 0.6) * 3.0;
                }

                self.tiles[y as usize][x as usize].altitude = altitude.clamp(-1.0, 2.0);
            }
        }
    }

    fn generate_climate(&mut self, rng: &mut SmallRng) {
        for y in 0..self.height {
            let latitude = (y as f32 / self.height as f32 - 0.5).abs() * 2.0;

            for x in 0..self.width {
                // Get altitude first (immutable borrow)
                let altitude = self.tiles[y as usize][x as usize].altitude;

                // Calculate values using self methods before mutable borrow
                let noise_temp = self.noise(x as f32, y as f32, 0.05, rng) * 5.0;
                let noise_rain = self.noise(x as f32, y as f32, 0.03, rng) * 0.3;

                // Temperature based on latitude and altitude
                let base_temp = 30.0 - latitude * 60.0;
                let altitude_penalty = altitude.max(0.0) * 20.0;
                let temperature = (base_temp - altitude_penalty + noise_temp).clamp(-40.0, 45.0);

                // Rainfall based on latitude and terrain
                let equator_dist = (y as f32 / self.height as f32 - 0.5).abs();
                let tropical_rain = if equator_dist < 0.15 { 1.0 } else { 0.0 };
                let temperate_rain = if equator_dist > 0.25 && equator_dist < 0.45 {
                    1.0 - (equator_dist - 0.35).abs() * 5.0
                } else {
                    0.0
                };

                let base_rain = (tropical_rain + temperate_rain) * 0.7 + 0.3;
                let coastal_bonus = if altitude <= 0.1 && altitude > 0.0 {
                    0.2
                } else {
                    0.0
                };
                let mountain_shadow = if altitude > 0.8 { -0.3 } else { 0.0 };

                let rainfall =
                    ((base_rain + coastal_bonus + mountain_shadow + noise_rain) * 3000.0)
                        .clamp(0.0, 4000.0);

                // Now do mutable borrow and assign
                let tile = &mut self.tiles[y as usize][x as usize];
                tile.temperature = temperature;
                tile.rainfall = rainfall;
            }
        }
    }

    fn generate_biomes(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &mut self.tiles[y as usize][x as usize];

                tile.biome = if tile.altitude < -0.3 {
                    BiomeType::DeepOcean
                } else if tile.altitude < 0.0 {
                    BiomeType::Ocean
                } else if tile.altitude < 0.05 {
                    BiomeType::CoastalWaters
                } else if tile.temperature < -15.0 {
                    BiomeType::IceCap
                } else if tile.temperature < -5.0 {
                    if tile.rainfall > 500.0 {
                        BiomeType::Taiga
                    } else {
                        BiomeType::Tundra
                    }
                } else if tile.altitude > 1.2 {
                    BiomeType::Mountains
                } else if tile.altitude > 0.6 {
                    BiomeType::Hills
                } else if tile.rainfall < 300.0 {
                    BiomeType::Desert
                } else if tile.rainfall < 800.0 {
                    if tile.temperature > 20.0 {
                        BiomeType::Savanna
                    } else {
                        BiomeType::Grassland
                    }
                } else if tile.rainfall > 2500.0 && tile.temperature > 20.0 {
                    BiomeType::Rainforest
                } else if tile.rainfall > 1500.0 {
                    if tile.temperature > 15.0 {
                        BiomeType::Forest
                    } else {
                        BiomeType::Taiga
                    }
                } else if tile.rainfall > 1000.0 && tile.altitude < 0.2 {
                    BiomeType::Wetlands
                } else if tile.temperature > 18.0 {
                    BiomeType::Savanna
                } else {
                    BiomeType::Grassland
                };
            }
        }
    }

    fn generate_rivers(&mut self, rng: &mut SmallRng) {
        let num_rivers = (self.width * self.height / 2000) as usize;

        for _ in 0..num_rivers {
            // Start from high altitude
            let mut best_start = None;
            let mut best_altitude = 0.0;

            for _ in 0..20 {
                let x = rng.gen_range(0..self.width);
                let y = rng.gen_range(0..self.height);
                let tile = &self.tiles[y as usize][x as usize];
                if tile.altitude > best_altitude && tile.altitude < 1.5 {
                    best_altitude = tile.altitude;
                    best_start = Some((x, y));
                }
            }

            if let Some((start_x, start_y)) = best_start {
                let mut x = start_x;
                let mut y = start_y;
                let mut steps = 0;

                while steps < 200 {
                    // Get altitude value first (copy, not borrow)
                    let current_altitude = self.tiles[y as usize][x as usize].altitude;
                    if current_altitude <= 0.0 {
                        break;
                    }

                    // Mark as river
                    self.tiles[y as usize][x as usize].is_river = true;

                    // Find lowest neighbor
                    let mut lowest_alt = current_altitude;
                    let mut next_x = x;
                    let mut next_y = y;

                    for (dx, dy) in &[(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
                        let nx = ((x as i32 + dx + self.width as i32) % self.width as i32) as u32;
                        let ny = (y as i32 + dy).clamp(0, self.height as i32 - 1) as u32;
                        let neighbor_alt = self.tiles[ny as usize][nx as usize].altitude;

                        if neighbor_alt < lowest_alt {
                            lowest_alt = neighbor_alt;
                            next_x = nx;
                            next_y = ny;
                        }
                    }

                    if next_x == x && next_y == y {
                        break;
                    }

                    x = next_x;
                    y = next_y;
                    steps += 1;
                }
            }
        }
    }

    fn mark_coastal_tiles(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if !self.tiles[y as usize][x as usize].is_land() {
                    continue;
                }

                let mut is_coastal = false;
                for (dx, dy) in &[(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
                    let nx = ((x as i32 + dx + self.width as i32) % self.width as i32) as u32;
                    let ny = (y as i32 + dy).clamp(0, self.height as i32 - 1) as u32;
                    if !self.tiles[ny as usize][nx as usize].is_land() {
                        is_coastal = true;
                        break;
                    }
                }

                self.tiles[y as usize][x as usize].is_coastal = is_coastal;
            }
        }
    }

    fn generate_resources(&mut self, rng: &mut SmallRng) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y as usize][x as usize];
                let biome = tile.biome;
                let altitude = tile.altitude;
                let is_coastal = tile.is_coastal;
                let is_river = tile.is_river;

                let mut resources = Vec::new();

                // Generate resources based on biome
                match biome {
                    BiomeType::Ocean | BiomeType::DeepOcean | BiomeType::CoastalWaters => {
                        if rng.gen_bool(0.3) {
                            resources.push(ResourceType::Fish);
                        }
                    }
                    BiomeType::Grassland => {
                        if rng.gen_bool(0.4) {
                            resources.push(ResourceType::Grain);
                        }
                        if rng.gen_bool(0.2) {
                            resources.push(ResourceType::Cattle);
                        }
                        if rng.gen_bool(0.15) {
                            resources.push(ResourceType::Horses);
                        }
                    }
                    BiomeType::Forest => {
                        if rng.gen_bool(0.5) {
                            resources.push(ResourceType::Wood);
                        }
                        if rng.gen_bool(0.2) {
                            resources.push(ResourceType::Game);
                        }
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Furs);
                        }
                    }
                    BiomeType::Rainforest => {
                        if rng.gen_bool(0.3) {
                            resources.push(ResourceType::Wood);
                        }
                        if rng.gen_bool(0.15) {
                            resources.push(ResourceType::Spices);
                        }
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Fruit);
                        }
                        if rng.gen_bool(0.08) {
                            resources.push(ResourceType::Gems);
                        }
                    }
                    BiomeType::Desert => {
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Gold);
                        }
                        if rng.gen_bool(0.15) {
                            resources.push(ResourceType::Salt);
                        }
                        if rng.gen_bool(0.05) {
                            resources.push(ResourceType::Incense);
                        }
                    }
                    BiomeType::Mountains => {
                        if rng.gen_bool(0.3) {
                            resources.push(ResourceType::Stone);
                        }
                        if rng.gen_bool(0.2) {
                            resources.push(ResourceType::Iron);
                        }
                        if rng.gen_bool(0.15) {
                            resources.push(ResourceType::Copper);
                        }
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Gold);
                        }
                        if rng.gen_bool(0.08) {
                            resources.push(ResourceType::Silver);
                        }
                        if rng.gen_bool(0.12) {
                            resources.push(ResourceType::Gems);
                        }
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Marble);
                        }
                    }
                    BiomeType::Hills => {
                        if rng.gen_bool(0.25) {
                            resources.push(ResourceType::Stone);
                        }
                        if rng.gen_bool(0.15) {
                            resources.push(ResourceType::Iron);
                        }
                        if rng.gen_bool(0.2) {
                            resources.push(ResourceType::Copper);
                        }
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Tin);
                        }
                        if rng.gen_bool(0.15) {
                            resources.push(ResourceType::Wine);
                        }
                    }
                    BiomeType::Savanna => {
                        if rng.gen_bool(0.2) {
                            resources.push(ResourceType::Cattle);
                        }
                        if rng.gen_bool(0.15) {
                            resources.push(ResourceType::Game);
                        }
                        if rng.gen_bool(0.05) {
                            resources.push(ResourceType::Elephants);
                        }
                        if rng.gen_bool(0.05) {
                            resources.push(ResourceType::Ivory);
                        }
                    }
                    BiomeType::Taiga => {
                        if rng.gen_bool(0.4) {
                            resources.push(ResourceType::Wood);
                        }
                        if rng.gen_bool(0.3) {
                            resources.push(ResourceType::Furs);
                        }
                        if rng.gen_bool(0.15) {
                            resources.push(ResourceType::Game);
                        }
                    }
                    BiomeType::Tundra => {
                        if rng.gen_bool(0.2) {
                            resources.push(ResourceType::Furs);
                        }
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Game);
                        }
                    }
                    BiomeType::Wetlands => {
                        if rng.gen_bool(0.3) {
                            resources.push(ResourceType::Fish);
                        }
                        if rng.gen_bool(0.2) {
                            resources.push(ResourceType::Clay);
                        }
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Dyes);
                        }
                    }
                    BiomeType::IceCap => {
                        if rng.gen_bool(0.1) {
                            resources.push(ResourceType::Fish);
                        }
                    }
                }

                // Coastal resources
                if is_coastal && rng.gen_bool(0.25) {
                    resources.push(ResourceType::Fish);
                }

                // River resources
                if is_river {
                    if rng.gen_bool(0.2) {
                        resources.push(ResourceType::Fish);
                    }
                    if rng.gen_bool(0.1) {
                        resources.push(ResourceType::Clay);
                    }
                }

                // Silk in specific regions (far east simulation)
                if x > self.width * 3 / 4 && rng.gen_bool(0.03) {
                    if matches!(biome, BiomeType::Forest | BiomeType::Grassland) {
                        resources.push(ResourceType::Silk);
                    }
                }

                self.tiles[y as usize][x as usize].resources = resources;
            }
        }
    }

    fn calculate_fertility(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &mut self.tiles[y as usize][x as usize];

                if !tile.is_land() {
                    tile.fertility = 0.0;
                    continue;
                }

                let base = tile.biome.agricultural_potential();
                let temp_factor = if tile.temperature > 10.0 && tile.temperature < 30.0 {
                    1.0
                } else {
                    0.5
                };
                let rain_factor = (tile.rainfall / 1500.0).min(1.5);
                let river_bonus = if tile.is_river { 0.3 } else { 0.0 };

                tile.fertility = (base * temp_factor * rain_factor + river_bonus).min(1.0);
            }
        }
    }

    fn count_land_tiles(&mut self) {
        self.land_tile_count = 0;
        for row in &self.tiles {
            for tile in row {
                if tile.is_land() {
                    self.land_tile_count += 1;
                }
            }
        }
        self.total_land_area = self.land_tile_count as f32;
    }

    // Utility functions for noise generation
    fn noise(&self, x: f32, y: f32, scale: f32, rng: &mut SmallRng) -> f32 {
        // Simplified noise using the seed
        let ix = (x * scale) as i32;
        let iy = (y * scale) as i32;
        let hash = self.hash_coords(ix, iy);
        (hash as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    fn ridge_noise(&self, x: f32, y: f32, scale: f32, rng: &mut SmallRng) -> f32 {
        let n = self.noise(x, y, scale, rng);
        1.0 - n.abs()
    }

    fn hash_coords(&self, x: i32, y: i32) -> u32 {
        let mut h = self.seed;
        h = h.wrapping_add(x as u32).wrapping_mul(0x85ebca6b);
        h ^= h >> 13;
        h = h.wrapping_add(y as u32).wrapping_mul(0xc2b2ae35);
        h ^= h >> 16;
        h
    }

    fn wrap_distance_x(&self, x1: f32, x2: f32) -> f32 {
        let direct = (x1 - x2).abs();
        let wrapped = self.width as f32 - direct;
        direct.min(wrapped)
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        self.tiles.get(y as usize)?.get(x as usize)
    }

    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        self.tiles.get_mut(y as usize)?.get_mut(x as usize)
    }

    pub fn get_neighbors(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut neighbors = Vec::new();

        for (dx, dy) in &[(-1i32, 0), (1, 0), (0, -1i32), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)]
        {
            let nx = ((x as i32 + dx + self.width as i32) % self.width as i32) as u32;
            let ny = (y as i32 + dy).clamp(0, self.height as i32 - 1) as u32;
            neighbors.push((nx, ny));
        }

        neighbors
    }

    pub fn find_suitable_settlement_locations(&self) -> Vec<(u32, u32, f32)> {
        let mut locations = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y as usize][x as usize];
                if !tile.is_land() {
                    continue;
                }

                let mut score = tile.biome.habitability() * 30.0;
                score += tile.fertility * 20.0;
                if tile.is_river {
                    score += 25.0;
                }
                if tile.is_coastal {
                    score += 15.0;
                }
                score += tile.resources.len() as f32 * 5.0;

                if score > 20.0 {
                    locations.push((x, y, score));
                }
            }
        }

        locations.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        locations
    }
}
