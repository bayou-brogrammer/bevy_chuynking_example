use crate::prelude::*;
use derive_more::Sub;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Sub, Serialize, Deserialize)]
pub struct ChunkLocation {
    pub x: usize,
    pub y: usize,
}

impl ChunkLocation {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    #[inline(always)]
    pub fn new(x: usize, y: usize) -> Self { Self { x, y } }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: usize) -> Self { Self { x: v, y: v } }

    pub fn to_tile_index(&self) -> usize { mapidx(self.x, self.y) }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub fn as_vec2(&self) -> crate::Vec2 { crate::Vec2::new(self.x as f32, self.y as f32) }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub fn as_ivec2(&self) -> crate::IVec2 { crate::IVec2::new(self.x as i32, self.y as i32) }

    pub fn chunk_key_from(&self, x: i32, y: i32) -> ChunkLocation {
        let new_x = (self.x + x as usize).clamp(0, CHUNKS_PER_REGION * 2);
        let new_y = (self.y + y as usize).clamp(0, CHUNKS_PER_REGION * 2);
        ChunkLocation::new(new_x, new_y)
    }
}

impl From<ChunkLocation> for Vec2 {
    fn from(loc: ChunkLocation) -> Self { Self { x: loc.x as f32, y: loc.y as f32 } }
}

impl From<Vec2> for ChunkLocation {
    fn from(vec: Vec2) -> Self { Self { x: vec.x as usize, y: vec.y as usize } }
}

impl From<IVec2> for ChunkLocation {
    fn from(vec: IVec2) -> Self { Self { x: vec.x as usize, y: vec.y as usize } }
}
