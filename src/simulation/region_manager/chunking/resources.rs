use crate::prelude::*;
use bevy::utils::{HashMap, HashSet};

//////////////////////////////////////////////////////////////////////////////////////////
// Chunk System Labels
//////////////////////////////////////////////////////////////////////////////////////////

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

//////////////////////////////////////////////////////////////////////////////////////////
// Local Player Chunk Tracking
//////////////////////////////////////////////////////////////////////////////////////////

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

//////////////////////////////////////////////////////////////////////////////////////////
// Chunk Entities
//////////////////////////////////////////////////////////////////////////////////////////

/// Stores the Entity <-> Chunk voxel data buffer mapping
#[derive(Debug, Default)]
pub struct ChunkEntities(pub HashMap<ChunkLocation, Entity>);

impl ChunkEntities {
    /// Returns the entity attached to the chunk.
    pub fn entity(&self, pos: ChunkLocation) -> Option<Entity> { self.0.get(&pos).copied() }

    /// Attaches the specified entity to the chunk data.
    pub fn attach_entity(&mut self, pos: ChunkLocation, entity: Entity) {
        if self.0.get(&pos).is_none() {
            self.0.insert(pos, entity);
        }
    }

    /// Detaches the specified entity to the chunk data.
    pub fn detach_entity(&mut self, pos: ChunkLocation) -> Option<Entity> {
        self.0.remove(&pos)
    }

    pub fn get(&self, pos: ChunkLocation) -> Option<&Entity> { self.0.get(&pos) }

    pub fn iter(&self) -> impl Iterator<Item = (&ChunkLocation, &Entity)> { self.0.iter() }

    /// Returns an iterator iterating over the loaded chunk keys.
    pub fn iter_keys(&self) -> impl Iterator<Item = &ChunkLocation> { self.0.keys() }

    /// Return the number of loaded chunks.
    pub fn len(&self) -> usize { self.0.len() }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Holds the dirty chunk for the current frame.
#[derive(Debug, Default)]
pub struct DirtyChunks(pub HashSet<ChunkLocation>);

#[allow(dead_code)]
impl DirtyChunks {
    pub fn mark_dirty(&mut self, chunk: ChunkLocation) { self.0.insert(chunk); }

    pub fn iter_dirty(&self) -> impl Iterator<Item = &ChunkLocation> { self.0.iter() }

    pub fn num_dirty(&self) -> usize { self.0.len() }
}
