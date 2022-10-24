use crate::prelude::*;

/// Handles the process of loading required regions, spawning off async
/// tasks that are then handled by the created_regions_handler
/// system.
pub fn load_regions() {
    let to_do = REGIONS
        .read()
        .regions
        .values()
        .filter(|r| r.status == RegionStatus::NotLoaded)
        .count();

    println!("Regions to do: {}", to_do);
    if to_do == 0 {
        return;
    }

    let mut region_lock = REGIONS.write();
    for (region_id, region) in region_lock.regions.iter_mut() {
        if region.status == RegionStatus::NotLoaded {
            // Spawn a region loader task
            region.status = RegionStatus::CreatingTiles;

            AllChunksIterator::new_with(region.location.to_world().into()).for_each(
                |chunk_base| {
                    let region = *region_id; // Copy to ensure we have a local to move
                    let cloc = chunk_base; // Ditto

                    println!("Spawning chunk loader task: {:?}", chunk_base);

                    std::thread::spawn(move || {
                        populate_region_chunk(region, cloc);
                    });
                },
            );
        }
    }
}

pub fn populate_region_chunk(region_id: usize, chunk_loc: ChunkLocation) {
    if !does_chunk_file_exist(chunk_loc) {
        let mut chunk = Chunk::new(region_id, chunk_loc);
        chunk.generate();
        let thread = std::thread::spawn(move || {
            save_chunk(&chunk);
        });

        // Wait for saving :)
        thread.join().expect("Failed to join thread");
    }

    let mut region_lock = REGIONS.write();
    let region_write = region_lock.regions.get_mut(&region_id).unwrap();
    region_write.chunks_loaded += 1;
    if region_write.chunks_loaded == CHUNKS_PER_REGION {
        println!("Created tiles");
        region_write.status = RegionStatus::CreatedTiles;
    }
    std::mem::drop(region_lock);
}
