use std::time::Duration;

use crate::prelude::*;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

lazy_static! {
    static ref REGION_GEN: Lazy<RwLock<RegionGen>> =
        Lazy::new(|| RwLock::new(RegionGen::new()));
}

#[allow(dead_code)]
#[derive(Default, PartialEq)]
pub enum RegionBuilderStatus {
    #[default]
    Initializing,
    Chunking,
    Loaded,
    Water,
    Ramping,
    Vegetation,
    Trees,
    Crashing,
    Debris,
    Done,
}

#[derive(Default)]
pub struct RegionGen {
    pub status: RegionBuilderStatus,
}

impl RegionGen {
    pub fn new() -> Self { Self { status: RegionBuilderStatus::Initializing } }
}

fn update_status(new_status: RegionBuilderStatus) { REGION_GEN.write().status = new_status; }

pub struct RegionBuilder {
    planet: Planet,
    started: bool,
    crash_site: PlanetLocation,
}

impl RegionBuilder {
    pub fn new(planet: Planet, crash_site: PlanetLocation) -> Self {
        Self { planet, crash_site, started: false }
    }

    pub fn status(&self) -> String {
        match REGION_GEN.read().status {
            RegionBuilderStatus::Initializing => String::from("Initializing"),
            RegionBuilderStatus::Chunking => String::from("Dividing & Conquering"),
            RegionBuilderStatus::Loaded => String::from("Region activated, making it pretty"),
            RegionBuilderStatus::Ramping => String::from("Smoothing Rough Edges"),
            RegionBuilderStatus::Crashing => String::from("Crash Landing"),
            RegionBuilderStatus::Water => String::from("Just Add Water"),
            RegionBuilderStatus::Vegetation => String::from("Re-seeding the lawn"),
            RegionBuilderStatus::Debris => String::from("Making a terrible mess"),
            RegionBuilderStatus::Trees => String::from("Planting trees"),
            RegionBuilderStatus::Done => String::from("Done"),
        }
    }

    pub fn generate(&mut self) {
        if !self.started {
            self.started = true;
            let p = self.planet.clone();
            let c = self.crash_site;

            std::thread::spawn(move || build_region(p, c));
        }
    }

    pub fn is_done(&self) -> bool { REGION_GEN.read().status == RegionBuilderStatus::Done }
}

fn build_region(planet: Planet, planet_idx: PlanetLocation) {
    println!("Building region");
    set_global_planet(planet);
    update_status(RegionBuilderStatus::Chunking);
    spawn_playable_region(planet_idx);

    while !is_region_loaded(planet_idx) {
        std::thread::sleep(Duration::from_millis(10));
    }

    println!("Region loaded");
    update_status(RegionBuilderStatus::Loaded);
    update_status(RegionBuilderStatus::Done);
}

pub fn spawn_playable_region(location: PlanetLocation) {
    let index = location.to_region_index();
    let mut region_lock = REGIONS.write();
    region_lock.regions.insert(index, Region::new(location));
}

/// Returns true when a region has reached the "TilesCreated" stage---it
/// can be queried for tile content. Prettying hasn't occurred yet,
/// neither has render creation.
pub fn is_region_loaded(location: PlanetLocation) -> bool {
    let index = location.to_region_index();
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&index) {
        region.status == RegionStatus::CreatedTiles
    } else {
        false
    }
}
