use crate::prelude::*;
use futures_lite::future;

/// Applies the result of building individual region chunks
pub fn region_tile_applicator_system(
    mut commands: Commands,
    mut region_loaders: Query<(Entity, &mut RegionLoaderTask)>,
) {
    for (task_entity, mut task) in region_loaders.iter_mut() {
        if let Some(chunk) = future::block_on(future::poll_once(&mut task.0)) {
            let mut region_lock = REGIONS.write();
            let region_id = chunk.region_id;

            if let Some(region) = region_lock.regions.get_mut(&region_id) {
                let chunk_x = chunk.chunk_id.x / CHUNK_SIZE;
                let chunk_y = chunk.chunk_id.y / CHUNK_SIZE;
                let chunk_id = (chunk_y * CHUNK_WIDTH) + chunk_x;

                ChunkIterator::new(chunk.chunk_id).enumerate().for_each(|(idx, chunk_idx)| {
                    region.tiles[chunk_idx.to_tile_index()] = chunk.tiles[idx];
                    region.material[chunk_idx.to_tile_index()] = chunk.material[idx];
                });

                region.chunks_loaded[chunk_id] = true;
                if region.chunks_loaded.iter().filter(|l| **l).count() == CHUNKS_PER_REGION {
                    region.status = RegionStatus::CreatedTiles;
                }
            }
        } else {
            panic!("Received region chunk data for a non-loaded region");
        }

        // Remove the task now that it's done
        commands.entity(task_entity).despawn();
    }
}
