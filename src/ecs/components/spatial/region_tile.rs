use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionTileLocation {
    pub x: i32,
    pub y: i32,
}

impl RegionTileLocation {
    pub fn new<N: Into<i32>>(x: N, y: N) -> Self { Self { x: x.into(), y: y.into() } }

    /// Convert to a region tile index
    pub fn to_tile_index(&self) -> usize { (self.y as usize * REGION_WIDTH) + self.x as usize }

    /// Convert to a region-local render world-space
    pub fn to_world(&self) -> IVec2 { IVec2::new(self.x, self.y) }

    /// Convert to a region-local render world-space
    pub fn to_point(&self) -> Point { Point::new(self.x, self.y) }
}
