use crate::prelude::*;
use bevy::utils::{HashMap, HashSet};

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
