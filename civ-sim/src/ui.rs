// Browser UI helpers - most UI logic is in JavaScript
// This module provides rendering utilities

use crate::world::BiomeType;

/// Get the RGB color for a biome
pub fn biome_color(biome: &BiomeType) -> (u8, u8, u8) {
    biome.color()
}

/// Render the world map to an RGBA buffer
pub fn render_world_map(
    tiles: &[crate::world::Tile],
    width: u32,
    height: u32,
    view_mode: MapViewMode,
    nations: &[(u64, (u8, u8, u8))], // nation_id -> color
) -> Vec<u8> {
    let mut buffer = vec![0u8; (width * height * 4) as usize];

    for tile in tiles {
        let idx = ((tile.y * width + tile.x) * 4) as usize;
        if idx + 3 >= buffer.len() {
            continue;
        }

        let (r, g, b) = match view_mode {
            MapViewMode::Biomes => tile.biome.color(),
            MapViewMode::Political => {
                if let Some(nation_id) = tile.owner_nation_id {
                    nations
                        .iter()
                        .find(|(id, _)| *id == nation_id)
                        .map(|(_, color)| *color)
                        .unwrap_or_else(|| tile.biome.color())
                } else {
                    tile.biome.color()
                }
            }
            MapViewMode::Terrain => {
                // Grayscale based on altitude
                let alt_normalized = ((tile.altitude + 1.0) / 2.0 * 255.0) as u8;
                if tile.altitude < 0.0 {
                    // Water - blue tint
                    let depth = ((-tile.altitude) * 100.0).min(255.0) as u8;
                    (0, 50.max(100 - depth), 150.max(200 - depth))
                } else {
                    // Land - brown/green gradient
                    (
                        (100 + alt_normalized / 3).min(255),
                        (80 + alt_normalized / 4).min(200),
                        (60).min(100),
                    )
                }
            }
            MapViewMode::Temperature => {
                // Blue (cold) to Red (hot)
                let t_normalized = ((tile.temperature + 40.0) / 80.0).clamp(0.0, 1.0);
                (
                    (t_normalized * 255.0) as u8,
                    50,
                    ((1.0 - t_normalized) * 255.0) as u8,
                )
            }
            MapViewMode::Rainfall => {
                // Brown (dry) to Blue (wet)
                let r_normalized = (tile.rainfall / 4000.0).clamp(0.0, 1.0);
                (
                    ((1.0 - r_normalized) * 200.0) as u8,
                    ((1.0 - r_normalized) * 150.0) as u8,
                    (r_normalized * 255.0) as u8,
                )
            }
            MapViewMode::Population => {
                if tile.population > 0 {
                    let pop_factor = (tile.population as f32 / 10000.0).min(1.0);
                    (
                        (255.0 * pop_factor) as u8,
                        (100.0 * (1.0 - pop_factor)) as u8,
                        0,
                    )
                } else {
                    tile.biome.color()
                }
            }
        };

        buffer[idx] = r;
        buffer[idx + 1] = g;
        buffer[idx + 2] = b;
        buffer[idx + 3] = 255; // Alpha
    }

    buffer
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapViewMode {
    Biomes,
    Political,
    Terrain,
    Temperature,
    Rainfall,
    Population,
}
