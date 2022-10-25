use bevy::tasks::{AsyncComputeTaskPool, Task};

use crate::prelude::*;

#[derive(Component)]
pub struct RegionLoaderTask(pub Task<RegionChunkPopulator>);

/// Handles the process of loading required regions, spawning off async
/// tasks that are then handled by the created_regions_handler
/// system.
pub fn load_regions(mut commands: Commands) {
    let to_do = REGIONS
        .read()
        .regions
        .values()
        .filter(|r| r.status == RegionStatus::NotLoaded)
        .count();

    if to_do == 0 {
        return;
    }

    let task_manager = AsyncComputeTaskPool::get();

    let mut region_lock = REGIONS.write();
    for (region_id, region) in region_lock.regions.iter_mut() {
        if region.status == RegionStatus::NotLoaded {
            // Spawn a region loader task
            region.status = RegionStatus::CreatingTiles;

            AllChunksIterator::new().for_each(|chunk_base| {
                let region = *region_id; // Copy to ensure we have a local to move
                let cloc = chunk_base; // Ditto

                let task =
                    task_manager.spawn(async move { populate_region_chunk(region, cloc) });

                commands.spawn().insert(RegionLoaderTask(task));
            });
        }
    }
}
