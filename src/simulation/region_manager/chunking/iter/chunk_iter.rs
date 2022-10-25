use crate::prelude::*;

/// Iterates all tiles in a CHUNK_SIZE^3 chunk, based on the ChunkLocation
/// as the base position. Each returned location is a region-wide location,
/// not a chunk-wide location.
pub struct ChunkIterator {
    done: bool,
    chunk_base: ChunkLocation,
    current: ChunkLocation,
}

impl ChunkIterator {
    pub fn new(chunk_base: ChunkLocation) -> Self {
        Self { chunk_base, current: chunk_base, done: false }
    }
}

impl Iterator for ChunkIterator {
    type Item = ChunkLocation;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current;
        self.current.x += 1;
        if self.current.x == self.chunk_base.x + CHUNK_SIZE {
            self.current.x = self.chunk_base.x;
            self.current.y += 1;
            if self.current.y == self.chunk_base.y + CHUNK_SIZE {
                self.done = true;
            }
        }
        Some(result)
    }
}

impl ExactSizeIterator for ChunkIterator {
    fn len(&self) -> usize { CHUNK_SIZE * CHUNK_SIZE }
}
