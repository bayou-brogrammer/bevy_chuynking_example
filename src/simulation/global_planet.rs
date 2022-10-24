use crate::prelude::*;
use bracket_noise::prelude::FastNoise;
use lazy_static::*;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

lazy_static! {
    pub static ref PLANET_STORE: Lazy<RwLock<PlanetData>> =
        Lazy::new(|| RwLock::new(PlanetData::new()));
}

#[derive(Default)]
pub struct PlanetData {
    pub planet: Option<Planet>,
    pub height_noise: Option<FastNoise>,
    pub strata: Option<StrataMaterials>,
    pub material_noise: Option<FastNoise>,
}

impl PlanetData {
    pub fn new() -> Self { Self::default() }
}

pub fn set_global_planet(planet: Planet) {
    let planet_copy = planet.clone();
    PLANET_STORE.write().planet = Some(planet);
    PLANET_STORE.write().height_noise = Some(planet_copy.get_height_noise());
    PLANET_STORE.write().material_noise = Some(planet_copy.get_material_noise());
}
