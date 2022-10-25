use crate::prelude::*;

// A component tagging an entity as a chunk.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Chunk {
    pub tiles: Vec<TileType>,
    pub region: PlanetLocation,
    pub location: ChunkLocation,
}

impl Chunk {
    pub fn new(region: PlanetLocation, location: ChunkLocation) -> Self {
        Self { location, region, tiles: vec![TileType::Floor; TILES_PER_CHUNK] }
    }

    pub fn empty(region: PlanetLocation, location: ChunkLocation) -> Self {
        Self { tiles: Vec::with_capacity(0), location, region }
    }
}
