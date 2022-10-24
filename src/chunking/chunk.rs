use std::collections::HashSet;

use crate::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;

fn cell_altitude(noise: &FastNoise, tile_x: usize, tile_y: usize, x: usize, y: usize) -> u32 {
    let lat = noise_lat(tile_y, y);
    let lon = noise_lon(tile_x, x);
    let sphere_coords = sphere_vertex(100.0, Degrees::new(lat), Degrees::new(lon));
    let noise_height = noise.get_noise3d(sphere_coords.0, sphere_coords.1, sphere_coords.2);
    noise_to_planet_height(noise_height)
}

// A component tagging an entity as a chunk.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Chunk {
    pub region_id: usize,
    pub pos: ChunkLocation,
    pub material: Vec<usize>,
    pub tiles: Vec<TileType>,
}

impl Chunk {
    pub fn new(region_id: usize, pos: ChunkLocation) -> Self {
        Self {
            pos,
            region_id,
            material: vec![0; TILES_PER_CHUNK],
            tiles: vec![TileType::Floor; TILES_PER_CHUNK],
        }
    }

    pub fn generate(&mut self) {
        // Obtain resources
        let rawlock = RAWS.read();
        let plock = PLANET_STORE.read();
        let planet = plock.planet.as_ref().unwrap();
        let strata = plock.strata.as_ref().unwrap();
        let noise = plock.height_noise.as_ref().unwrap();
        let cell_noise = plock.material_noise.as_ref().unwrap();

        let tile_x = self.region_id % WORLD_WIDTH;
        let tile_y = self.region_id / WORLD_WIDTH;
        let biome_idx = planet.landblocks[self.region_id].biome_idx;
        let biome = &RAWS.read().biomes.areas[biome_idx];

        let chunk_id = self.pos;

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

        self.tiles.fill(TileType::Soil);

        for cy in 0..CHUNK_SIZE {
            let ry = cy + chunk_id.y;
            for cx in 0..CHUNK_SIZE {
                let rx = cx + chunk_id.x;
                // let altitude_idx = (cy * CHUNK_SIZE) + cx;
                // let altitude = altitudes[altitude_idx] as usize;

                let chunk_idx = chunk_idx(cx, cy);

                // Soil or sand
                let n =
                    cell_noise.get_noise(noise_lon(tile_y, ry * 2), noise_lat(tile_x, rx * 2));

                if rng.roll_dice(1, 100) < biome.soils.soil {
                    self.tiles[chunk_idx] = TileType::Soil;
                    self.material[chunk_idx] = pick_material(&strata.soils, n);
                } else if rng.roll_dice(1, 100) < biome.soils.sand {
                    self.tiles[chunk_idx] = TileType::Sand;
                    self.material[chunk_idx] = pick_material(&strata.sand, n);
                }
            }
        }

        self.run_rivers(planet, altitudes, &mut rng);
        self.plants(&rawlock, planet, &mut rng);
        self.plant_trees();
    }

    pub fn run_rivers(
        &mut self,
        planet: &Planet,
        altitudes: Vec<u32>,
        rng: &mut RandomNumberGenerator,
    ) {
        let center_point = Point::new(CHUNK_WIDTH / 2, CHUNK_HEIGHT / 2);
        //let mut river_starts_here = false;
        let mut river_terminates_here = false;
        let mut has_river = false;

        let mut river_entry = [0, 0, 0, 0];
        let mut river_exit = 0;
        let region_loc = IVec2::new(
            (self.region_id % WORLD_WIDTH) as i32,
            (self.region_id / WORLD_WIDTH) as i32,
        );

        let regions = REGIONS.read();
        let region = &regions.regions[&self.region_id];

        println!("Region: {:?}", region.location);

        for river in planet.rivers.iter() {
            if river.start == region_loc {
                //river_starts_here = true;
                has_river = true;
            }

            println!("River1");

            let mut last_pos = river.start;
            for i in 0..river.steps.len() {
                if river.steps[i].pos == region_loc {
                    has_river = true;

                    if last_pos.x < region_loc.x {
                        river_entry[0] += 1
                    }
                    if last_pos.x > region_loc.x {
                        river_entry[1] += 1
                    }
                    if last_pos.y < region_loc.y {
                        river_entry[2] += 1
                    }
                    if last_pos.y > region_loc.y {
                        river_entry[3] += 1
                    }

                    if i + 1 < river.steps.len() {
                        let next = river.steps[i + 1].pos;
                        if next.x < region_loc.x {
                            river_exit = 1;
                        }
                        if next.x > region_loc.x {
                            river_exit = 2;
                        }
                        if next.y < region_loc.y {
                            river_exit = 3;
                        }
                        if next.y > region_loc.y {
                            river_exit = 4;
                        }
                    } else {
                        river_terminates_here = true;
                    }
                }
                last_pos = river.steps[i].pos;
            }
        }

        if !has_river {
            return;
        }

        println!("River start: {:?}, end: {}", river_entry, river_terminates_here);

        // Determine a mid-point
        let mut mid_ok = false;
        let mut midpoint = Point::zero();
        while !mid_ok {
            midpoint = Point::new(
                rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4,
                rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4,
            );
            let d = DistanceAlg::Pythagoras.distance2d(center_point, midpoint);
            if d > 15.0 {
                mid_ok = true
            }
        }

        let mut dig_targets: HashSet<usize> = HashSet::new();

        // Run rivers to the confluence
        for _ in 0..river_entry[0] {
            let start = Point::new(
                0,
                rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4 - 1,
            );
            for point in line2d_vector(start, midpoint) {
                add_dig_target(point, 2, &mut dig_targets);
            }
        }
        for _ in 0..river_entry[1] {
            let start = Point::new(
                REGION_WIDTH as i32 - 1,
                rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4 - 1,
            );
            for point in line2d_vector(start, midpoint) {
                add_dig_target(point, 2, &mut dig_targets);
            }
        }
        for _ in 0..river_entry[2] {
            let start = Point::new(
                rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4 - 1,
                0,
            );
            for point in line2d_vector(start, midpoint) {
                add_dig_target(point, 2, &mut dig_targets);
            }
        }
        for _ in 0..river_entry[3] {
            let start = Point::new(
                rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4 - 1,
                REGION_HEIGHT as i32 - 2,
            );
            for point in line2d_vector(start, midpoint) {
                add_dig_target(point, 2, &mut dig_targets);
            }
        }

        if !river_terminates_here {
            let end = match river_exit {
                1 => Point::new(
                    0,
                    rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4 - 1,
                ),
                2 => Point::new(
                    REGION_WIDTH as i32 - 1,
                    rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4 - 1,
                ),
                3 => Point::new(
                    rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4 - 1,
                    0,
                ),
                _ => Point::new(
                    rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4 - 1,
                    REGION_HEIGHT as i32 - 1,
                ),
            };
            for point in line2d_vector(midpoint, end) {
                add_dig_target(point, 2, &mut dig_targets);
            }
        }

        // Do the digging
        for idx in dig_targets.iter() {
            let dig_at = Point::new(idx % REGION_WIDTH, idx / REGION_WIDTH);
            let mut min_altitude = std::u32::MAX;

            for off_y in -2..2 {
                for off_x in -2..=2 {
                    let pt = Point::new(off_x, off_y) + dig_at;
                    let idx = (pt.y * REGION_WIDTH as i32) + pt.x;
                    if idx > 0 && idx < REGION_TILES_COUNT as i32 {
                        let pt_alt = altitudes[idx as usize];
                        if pt_alt < min_altitude {
                            min_altitude = pt_alt;
                        }
                    }
                }
            }

            if min_altitude > 4 {
                self.tiles[*idx] = TileType::Floor;
            }
        }
    }

