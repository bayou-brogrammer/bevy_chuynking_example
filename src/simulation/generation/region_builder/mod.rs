use std::time::Duration;

use crate::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

mod divide;
mod plants;

lazy_static! {
    static ref REGION_GEN: Lazy<RwLock<RegionGen>> =
        Lazy::new(|| RwLock::new(RegionGen::new()));
}

#[allow(dead_code)]
#[derive(Default, PartialEq)]
pub enum RegionBuilderStatus {
    #[default]
    Initializing,
    Chunking,
    Loaded,
    Water,
    // Ramping,
    Vegetation,
    Trees,
    // Crashing,
    // Debris,
    Dividing,
    Done,
}

#[derive(Default)]
pub struct RegionGen {
    pub status: RegionBuilderStatus,
}

impl RegionGen {
    pub fn new() -> Self { Self { status: RegionBuilderStatus::Initializing } }
}

fn update_status(new_status: RegionBuilderStatus) { REGION_GEN.write().status = new_status; }

pub struct RegionBuilder {
    planet: Planet,
    started: bool,
    crash_site: PlanetLocation,
}

impl RegionBuilder {
    pub fn new(planet: Planet, crash_site: PlanetLocation) -> Self {
        Self { planet, crash_site, started: false }
    }

    pub fn status(&self) -> String {
        match REGION_GEN.read().status {
            RegionBuilderStatus::Initializing => String::from("Initializing"),
            RegionBuilderStatus::Chunking => String::from("Dividing & Conquering"),
            RegionBuilderStatus::Loaded => String::from("Region activated, making it pretty"),
            // RegionBuilderStatus::Ramping => String::from("Smoothing Rough Edges"),
            // RegionBuilderStatus::Crashing => String::from("Crash Landing"),
            RegionBuilderStatus::Water => String::from("Just Add Water"),
            RegionBuilderStatus::Vegetation => String::from("Re-seeding the lawn"),
            // RegionBuilderStatus::Debris => String::from("Making a terrible mess"),
            RegionBuilderStatus::Trees => String::from("Planting trees"),
            RegionBuilderStatus::Dividing => String::from("Dividing into chunks..."),
            RegionBuilderStatus::Done => String::from("Done"),
        }
    }

    pub fn generate(&mut self) {
        if !self.started {
            self.started = true;
            let p = self.planet.clone();
            let c = self.crash_site;

            std::thread::spawn(move || build_region(p, c));
        }
    }

    pub fn is_done(&self) -> bool { REGION_GEN.read().status == RegionBuilderStatus::Done }
}

fn build_region(planet: Planet, planet_idx: PlanetLocation) {
    println!("Building region");
    set_global_planet(planet);
    update_status(RegionBuilderStatus::Chunking);
    spawn_playable_region(planet_idx);

    set_tiles(planet_idx);
    update_status(RegionBuilderStatus::Loaded);
    std::thread::sleep(Duration::from_millis(500));

    // Beaches

    // Vegetation
    println!("Veggies");
    update_status(RegionBuilderStatus::Vegetation);
    plants::grow_plants(planet_idx);
    std::thread::sleep(Duration::from_millis(500));

    // Trees
    println!("Trees");
    update_status(RegionBuilderStatus::Trees);
    plants::plant_trees(planet_idx);

    // Divide
    println!("Divide");
    update_status(RegionBuilderStatus::Dividing);
    divide::divide_into_chunks(planet_idx);

    update_status(RegionBuilderStatus::Done);
}

pub fn spawn_playable_region(location: PlanetLocation) {
    let index = location.to_region_index();
    let mut region_lock = REGIONS.write();
    region_lock.regions.insert(index, Region::new(location));
}

pub fn set_tiles(planet_idx: PlanetLocation) {
    let region_id = &planet_idx.to_region_index();

    let mut region_lock = REGIONS.write();
    if let Some(region) = region_lock.regions.get_mut(region_id) {
        // Obtain resources
        let plock = PLANET_STORE.read();
        let planet = plock.planet.as_ref().unwrap();
        let strata = plock.strata.as_ref().unwrap();
        let noise = plock.height_noise.as_ref().unwrap();
        let cell_noise = plock.material_noise.as_ref().unwrap();

        let tile_x = region_id % WORLD_WIDTH;
        let tile_y = region_id / WORLD_WIDTH;
        let biome_idx = planet.landblocks[*region_id].biome_idx;
        let biome = &RAWS.read().biomes.areas[biome_idx];

        // Determine base altitudes for the region
        let mut altitudes = vec![0; REGION_WIDTH * REGION_HEIGHT];
        for y in 0..REGION_HEIGHT {
            for x in 0..REGION_WIDTH {
                let altitude = cell_altitude(noise, tile_x, tile_y, x, y);
                let altitude_idx = (y * REGION_WIDTH) + x;
                altitudes[altitude_idx] = altitude;
            }
        }

        // Build a local RNG
        let mut rng = RandomNumberGenerator::seeded(
            planet.noise_seed
                + ((tile_y * REGION_WIDTH * CHUNKS_PER_REGION)
                    + (tile_x * REGION_WIDTH * CHUNK_WIDTH)
                    + (planet_idx.y as usize * REGION_WIDTH)
                    + planet_idx.x as usize) as u64,
        );

        for cy in 0..REGION_HEIGHT {
            let ry = cy + planet_idx.y as usize;
            for cx in 0..REGION_WIDTH {
                let rx = cx + planet_idx.x as usize;
                let idx = mapidx(cx, cy);

                // Soil or sand
                let n =
                    cell_noise.get_noise(noise_lon(tile_y, ry * 2), noise_lat(tile_x, rx * 2));

                if rng.roll_dice(1, 100) < biome.soils.soil {
                    region.tiles[idx] = TileType::Soil;
                    region.material[idx] = pick_material(&strata.soils, n);
                } else if rng.roll_dice(1, 100) < biome.soils.sand {
                    region.tiles[idx] = TileType::Sand;
                    region.material[idx] = pick_material(&strata.sand, n);
                }
            }
        }
    }
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
