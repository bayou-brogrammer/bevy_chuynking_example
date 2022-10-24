use crate::prelude::*;

/// Iterates all chunks in a region, returning their base positions.
/// This is useful when you need to perform an action for every
/// chunk in a region, such as spawning an entire region.
#[derive(Default, Debug)]
pub struct AllChunksIterator {
    x: usize,
    y: usize,
    done: bool,
    chunk_base: ChunkLocation,
}

impl AllChunksIterator {
    pub fn new() -> Self {
        let chunk_base = ChunkLocation { x: 0, y: 0 };
        Self { chunk_base, x: 0, y: 0, done: false }
    }

    pub fn new_with(chunk_base: ChunkLocation) -> Self {
        Self { x: 0, y: 0, chunk_base, done: false }
    }
}

impl ExactSizeIterator for AllChunksIterator {
    fn len(&self) -> usize { CHUNKS_PER_REGION }
}

impl Iterator for AllChunksIterator {
    type Item = ChunkLocation;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result =
            self.chunk_base + ChunkLocation { x: self.x * CHUNK_SIZE, y: self.y * CHUNK_SIZE };
        self.x += 1;
        if self.x == CHUNK_WIDTH {
            self.x = 0;
            self.y += 1;
            if self.y == CHUNK_HEIGHT {
                self.done = true;
            }
        }
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length() {
        let mut n = 0;
        for loc in AllChunksIterator::new() {
            n += 1;
            println!("{:?}", loc);
        }
        assert_eq!(CHUNKS_PER_REGION, n);
    }

    #[test]
    fn test_other() {
        let mut n = 0;
        for loc in AllChunksIterator::new_with(ChunkLocation { x: 0, y: 256 }) {
            n += 1;
            println!("{:?}", loc);
        }
        assert_eq!(CHUNKS_PER_REGION, n);
    }
}
