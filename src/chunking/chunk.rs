use crate::prelude::*;

// A component tagging an entity as a chunk.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Chunk {
    pub pos: ChunkLocation,
    pub tiles: Vec<TileType>,
}

impl Chunk {
    pub fn new(pos: ChunkLocation) -> Self {
        Self { pos, tiles: vec![TileType::Floor; TILES_PER_CHUNK] }
    }
}
