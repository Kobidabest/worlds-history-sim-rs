use crate::{
    biome::{BiomeStats, BiomeType},
    math_util::{cartesian_coordinates, mix_values, random_point_in_sphere, repeat, Vec3},
    perlin,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f32::consts::{PI, TAU};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainCell {
    pub altitude: f32,
    pub rainfall: f32,
    pub temperature: f32,
    pub biome_presences: Vec<(BiomeType, f32)>,
}

impl Default for TerrainCell {
    fn default() -> Self {
        Self {
            altitude: 0.0,
            rainfall: 0.0,
            temperature: 0.0,
            biome_presences: Vec::new(),
        }
    }
}

impl TerrainCell {
    /// Returns the dominant biome for this cell.
    pub fn dominant_biome(&self) -> BiomeType {
        self.biome_presences
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(bt, _)| *bt)
            .unwrap_or(BiomeType::Ocean)
    }

    pub fn is_land(&self) -> bool {
        self.altitude > 0.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub width: u32,
    pub height: u32,
    pub seed: u32,
    pub terrain: Vec<Vec<TerrainCell>>,
    continent_offsets: Vec<[f32; 2]>,
    continent_sizes: Vec<[f32; 2]>,
}

impl World {
    pub const MAX_ALTITUDE: f32 = 15000.0;
    pub const MIN_ALTITUDE: f32 = -15000.0;
    pub const MAX_RAINFALL: f32 = 13000.0;
    pub const MIN_RAINFALL: f32 = 0.0;
    pub const MAX_TEMPERATURE: f32 = 30.0;
    pub const MIN_TEMPERATURE: f32 = -35.0;
    pub const NUM_CONTINENTS: usize = 12;

    const ALTITUDE_SPAN: f32 = Self::MAX_ALTITUDE - Self::MIN_ALTITUDE;
    const RAINFALL_SPAN: f32 = Self::MAX_RAINFALL - Self::MIN_RAINFALL;
    const RAINFALL_DRYNESS_FACTOR: f32 = 0.005;
    const RAINFALL_DRYNESS_OFFSET: f32 = Self::RAINFALL_DRYNESS_FACTOR * Self::MAX_RAINFALL;
    const TEMPERATURE_SPAN: f32 = Self::MAX_TEMPERATURE - Self::MIN_TEMPERATURE;
    const TEMPERATURE_ALTITUDE_FACTOR: f32 = 2.05;
    const CONTINENT_MAX_SIZE_FACTOR: f32 = 8.7;
    const CONTINENT_MIN_SIZE_FACTOR: f32 = 5.7;

    pub fn generate(width: u32, height: u32, seed: u32) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed as u64);
        let terrain = vec![vec![TerrainCell::default(); width as usize]; height as usize];
        let continent_offsets = vec![[0.0, 0.0]; Self::NUM_CONTINENTS];
        let continent_sizes = vec![[0.0, 0.0]; Self::NUM_CONTINENTS];

        let mut world = World {
            width,
            height,
            seed,
            terrain,
            continent_offsets,
            continent_sizes,
        };

        world.generate_continents(&mut rng);
        world.generate_altitude(&mut rng);
        world.generate_rainfall(&mut rng);
        world.generate_temperature(&mut rng);
        world.generate_biomes();

        world
    }

    fn generate_continents(&mut self, rng: &mut SmallRng) {
        let width = self.width as f32;
        let height = self.height as f32;

        const LONGITUDE_FACTOR: f32 = 15.0;
        const LATITUDE_FACTOR: f32 = 6.0;

        let mut prev_x = rng.gen_range(0.0..width * (LONGITUDE_FACTOR - 1.0) / LONGITUDE_FACTOR);
        let mut prev_y =
            rng.gen_range(height / LATITUDE_FACTOR..height * (LATITUDE_FACTOR - 1.0) / LATITUDE_FACTOR);

        for i in 0..Self::NUM_CONTINENTS {
            let width_offset: f32 = rng.gen_range(0.0..6.0);

            self.continent_offsets[i] = [prev_x, prev_y];
            self.continent_sizes[i] = [
                rng.gen_range(
                    Self::CONTINENT_MIN_SIZE_FACTOR + width_offset
                        ..Self::CONTINENT_MAX_SIZE_FACTOR + width_offset,
                ),
                rng.gen_range(
                    Self::CONTINENT_MIN_SIZE_FACTOR + width_offset
                        ..Self::CONTINENT_MAX_SIZE_FACTOR + width_offset,
                ),
            ];

            let y_position = rng.gen_range(
                height / LATITUDE_FACTOR..height * (LATITUDE_FACTOR - 1.0) / LATITUDE_FACTOR,
            );

            let new_x = if i % 3 == 2 {
                repeat(
                    prev_x + rng.gen_range(width * 4.0 / LONGITUDE_FACTOR..width * 6.0 / LONGITUDE_FACTOR),
                    width,
                )
            } else {
                repeat(
                    prev_x + rng.gen_range(width / LONGITUDE_FACTOR..width * 2.0 / LONGITUDE_FACTOR),
                    width,
                )
            };

            prev_x = new_x;
            prev_y = y_position;
        }
    }

    fn continent_distance(&self, continent_num: usize, x: usize, y: usize) -> f32 {
        let beta_factor = (PI * y as f32 / self.height as f32).sin();

        let [continent_x, continent_y] = self.continent_offsets[continent_num];

        let distance_x = (continent_x - x as f32)
            .abs()
            .min((self.width as f32 + continent_x - x as f32).abs())
            .min((continent_x - x as f32 - self.width as f32).abs())
            * beta_factor;

        let distance_y = (continent_y - y as f32).abs();

        let [continent_width, continent_height] = self.continent_sizes[continent_num];

        ((distance_x * continent_width).powi(2) + (distance_y * continent_height).powi(2)).sqrt()
    }

    fn continent_modifier(&self, x: usize, y: usize) -> f32 {
        let mut max_value: f32 = 0.0;

        for i in 0..Self::NUM_CONTINENTS {
            let distance = self.continent_distance(i, x, y);
            let value = (1.0 - distance / self.width as f32).clamp(0.0, 1.0);

            let mut other_value = value;

            if value > max_value {
                other_value = max_value;
                max_value = value;
            }

            let value_mod = (other_value * 2.0).min(1.0);
            max_value = mix_values(max_value, other_value, value_mod);
        }

        max_value
    }

    fn random_offset_vector(rng: &mut SmallRng) -> Vec3 {
        random_point_in_sphere(rng, 1000.0)
    }

    fn random_noise_from_polar(alpha: f32, beta: f32, radius: f32, offset: Vec3) -> f32 {
        let c = cartesian_coordinates(alpha, beta, radius);
        perlin::perlin_value(c.x + offset.x, c.y + offset.y, c.z + offset.z)
    }

    fn mountain_range_noise(noise: f32, width_factor: f32) -> f32 {
        let noise = noise * 2.0 - 1.0;
        let v1 = -(noise * width_factor + 1.0).powi(2).exp().copysign(1.0);
        let v2 = (-(noise * width_factor - 1.0).powi(2)).exp();
        (v1 + v2 + 1.0) / 2.0
    }

    fn generate_altitude(&mut self, rng: &mut SmallRng) {
        const R1: f32 = 0.75;
        const R2: f32 = 8.0;
        const R3: f32 = 4.0;
        const R4: f32 = 8.0;
        const R5: f32 = 16.0;
        const R6: f32 = 64.0;
        const R7: f32 = 128.0;
        const R8: f32 = 1.5;
        const R9: f32 = 1.0;

        let o1 = Self::random_offset_vector(rng);
        let o1b = Self::random_offset_vector(rng);
        let o2 = Self::random_offset_vector(rng);
        let o2b = Self::random_offset_vector(rng);
        let o3 = Self::random_offset_vector(rng);
        let o4 = Self::random_offset_vector(rng);
        let o5 = Self::random_offset_vector(rng);
        let o6 = Self::random_offset_vector(rng);
        let o7 = Self::random_offset_vector(rng);
        let o8 = Self::random_offset_vector(rng);
        let o9 = Self::random_offset_vector(rng);

        for y in 0..self.height as usize {
            let alpha = (y as f32 / self.height as f32) * PI;
            for x in 0..self.width as usize {
                let beta = (x as f32 / self.width as f32) * TAU;

                let v1 = Self::random_noise_from_polar(alpha, beta, R1, o1);
                let v1b = Self::random_noise_from_polar(alpha, beta, R1, o1b);
                let v2 = Self::random_noise_from_polar(alpha, beta, R2, o2);
                let v2b = Self::random_noise_from_polar(alpha, beta, R2, o2b);
                let v3 = Self::random_noise_from_polar(alpha, beta, R3, o3);
                let v4 = Self::random_noise_from_polar(alpha, beta, R4, o4);
                let v5 = Self::random_noise_from_polar(alpha, beta, R5, o5);
                let v6 = Self::random_noise_from_polar(alpha, beta, R6, o6);
                let v7 = Self::random_noise_from_polar(alpha, beta, R7, o7);
                let v8 = Self::random_noise_from_polar(alpha, beta, R8, o8) * 1.5 + 0.25;
                let v9 = Self::random_noise_from_polar(alpha, beta, R9, o9);

                let mut va = self.continent_modifier(x, y);
                va = mix_values(va, v3, 0.22 * v8);
                va = mix_values(va, v4, 0.15 * v8);
                va = mix_values(va, v5, 0.1 * v8);
                va = mix_values(va, v6, 0.03 * v8);
                va = mix_values(va, v7, 0.005 * v8);

                let mut vc = mix_values(v1, v9, 0.5 * v8);
                vc = mix_values(vc, v2, 0.04 * v8);
                vc = Self::mountain_range_noise(vc, 25.0);

                let mut vcb = mix_values(v1b, v9, 0.5 * v8);
                vcb = mix_values(vcb, v2b, 0.04 * v8);
                vcb = Self::mountain_range_noise(vcb, 25.0);

                vc = mix_values(vc, vcb, 0.5 * v8);
                vc = mix_values(vc, v3, 0.35 * v8);
                vc = mix_values(vc, v4, 0.075);
                vc = mix_values(vc, v5, 0.05);
                vc = mix_values(vc, v6, 0.02);
                vc = mix_values(vc, v7, 0.01);

                let vb = mix_values(va, va * 0.02 + 0.49, va - (2.0 * vc - 1.0).max(0.0));
                let vd = mix_values(vb, vc, 0.225 * v8);

                self.terrain[y][x].altitude =
                    Self::MIN_ALTITUDE + (vd * Self::ALTITUDE_SPAN);
            }
        }
    }

    fn generate_rainfall(&mut self, rng: &mut SmallRng) {
        const R1: f32 = 2.0;
        const R2: f32 = 1.0;
        const R3: f32 = 16.0;

        let o1 = Self::random_offset_vector(rng);
        let o2 = Self::random_offset_vector(rng);
        let o3 = Self::random_offset_vector(rng);

        let height = self.height as usize;
        let width = self.width as usize;

        for y in 0..height {
            let alpha = (y as f32 / self.height as f32) * PI;
            for x in 0..width {
                let beta = (x as f32 / self.width as f32) * TAU;

                let rn1 = Self::random_noise_from_polar(alpha, beta, R1, o1);
                let rn2 = Self::random_noise_from_polar(alpha, beta, R2, o2) * 1.5 + 0.25;
                let rn3 = Self::random_noise_from_polar(alpha, beta, R3, o3);

                let va = mix_values(rn1, rn3, 0.15);

                let lat_factor = alpha + (va * 2.0 - 1.0) * PI * 0.2;
                let lat_mod1 = (1.5 * lat_factor.sin()) - 0.5;
                let lat_mod2 = lat_factor.cos();

                let ox1 = (width + x + (lat_mod2 * width as f32 / 20.0).floor() as usize) % width;
                let ox2 = (width + x + (lat_mod2 * width as f32 / 15.0).floor() as usize) % width;
                let ox3 = (width + x + (lat_mod2 * width as f32 / 10.0).floor() as usize) % width;
                let ox4 = (width + x + (lat_mod2 * width as f32 / 5.0).floor() as usize) % width;
                let oy = ((y as f32 + lat_mod2 * height as f32 / 10.0).floor() as usize)
                    .min(height - 1);

                let oa1 = self.terrain[y][ox1].altitude.max(0.0);
                let oa2 = self.terrain[y][ox2].altitude.max(0.0);
                let oa3 = self.terrain[y][ox3].altitude.max(0.0);
                let oa4 = self.terrain[y][ox4].altitude.max(0.0);
                let oa5 = self.terrain[oy][x].altitude.max(0.0);

                let alt = self.terrain[y][x].altitude.max(0.0);

                let alt_mod = (alt - oa1 * 0.7 - oa2 * 0.6 - oa3 * 0.5 - oa4 * 0.4 - oa5 * 0.5
                    + Self::MAX_ALTITUDE * 0.18 * rn2
                    - alt * 0.25)
                    / Self::MAX_ALTITUDE;

                let mut rv = mix_values(lat_mod1, alt_mod, 0.85);
                rv = mix_values(rv.powi(2).copysign(rv), rv, 0.75);

                let rainfall = ((rv * (Self::RAINFALL_SPAN + Self::RAINFALL_DRYNESS_OFFSET))
                    + Self::MIN_RAINFALL
                    - Self::RAINFALL_DRYNESS_OFFSET)
                    .clamp(0.0, Self::MAX_RAINFALL);

                self.terrain[y][x].rainfall = rainfall;
            }
        }
    }

    fn generate_temperature(&mut self, rng: &mut SmallRng) {
        let o1 = Self::random_offset_vector(rng);
        let o2 = Self::random_offset_vector(rng);
        const R1: f32 = 2.0;
        const R2: f32 = 16.0;

        for y in 0..self.height as usize {
            let alpha = (y as f32 / self.height as f32) * PI;
            for x in 0..self.width as usize {
                let beta = (x as f32 / self.width as f32) * TAU;

                let rn1 = Self::random_noise_from_polar(alpha, beta, R1, o1);
                let rn2 = Self::random_noise_from_polar(alpha, beta, R2, o2);

                let lat_mod = alpha * 0.9 + (rn1 + rn2) * 0.05 * PI;
                let alt_factor =
                    (self.terrain[y][x].altitude / Self::MAX_ALTITUDE * Self::TEMPERATURE_ALTITUDE_FACTOR)
                        .max(0.0);

                let temperature =
                    ((lat_mod.sin() - alt_factor) * Self::TEMPERATURE_SPAN + Self::MIN_TEMPERATURE)
                        .clamp(Self::MIN_TEMPERATURE, Self::MAX_TEMPERATURE);

                self.terrain[y][x].temperature = temperature;
            }
        }
    }

    fn generate_biomes(&mut self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let cell = &self.terrain[y][x];
                let mut total = 0.0;
                let mut presences = Vec::new();

                for biome_type in BiomeType::ALL {
                    let stats: BiomeStats = (*biome_type).into();
                    let presence = Self::biome_presence(cell, &stats);
                    if presence > 0.0 {
                        presences.push((*biome_type, presence));
                        total += presence;
                    }
                }

                self.terrain[y][x].biome_presences = presences
                    .into_iter()
                    .map(|(bt, p)| (bt, p / total))
                    .collect();
            }
        }
    }

    fn biome_presence(cell: &TerrainCell, biome: &BiomeStats) -> f32 {
        let alt_diff = cell.altitude - biome.min_altitude;
        if alt_diff < 0.0 {
            return 0.0;
        }
        let alt_factor = alt_diff / (biome.max_altitude - biome.min_altitude);
        if alt_factor > 1.0 {
            return 0.0;
        }

        let mut presence = if alt_factor > 0.5 {
            1.0 - alt_factor
        } else {
            alt_factor
        };

        let rain_diff = cell.rainfall - biome.min_rainfall;
        if rain_diff < 0.0 {
            return 0.0;
        }
        let rain_factor = rain_diff / (biome.max_rainfall - biome.min_rainfall);
        if rain_factor > 1.0 {
            return 0.0;
        }
        presence += if rain_factor > 0.5 {
            1.0 - rain_factor
        } else {
            rain_factor
        };

        let temp_diff = cell.temperature - biome.min_temperature;
        if temp_diff < 0.0 {
            return 0.0;
        }
        let temp_factor = temp_diff / (biome.max_temperature - biome.min_temperature);
        if temp_factor > 1.0 {
            return 0.0;
        }
        presence += if temp_factor > 0.5 {
            1.0 - temp_factor
        } else {
            temp_factor
        };

        presence
    }

    /// Get habitable land tiles as (x, y) coordinates.
    pub fn habitable_tiles(&self) -> Vec<(usize, usize)> {
        let mut tiles = Vec::new();
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let biome = self.terrain[y][x].dominant_biome();
                if biome.is_habitable() {
                    tiles.push((x, y));
                }
            }
        }
        tiles
    }
}
