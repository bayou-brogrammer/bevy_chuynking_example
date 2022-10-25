use crate::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;

const TREE_CHANCE: i32 = 1000;
const PLANTING_CHANCE: i32 = 20;

pub fn grow_plants(region_id: PlanetLocation) {
    let raws = RAWS.read();
    let planet_lock = PLANET_STORE.read();
    let planet = planet_lock.planet.as_ref().unwrap();

    let planet_idx = region_id.to_region_index();
    let mean_temperature = planet.landblocks[planet_idx].temperature_c as i8;
    let mut rng =
        RandomNumberGenerator::seeded(planet.noise_seed + region_id.to_region_index() as u64);

    let mut region_write = REGIONS.write();
    let region = region_write.regions.get_mut(&planet_idx).unwrap();

    for y in 0..REGION_HEIGHT {
        for x in 0..REGION_WIDTH {
            let tile_idx = mapidx(x, y);
            if region.is_floor(tile_idx) && region.tiles[tile_idx] == TileType::Water {
                let material = get_material_idx(region_id, tile_idx);
                let soil_quality = match raws.materials.materials[material].layer {
                    MaterialLayer::Soil { quality } => quality,
                    _ => 1,
                };

                let available_plants = raws
                    .plants
                    .plants_by_hardiness_and_soil_quality(mean_temperature, soil_quality);

                if !available_plants.is_empty()
                    && (rng.roll_dice(1, PLANTING_CHANCE) as u8) <= soil_quality
                {
                    let chosen_plant = rng.random_slice_entry(&available_plants);
                    if let Some(plant_idx) = chosen_plant {
                        match plant_idx {
                            0 => region.tiles[tile_idx] = TileType::Plant(PlantType::Grass),
                            1 => region.tiles[tile_idx] = TileType::Plant(PlantType::Daisy),
                            _ => region.tiles[tile_idx] = TileType::Plant(PlantType::Heather),
                        }
                    }
                }
            }
        }
    }
}

pub fn plant_trees(region_id: PlanetLocation) {
    let planet_idx = region_id.to_region_index();
    let planet_lock = PLANET_STORE.read();
    let planet = planet_lock.planet.as_ref().unwrap();

    let mut rng = RandomNumberGenerator::seeded(planet.noise_seed + planet_idx as u64);
    let biome_idx = planet.landblocks[planet_idx].biome_idx;
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

    let mut region_write = REGIONS.write();
    let region = region_write.regions.get_mut(&planet_idx).unwrap();
    for y in 10..REGION_HEIGHT - 10 {
        for x in 10..REGION_WIDTH - 10 {
            let idx = mapidx(x, y);
            let crash_distance = DistanceAlg::Pythagoras
                .distance2d(Point::new(REGION_WIDTH / 2, REGION_HEIGHT / 2), Point::new(x, y));

            if crash_distance > 20.0 && region.is_floor(idx) {
                let mat_idx = region.material[idx];
                let floor_material = &RAWS.read().materials.materials[mat_idx];
                let (can_plant, quality) = match floor_material.layer {
                    MaterialLayer::Sand => (true, 2.0),
                    MaterialLayer::Soil { quality } => (true, quality as f32),
                    _ => (false, 0.0),
                };

                if can_plant && (rng.roll_dice(1, 10) as f32) < quality {
                    let die_roll = rng.roll_dice(1, TREE_CHANCE);
                    if die_roll < deciduous_chance {
                        region.tiles[idx] = TileType::Tree(TreeType::Deciduous);
                    } else if die_roll < evergreen_chance {
                        region.tiles[idx] = TileType::Tree(TreeType::Evergreen);
                    }
                }
            }
        }
    }
}
