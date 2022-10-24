use crate::prelude::*;

/// Label for the stage housing the chunk loading systems.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, StageLabel)]
pub struct ChunkLoadingStage;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, SystemLabel)]
/// Labels for the systems added by [`VoxelWorldChunkingPlugin`]
pub enum ChunkLoadingSystem {
    /// Runs chunk view distance calculations and queue events for chunk creations and
    /// deletions.
    UpdateViewChunks,
    /// Creates the voxel buffers to hold chunk data and attach them a chunk entity in the ECS
    /// world.
    CreateChunks,
    /// Clears the dirty chunks list.
    ClearDirtyChunks,
}

/// Resource storing the current chunk the player is in as well as its current coords.
#[derive(Debug)]
pub struct CurrentLocalPlayerChunk {
    pub chunk_min: IVec2,
    pub world_pos: IVec2,
}

impl CurrentLocalPlayerChunk {
    pub const ZERO: Self = Self::splat(0);

    #[inline]
    pub fn empty() -> Self { Self::ZERO }

    #[inline]
    pub fn new(chunk_min: IVec2, world_pos: IVec2) -> Self { Self { chunk_min, world_pos } }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: i32) -> Self {
        Self { world_pos: IVec2::splat(v), chunk_min: IVec2::splat(v) }
    }
}

// Resource holding the view distance.
pub struct ChunkLoadRadius {
    pub horizontal: i32,
    pub vertical: i32,
}

/// A queue tracking the creation / destroy commands for chunks.
#[derive(Default)]
pub struct ChunkCommandQueue {
    pub create: Vec<ChunkLocation>,
    pub destroy: Vec<ChunkLocation>,
}

impl ChunkCommandQueue {
    pub fn queue_unload<'a>(&mut self, region: impl Iterator<Item = &'a ChunkLocation>) {
        self.destroy.extend(region);
    }
}
