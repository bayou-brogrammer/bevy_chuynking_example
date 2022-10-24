use crate::prelude::*;

/// Represents a location in the world
#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub region: PlanetLocation,
    pub chunk_min: ChunkLocation,
    pub tile: RegionTileLocation,
}

impl Position {
    /// Create with a specific region identifier and tile coordinates
    pub fn new(region: PlanetLocation, tile: RegionTileLocation) -> Self {
        Self { chunk_min: ChunkLocation::ZERO, region, tile }
    }

    pub fn with_tile_coords<N: Into<i32>>(region: PlanetLocation, x: N, y: N) -> Self {
        Self { chunk_min: ChunkLocation::ZERO, region, tile: RegionTileLocation::new(x, y) }
    }

    /// Convert to a region tile index
    pub fn to_tile_index(&self) -> usize { self.tile.to_tile_index() }

    /// Convert to render-space world coordinates
    pub fn to_world(&self) -> IVec2 { self.region.to_world() + self.tile.to_world() }

    pub fn to_point(&self) -> Point {
        let world_pt = self.to_world();
        Point::new(world_pt.x, world_pt.y)
    }

    /// Apply a tile offset and recalculate IDs as needed.
    /// Returns a new position.
    pub fn offset<N: Into<i32>>(&self, x: N, y: N) -> Self {
        let mut new_pos = (self.tile.x + x.into(), self.tile.y + y.into());
        let mut region = self.region;
        while new_pos.0 < 0 {
            region.x -= 1;
            new_pos.0 += REGION_WIDTH as i32;
        }
        while new_pos.0 > REGION_WIDTH as i32 - 1 {
            region.x += 1;
            new_pos.0 -= REGION_WIDTH as i32;
        }
        while new_pos.1 < 0 {
            region.y -= 1;
            new_pos.1 += REGION_WIDTH as i32;
        }
        while new_pos.1 > REGION_HEIGHT as i32 - 1 {
            region.y += 1;
            new_pos.1 -= REGION_WIDTH as i32;
        }

        Self::with_tile_coords(region, new_pos.0, new_pos.1)
    }
}
