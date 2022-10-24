use crate::BiomeType;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Biomes {
    pub areas: Vec<Biome>,
}

impl Biomes {
    pub fn new() -> Self { Self { areas: Vec::new() } }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Biome {
    pub name: String,
    pub min_temp: i8,
    pub max_temp: i8,
    pub min_rain: i32,
    pub max_rain: i32,
    pub min_mutation: u8,
    pub max_mutation: u8,
    pub occurs: Vec<BiomeType>,
    pub soils: SoilTypes,
    pub trees: Vec<Tree>,
    pub nouns: Vec<String>,
    pub worldgen_tile: usize,
    pub embark_tile: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SoilTypes {
    pub soil: i32,
    pub sand: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tree {
    pub tree: String,
    pub freq: f32,
}
