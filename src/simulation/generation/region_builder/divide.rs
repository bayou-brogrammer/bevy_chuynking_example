// use crate::prelude::*;

// pub fn divide_into_chunks(region_id: PlanetLocation) {
//     let region_chunk_base: ChunkLocation = region_id.to_world().into();

//     let mut region_write = REGIONS.write();
//     let region = region_write.regions.get_mut(&region_id.to_region_index()).unwrap();

//     AllChunksIterator::new().for_each(|chunk_id| {
//         let chunk_base = region_chunk_base + chunk_id;
//         let mut chunk = Chunk::new(region_id, chunk_base);

//         let chunk_x = chunk.pos.x / CHUNK_SIZE;
//         let chunk_y = chunk.pos.y / CHUNK_SIZE;
//         let chunk_id = (chunk_y * CHUNK_WIDTH) + chunk_x;

//         // // Wait for saving :)
//         // thread.join().expect("Failed to join thread");
//     });
// }
