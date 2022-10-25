use crate::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;

/// Provides an async interface to filling the tile types/materials/etc.
/// for a chunk from noise (does *not* include subsequent changes or)
/// ramps.
/// Used by the region loader to break tiling into multiple processes.
pub struct RegionChunkPopulator {
    pub region_id: usize,
    pub material: Vec<usize>,
    pub revealed: Vec<bool>,
    pub tiles: Vec<TileType>,
    pub chunk_id: ChunkLocation,
}

impl RegionChunkPopulator {
    fn new(region_id: usize, chunk_id: ChunkLocation) -> Self {
        Self {
            region_id,
            chunk_id,
            material: vec![0; TILES_PER_CHUNK],
            revealed: vec![false; TILES_PER_CHUNK],
            tiles: vec![TileType::Floor; TILES_PER_CHUNK],
        }
    }
}

pub fn populate_region_chunk(
    region_id: usize,
    chunk_id: ChunkLocation,
) -> RegionChunkPopulator {
    let mut result = RegionChunkPopulator::new(region_id, chunk_id);

    // Obtain resources
    let plock = PLANET_STORE.read();
    let planet = plock.planet.as_ref().unwrap();
    let strata = plock.strata.as_ref().unwrap();
    let noise = plock.height_noise.as_ref().unwrap();
    let cell_noise = plock.material_noise.as_ref().unwrap();

    let tile_x = region_id % WORLD_WIDTH;
    let tile_y = region_id / WORLD_WIDTH;
    let biome_idx = planet.landblocks[region_id].biome_idx;
    let biome = &RAWS.read().biomes.areas[biome_idx];

    // Determine base altitudes for the region
    let mut altitudes = vec![0; CHUNK_SIZE * CHUNK_SIZE];
    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let altitude =
                cell_altitude(noise, tile_x, tile_y, x + chunk_id.x, y + chunk_id.y);
            let altitude_idx = (y * CHUNK_SIZE) + x;
            altitudes[altitude_idx] = altitude;
        }
    }

    // let max_altitude = *altitudes.iter().max().unwrap() as usize;

    // Build a local RNG
    let mut rng = RandomNumberGenerator::seeded(
        planet.noise_seed
            + ((tile_y * REGION_WIDTH * CHUNKS_PER_REGION)
                + (tile_x * REGION_WIDTH * CHUNK_WIDTH)
                + (chunk_id.y * CHUNK_SIZE)
                + chunk_id.x) as u64,
    );

    for cy in 0..CHUNK_SIZE {
        let ry = cy + chunk_id.y;
        for cx in 0..CHUNK_SIZE {
            let rx = cx + chunk_id.x;
            // let altitude_idx = (cy * CHUNK_SIZE) + cx;
            // let altitude = altitudes[altitude_idx] as usize;

            let chunk_idx = chunk_idx(cx, cy);

            // Soil or sand
            let n = cell_noise.get_noise(noise_lon(tile_y, ry * 2), noise_lat(tile_x, rx * 2));

            if rng.roll_dice(1, 100) < biome.soils.soil {
                result.tiles[chunk_idx] = TileType::Soil;
                result.material[chunk_idx] = pick_material(&strata.soils, n);
            } else if rng.roll_dice(1, 100) < biome.soils.sand {
                result.tiles[chunk_idx] = TileType::Sand;
                result.material[chunk_idx] = pick_material(&strata.sand, n);
            }
        }
    }

    result
}

fn cell_altitude(noise: &FastNoise, tile_x: usize, tile_y: usize, x: usize, y: usize) -> u32 {
    let lat = noise_lat(tile_y, y);
    let lon = noise_lon(tile_x, x);
    let sphere_coords = sphere_vertex(100.0, Degrees::new(lat), Degrees::new(lon));
    let noise_height = noise.get_noise3d(sphere_coords.0, sphere_coords.1, sphere_coords.2);
    noise_to_planet_height(noise_height)
}

fn pick_material(materials: &[usize], noise: f32) -> usize {
    let noise_normalized = (noise + 1.0) / 2.0;
    let n = materials.len() as f32 / 1.0;
    materials[(noise_normalized * n) as usize]
}
