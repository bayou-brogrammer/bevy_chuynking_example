use crate::prelude::*;
use derive_more::{Deref, DerefMut};

#[derive(
    Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Serialize, Deserialize,
)]
pub struct PlanetLocation(pub IVec2);

impl PlanetLocation {
    pub fn new(pos: IVec2) -> Self { Self(pos) }

    pub fn to_region_index(&self) -> usize {
        ((self.x * WORLD_WIDTH as i32) + self.x) as usize
    }

    pub fn to_world(&self) -> IVec2 {
        IVec2::new(self.x * REGION_WIDTH as i32, self.y * REGION_HEIGHT as i32)
    }
}
