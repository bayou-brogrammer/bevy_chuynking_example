use super::*;

pub fn planet_biomes(planet: &mut Planet) {
    use bracket_random::prelude::RandomNumberGenerator;

    let biome_reader = RAWS.read();
    let mut rng = RandomNumberGenerator::seeded(planet.rng_seed);

    for i in 0..planet.landblocks.len() {
        let lb = &planet.landblocks[i];
        let possible_biomes: Vec<(usize, &Biome)> = biome_reader
            .biomes
            .areas
            .iter()
            .enumerate()
            .filter(|b| b.1.occurs.contains(&lb.btype))
            .filter(|b| {
                lb.temperature_c >= b.1.min_temp as f32
                    && lb.temperature_c < b.1.max_temp as f32
            })
            .filter(|b| lb.rainfall_mm >= b.1.min_rain && lb.rainfall_mm < b.1.max_rain)
            .collect();

        if possible_biomes.is_empty() {
            panic!("No biomes for {:#?}", lb);
        } else if let Some(choice) = rng.random_slice_entry(&possible_biomes) {
            planet.landblocks[i].biome_idx = choice.0;
        }
    }
}
