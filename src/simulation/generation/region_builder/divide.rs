use crate::prelude::*;

pub fn divide_into_chunks(region_id: PlanetLocation) {
    let region_idx = &region_id.to_region_index();
    let mut region_lock = REGIONS.write();
    if let Some(region) = region_lock.regions.get_mut(region_idx) {
        let region_chunk_base: ChunkLocation = region.location.to_world().into();

        for chunk_location in AllChunksIterator::new() {
            let actual_chunk_location = region_chunk_base + chunk_location;
            let mut chunk = Chunk::new(region_id, actual_chunk_location);

            ChunkIterator::new(chunk_location)
                .map(|loc| loc.to_tile_index())
                .enumerate()
                .for_each(|(idx, region_tile_idx)| {
                    chunk.tiles[idx] = region.tiles[region_tile_idx];
                });

            save_chunk(&chunk)
        }
    }
}
