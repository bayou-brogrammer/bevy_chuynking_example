use crate::prelude::*;

/// Iterates all chunks in a region, returning their base positions.
/// This is useful when you need to perform an action for every
/// chunk in a region, such as spawning an entire region.
#[derive(Default, Debug)]
pub struct AllChunksIterator {
    x: usize,
    y: usize,
    done: bool,
}

impl AllChunksIterator {
    pub fn new() -> Self {
        Self { x: 0, y: 0, done: false }
    }
}

impl ExactSizeIterator for AllChunksIterator {
    fn len(&self) -> usize {
        CHUNKS_PER_REGION
    }
}

impl Iterator for AllChunksIterator {
    type Item = ChunkLocation;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = ChunkLocation { x: self.x * CHUNK_SIZE, y: self.y * CHUNK_SIZE };
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
        for _ in AllChunksIterator::new() {
            n += 1;
        }
        assert_eq!(CHUNKS_PER_REGION, n);
    }
}