    pub fn plants(&mut self, raws: &Raws, planet: &Planet, rng: &mut RandomNumberGenerator) {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let chunk_idx = chunk_idx(x, y);

                let soil_quality =
                    match raws.materials.materials[self.material[chunk_idx]].layer {
                        MaterialLayer::Soil { quality } => quality,
                        _ => 1,
                    };

                let mean_temperature = planet.landblocks[self.region_id].temperature_c as i8;

                let available_plants = raws
                    .plants
                    .plants_by_hardiness_and_soil_quality(mean_temperature, soil_quality);

                if !available_plants.is_empty() && (rng.roll_dice(1, 10) as u8) <= soil_quality
                {
                    let chosen_plant = rng.random_slice_entry(&available_plants);
                    if let Some(plant_idx) = chosen_plant {
                        match plant_idx {
                            0 => self.tiles[chunk_idx] = TileType::Grass,
                            1 => self.tiles[chunk_idx] = TileType::Daisy,
                            _ => self.tiles[chunk_idx] = TileType::Heather,
                        }
                    }
                }
            }
        }
    }

    pub fn plant_trees(&mut self) {
        let mut rng = RandomNumberGenerator::seeded(
            PLANET_STORE.read().planet.as_ref().unwrap().noise_seed + self.region_id as u64,
        );
        let biome_idx =
            PLANET_STORE.read().planet.as_ref().unwrap().landblocks[self.region_id].biome_idx;
        let biome = &RAWS.read().biomes.areas[biome_idx];

        let mut deciduous_chance = 0;
        let mut evergreen_chance = 0;
        for t in biome.trees.iter() {
            if t.tree.to_lowercase() == "d" {
                deciduous_chance = t.freq as i32;
            } else if t.tree.to_lowercase() == "e" {
                evergreen_chance = t.freq as i32;
            }
        }

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let chunk_idx = chunk_idx(x, y);

                let floor_material =
                    &RAWS.read().materials.materials[self.material[chunk_idx]];
                let (can_plant, quality) = match floor_material.layer {
                    MaterialLayer::Sand => (true, 2.0),
                    MaterialLayer::Soil { quality } => (true, quality as f32),
                    _ => (false, 0.0),
                };

                if can_plant && (rng.roll_dice(1, 10) as f32) < quality {
                    let die_roll = rng.roll_dice(1, 200);
                    if die_roll < deciduous_chance {
                        println!("Planted a deciduous tree");
                        self.tiles[chunk_idx] = TileType::Tree;
                    } else if die_roll < evergreen_chance {
                        println!("Planted an evergreen tree");
                        self.tiles[chunk_idx] = TileType::Tree;
                    }
                }
            }
        }
    }
}

fn pick_material(materials: &[usize], noise: f32) -> usize {
    let noise_normalized = (noise + 1.0) / 2.0;
    let n = materials.len() as f32 / 1.0;
    materials[(noise_normalized * n) as usize]
}

pub fn get_material_idx(region_idx: usize, tile_idx: usize) -> usize {
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&region_idx) {
        region.material[tile_idx]
    } else {
        0
    }
}

fn add_dig_target(pt: Point, radius: i32, dig_targets: &mut HashSet<usize>) {
    for y in 0 - radius..radius {
        for x in 0 - radius..radius {
            let apt = Point::new(x, y) + pt;
            if apt.x > 0
                && apt.x < REGION_WIDTH as i32 - 1
                && apt.y > 0
                && apt.y < REGION_HEIGHT as i32 - 1
            {
                let idx = (apt.y * REGION_WIDTH as i32) + apt.x;
                dig_targets.insert(idx as usize);
            }
        }
    }
}
