use crate::prelude::*;

pub fn build_chunk(region_id: PlanetLocation, location: ChunkLocation) -> Chunk {
    if does_chunk_file_exist(location) {
        println!("Loading chunk from file: {:?}", location);
        load_chunk(location)
    } else {
        println!("Creating new chunk: {:?}", location);
        let mut chunk = Chunk::new(region_id, location);

        // Build the basic geometric elements
        let region_lock = REGIONS.read();
        let region_idx = region_id.to_region_index();
        if let Some(region) = region_lock.regions.get(&region_idx) {
            let x = region.location.x;
            let y = region.location.y;
            println!("Building chunk from region: {:?}", location);
            let loc = ChunkLocation::new(location.x % x as usize, location.y % y as usize);

            ChunkIterator::new(loc)
                .map(|loc| (loc, loc.to_tile_index()))
                // .filter(|(_, idx)| region.revealed[*idx])
                // .filter(|(_, idx)| region.tiles[*idx] != TileType::Empty)
                .for_each(|(loc, idx)| {
                    let chunk_idx = chunk_idx(loc.x, loc.y);
                    println!("Chunk idx: {} {:?} {}", idx, loc, chunk_idx);

                    chunk.tiles[chunk_idx] = region.tiles[idx];
                });
        }

        chunk
    }
}
