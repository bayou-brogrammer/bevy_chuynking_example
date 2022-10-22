use crate::prelude::*;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

mod biomes;
mod calc;
mod coast;
mod noise;
mod type_allocation;
mod wind_and_rain;
mod zero;
pub use calc::*;

lazy_static! {
    pub static ref PLANET_BUILDER: Lazy<RwLock<PlanetBuilder>> =
        Lazy::new(|| RwLock::new(PlanetBuilder::new()));
}

pub fn update_status(status: PlanetBuilderStatus) {
    let mut pb = PLANET_BUILDER.write();
    pb.status = status;
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum PlanetBuilderStatus {
    #[default]
    Initializing,
    Flattening,
    Altitudes,
    Dividing,
    Coast,
    Rainfall {
        amount: u8,
    },
    Biomes,
    Saving,
    Done,
}

#[derive(Default, Clone, Copy)]
pub struct PlanetBuilder {
    status: PlanetBuilderStatus,
}

impl PlanetBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn generate(&self, seed: &str, worldgen_lacunarity: f32) {
        let seed = seed.to_string();
        let lacunarity = worldgen_lacunarity;

        std::thread::spawn(move || make_planet(seed, lacunarity));
    }

    pub fn get_status(&self) -> String {
        match self.status {
            PlanetBuilderStatus::Initializing => String::from("Building a giant ball of mud"),
            PlanetBuilderStatus::Flattening => String::from("Smoothing out the corners"),
            PlanetBuilderStatus::Altitudes => String::from("Squishing out some topology"),
            PlanetBuilderStatus::Dividing => String::from("Dividing the heaven and hearth"),
            PlanetBuilderStatus::Coast => String::from("Crinkling up the coastlines"),
            PlanetBuilderStatus::Rainfall { amount } => {
                format!("Spinning the barometer {}%", amount)
            }
            PlanetBuilderStatus::Biomes => String::from("Zooming on on details"),
            PlanetBuilderStatus::Saving => String::from("Saving the World"),
            PlanetBuilderStatus::Done => String::from("Redirecting..."),
        }
    }

    pub fn is_done(&self) -> bool { self.status == PlanetBuilderStatus::Done }
}

fn make_planet(seed: String, worldgen_lacunarity: f32) {
    let mut base_seed = 0;

    seed.chars().for_each(|c| base_seed += c as u64);
    update_status(PlanetBuilderStatus::Initializing);

    let mut planet = Planet {
        rng_seed: base_seed + 1,
        noise_seed: base_seed,
        landblocks: Vec::with_capacity(WORLD_TILES_COUNT),
        water_height: 0,
        plains_height: 0,
        hills_height: 0,
        lacunarity: worldgen_lacunarity,
    };

    update_status(PlanetBuilderStatus::Flattening);
    println!("Zero Fill");
    zero::zero_fill(&mut planet);

    update_status(PlanetBuilderStatus::Altitudes);
    noise::planetary_noise(&mut planet);

    update_status(PlanetBuilderStatus::Dividing);
    type_allocation::planet_type_allocation(&mut planet);

    update_status(PlanetBuilderStatus::Coast);
    coast::planet_coastlines(&mut planet);

    println!("Wind and rain");
    update_status(PlanetBuilderStatus::Rainfall { amount: 0 });
    wind_and_rain::planet_rainfall(&mut planet);

    println!("Biomes");
    update_status(PlanetBuilderStatus::Biomes);
    biomes::planet_biomes(&mut planet);

    println!("Saving...");
    update_status(PlanetBuilderStatus::Saving);
    save_planet(planet);

    println!("Done...");
    update_status(PlanetBuilderStatus::Done);
}
