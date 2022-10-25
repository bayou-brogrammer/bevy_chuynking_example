use crate::prelude::*;

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

/// Checks a set of regions for "TileCreated" status---ready to be queried.
/// Prettying hasn't occurred, neither has render creation.
pub fn are_regions_loaded(locations: &[PlanetLocation]) -> bool {
    let mut loaded = true;
    let region_lock = REGIONS.read();
    locations.iter().map(|loc| loc.to_region_index()).for_each(|idx| {
        if let Some(region) = region_lock.regions.get(&idx) {
            if region.status != RegionStatus::CreatedTiles {
                loaded = false;
            }
        }
    });
    loaded
}

// /// Returns true if a tile is a floor or has a solid tile underneath it.
// pub fn is_tile_floor(region_id: PlanetLocation, tile_idx: usize) -> bool {
//     let index = region_id.to_region_index();
//     let region_lock = REGIONS.read();
//     if let Some(region) = region_lock.regions.get(&index) {
//         matches!(region.tiles[tile_idx], TileType::Floor)
//     } else {
//         false
//     }
// }

pub fn get_material_idx(region_id: PlanetLocation, tile_idx: usize) -> usize {
    let index = region_id.to_region_index();
    let region_lock = REGIONS.read();
    if let Some(region) = region_lock.regions.get(&index) {
        region.material[tile_idx]
    } else {
        0
    }
}
