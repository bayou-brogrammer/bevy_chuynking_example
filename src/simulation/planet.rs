use crate::prelude::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub rng_seed: u64,
    pub noise_seed: u64,
    pub lacunarity: f32,
    pub water_height: u32,
    pub hills_height: u32,
    pub plains_height: u32,
    pub rivers: Vec<River>,
    pub landblocks: Vec<Landblock>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Landblock {
    pub height: u32,
    pub variance: u32,
    pub btype: BiomeType,
    pub rainfall_mm: i32,
    pub biome_idx: usize,
    pub temperature_c: f32,
    pub air_pressure_kpa: f32,
    pub prevailing_wind: Direction,
    pub neighbors: [(Direction, usize); 4],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
    None,
}

impl Planet {
    pub fn get_height_noise(&self) -> FastNoise {
        let mut noise = FastNoise::seeded(self.noise_seed);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.5);
        noise.set_fractal_lacunarity(self.lacunarity);
        noise.set_frequency(0.01);
        noise
    }

    pub fn get_material_noise(&self) -> FastNoise {
        let mut cell_noise = FastNoise::seeded(self.noise_seed + 1);
        cell_noise.set_cellular_return_type(CellularReturnType::CellValue);
        cell_noise.set_noise_type(NoiseType::Cellular);
        cell_noise.set_frequency(0.08);
        cell_noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);
        cell_noise
    }
}

pub fn save_planet(planet: Planet) {
    println!("Saving planet");
    if let Err(err) = save_data(world_save_location("world.dat"), planet) {
        println!("Error saving world: {:?}", err);
    }
}

pub fn load_planet() -> Planet {
    match load_data::<Planet>(world_save_location("world.dat")) {
        Ok(planet) => planet,
        Err(err) => panic!("Error loading world: {:?}", err),
    }
}
