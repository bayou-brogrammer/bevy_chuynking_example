use ilattice::morton::Morton2i32;
use std::{collections::BTreeMap, hash::Hash};

use bevy::math::IVec2;
use ndshape::Shape;

use crate::ChunkBuffer;

/// Provides an interface to query or modify voxel data for worlds or scenes split into multiple voxel data buffers of a same shape with no level of detail.
#[derive(Default)]
pub struct ChunkMap<V, S>
where
    V: Clone + Copy + Default + PartialEq + Eq + Hash,
    S: Shape<2, Coord = u32> + Clone,
{
    shape: S,
    shape_mask: IVec2,
    chunks: BTreeMap<Morton2i32, ChunkBuffer<V, S>>,
}

#[allow(dead_code)]
impl<V, S> ChunkMap<V, S>
where
    V: Clone + Copy + Default + PartialEq + Eq + Hash,
    S: Shape<2, Coord = u32> + Clone,
{
    pub fn new(chunk_shape: S) -> Self {
        Self {
            chunks: Default::default(),
            shape_mask: !(IVec2::from(chunk_shape.as_array().map(|x| x as i32)) - IVec2::ONE),
            shape: chunk_shape,
        }
    }

    /// Inserts a new buffer at the specified minimum.
    pub fn insert(&mut self, minimum: IVec2, buffer: ChunkBuffer<V, S>) {
        assert!(buffer.shape().as_array() == self.shape.as_array());
        self.chunks.insert(minimum.into(), buffer);
    }

    /// Inserts a new buffer inititalized with the default value of [`V`] at the specified minimum.
    pub fn insert_empty(&mut self, minimum: IVec2) {
        self.chunks.insert(minimum.into(), ChunkBuffer::<V, S>::new_empty(self.shape.clone()));
    }

    /// Inserts buffers from an iterator passed as a parameter
    pub fn insert_batch<T: IntoIterator<Item = (Morton2i32, ChunkBuffer<V, S>)>>(
        &mut self,
        iter: T,
    ) {
        self.chunks.extend(iter);
    }

    /// Returns a reference to the [`VoxelBuffer<V, S>`] at the specified minimum if there's one.
    #[inline]
    pub fn buffer_at(&self, minimum: IVec2) -> Option<&ChunkBuffer<V, S>> {
        self.chunks.get(&minimum.into())
    }

    /// Returns a mutable reference to the [`VoxelBuffer<V, S>`] at the specified minimum if there's one.
    #[inline]
    pub fn buffer_at_mut(&mut self, minimum: IVec2) -> Option<&mut ChunkBuffer<V, S>> {
        self.chunks.get_mut(&minimum.into())
    }

    /// Removes the buffer at the specified minimum and returns it if it exists.
    pub fn remove(&mut self, pos: IVec2) -> Option<ChunkBuffer<V, S>> {
        self.chunks.remove(&pos.into())
    }

    #[inline]
    pub fn shape_mask(&self) -> IVec2 {
        self.shape_mask
    }
}
