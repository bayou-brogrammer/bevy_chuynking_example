use super::*;

pub fn planet_coastlines(planet: &mut Planet) {
    for y in 1..WORLD_HEIGHT - 1 {
        for x in 1..WORLD_WIDTH - 1 {
            let base_idx = planet_idx(x, y);
            if planet.landblocks[base_idx].btype != BiomeType::Water
                && (planet.landblocks[base_idx - 1].btype == BiomeType::Water
                    || planet.landblocks[base_idx + 1].btype == BiomeType::Water
                    || planet.landblocks[base_idx - WORLD_WIDTH].btype == BiomeType::Water
                    || planet.landblocks[base_idx + WORLD_WIDTH].btype == BiomeType::Water)
            {
                planet.landblocks[base_idx].btype = BiomeType::Coastal;
            }
        }
    }
}
