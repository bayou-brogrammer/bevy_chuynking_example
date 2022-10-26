use crate::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;

const TREE_CHANCE: i32 = 10;
const PLANTING_CHANCE: i32 = 10;

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

    for y in 10..REGION_HEIGHT - 10 {
        for x in 10..REGION_WIDTH - 10 {
            let tile_idx = mapidx(x, y);
            if region.is_floor(tile_idx) {
                let material = region.material[tile_idx];
                let soil_quality = match raws.materials.materials[material].layer {
                    MaterialLayer::Soil { quality } => quality,
                    _ => 1,
                };

                let available_plants = raws
                    .plants
                    .plants_by_hardiness_and_soil_quality(mean_temperature, soil_quality);

                if !available_plants.is_empty() && (rng.roll_dice(1, 10) as u8) <= soil_quality
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

pub fn plant_trees(planet_idx: PlanetLocation) {
    let region_id = planet_idx.to_region_index();
    let planet_lock = PLANET_STORE.read();
    let planet = planet_lock.planet.as_ref().unwrap();

    let mut rng = RandomNumberGenerator::seeded(planet.noise_seed + region_id as u64);
    let biome_idx = planet.landblocks[region_id].biome_idx;
    let biome = &RAWS.read().biomes.areas[biome_idx];

    let mut region_write = REGIONS.write();
    let region = region_write.regions.get_mut(&region_id).unwrap();

    for y in 10..REGION_HEIGHT - 10 {
        for x in 10..REGION_WIDTH - 10 {
            let tile_idx = mapidx(x, y);
            if region.is_floor(tile_idx) && rng.roll_dice(1, 100) < TREE_CHANCE {
                if rng.rand::<bool>() {
                    region.tiles[tile_idx] = TileType::Tree(TreeType::Deciduous);
                } else {
                    region.tiles[tile_idx] = TileType::Tree(TreeType::Evergreen);
                }
            }
        }
    }
}
