use crate::prelude::*;
use crate::simulation::planet::Direction;
use bevy_ecs_tilemap::tiles::TilePos;
use derive_more::{Deref, DerefMut};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

mod biomes;
mod calc;
mod coast;
mod noise;
mod rivers;
mod type_allocation;
mod wind_and_rain;
mod zero;

pub use calc::*;
pub use noise::*;

lazy_static! {
    pub static ref PLANET_GEN: Lazy<RwLock<PlanetGen>> =
        Lazy::new(|| RwLock::new(PlanetGen::new()));
}

#[derive(Default, Deref, DerefMut)]
pub struct PlanetGen {
    planet: Option<Planet>,

    #[deref(ignore)]
    #[deref_mut(ignore)]
    status: PlanetBuilderStatus,

    #[deref(ignore)]
    #[deref_mut(ignore)]
    tiles: Option<Vec<(TilePos, u32)>>,
}

impl PlanetGen {
    fn new() -> Self { Self::default() }
}

pub fn update_status(status: PlanetBuilderStatus) {
    let mut pb = PLANET_GEN.write();
    pb.status = status;
}

///////////////////////////////////////////////////////////////////////////////

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
    Rivers,
    Saving,
    Done,
}

#[derive(Default, Clone)]
pub struct PlanetBuilder;

impl PlanetBuilder {
    pub fn new() -> Self {
        *PLANET_GEN.write() = PlanetGen::new();
        Self::default()
    }

    pub fn get_planet(&self) -> Option<Planet> { PLANET_GEN.read().planet.clone() }

    pub fn generate(&self, seed: &str, worldgen_lacunarity: f32) {
        let seed = seed.to_string();
        let lacunarity = worldgen_lacunarity;

        std::thread::spawn(move || make_planet(seed, lacunarity));
    }

    pub fn get_status(&self) -> String {
        match PLANET_GEN.read().status {
            PlanetBuilderStatus::Initializing => String::from("Building a giant ball of mud"),
            PlanetBuilderStatus::Flattening => String::from("Smoothing out the corners"),
            PlanetBuilderStatus::Altitudes => String::from("Squishing out some topology"),
            PlanetBuilderStatus::Dividing => String::from("Dividing the heaven and hearth"),
            PlanetBuilderStatus::Coast => String::from("Crinkling up the coastlines"),
            PlanetBuilderStatus::Rainfall { amount } => {
                format!("Spinning the barometer {amount}%")
            }
            PlanetBuilderStatus::Biomes => String::from("Zooming on on details"),
            PlanetBuilderStatus::Rivers => String::from("Digging the rivers!"),
            PlanetBuilderStatus::Saving => String::from("Saving the World"),
            PlanetBuilderStatus::Done => String::from("Planet Gen Done"),
        }
    }

    pub fn tile_info(&self) -> Option<Vec<(TilePos, u32)>> {
        let has_info = PLANET_GEN.read().tiles.is_some();
        if has_info {
            let mut write_lock = PLANET_GEN.write();
            write_lock.tiles.take()
        } else {
            None
        }
    }

    pub fn clear_tiles(&self) {
        println!("Clearing tiles");
        let mut write_lock = PLANET_GEN.write();
        write_lock.tiles = None;
    }

    pub fn is_done(&self) -> bool { PLANET_GEN.read().status == PlanetBuilderStatus::Done }

    pub fn is_building(&self) -> bool {
        PLANET_GEN.read().status != PlanetBuilderStatus::Done
            && PLANET_GEN.read().status != PlanetBuilderStatus::Initializing
    }

    pub fn save_planet(&self) {
        println!("Saving...");
        if PLANET_GEN.read().planet.is_some() {
            let mut write_lock = PLANET_GEN.write();
            let planet = write_lock.planet.take();

            std::thread::spawn(move || {
                update_status(PlanetBuilderStatus::Saving);
                save_planet(planet.unwrap());
            });
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

fn make_planet(seed: String, worldgen_lacunarity: f32) {
    let mut base_seed = 0;

    seed.chars().for_each(|c| base_seed += c as u64);
    update_status(PlanetBuilderStatus::Initializing);

    let mut planet = Planet {
        water_height: 0,
        hills_height: 0,
        plains_height: 0,
        noise_seed: base_seed,
        rng_seed: base_seed + 1,
        lacunarity: worldgen_lacunarity,
        rivers: Vec::with_capacity(WORLD_TILES_COUNT),
        landblocks: Vec::with_capacity(WORLD_TILES_COUNT),
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
    {
        biomes::planet_biomes(&mut planet);
        let tiles = fill_tiles(&planet);
        PLANET_GEN.write().tiles = Some(tiles);
    }

    println!("Rivers");
    update_status(PlanetBuilderStatus::Rivers);
    rivers::run_rivers(&mut planet);

    println!("Done...");
    update_status(PlanetBuilderStatus::Done);
    {
        PLANET_GEN.write().planet = Some(planet);
    }
}

pub fn fill_tiles(planet: &Planet) -> Vec<(TilePos, u32)> {
    let mut tiles: Vec<(TilePos, u32)> = Vec::new();
    for y in 0..WORLD_HEIGHT as i32 {
        for x in 0..WORLD_WIDTH as i32 {
            let pidx = planet_idx(x as usize, y as usize);
            let biome_idx = planet.landblocks[pidx].biome_idx;
            let tile_index = crate::raws::RAWS.read().biomes.areas[biome_idx].embark_tile;
            // let tx = x - WORLD_WIDTH as i32 / 2;
            // let ty = y - WORLD_HEIGHT as i32 / 2;
            tiles.push((TilePos { x: x as u32, y: y as u32 }, tile_index as u32));
        }
    }

    tiles
}
