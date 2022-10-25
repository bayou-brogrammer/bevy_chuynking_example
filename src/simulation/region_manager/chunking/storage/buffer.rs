use ilattice::extent::Extent;
use ilattice::glam::UVec2;
use ndshape::Shape;

/// A buffer of typed voxel data stored as a contiguous array in memory.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ChunkBuffer<V, S: Shape<2, Coord = u32>>
where
    V: Copy + Clone + Default,
{
    data: Box<[V]>,
    shape: S,
}

#[allow(dead_code)]
impl<V, S: Shape<2, Coord = u32>> ChunkBuffer<V, S>
where
    V: Copy + Clone + Default,
{
    #[inline]
    pub fn new(shape: S, initial_val: V) -> Self {
        Self { data: vec![initial_val; shape.size() as usize].into_boxed_slice(), shape }
    }

    #[inline]
    pub fn new_empty(shape: S) -> Self {
        Self {
            data: vec![Default::default(); shape.size() as usize].into_boxed_slice(),
            shape,
        }
    }

    #[inline]
    pub fn shape(&self) -> &S {
        &self.shape
    }

    /// Fills an extent of this buffer with the specified value.
    #[inline]
    pub fn fill_extent(&mut self, extent: Extent<UVec2>, val: V) {
        ndcopy::fill2(
            extent.shape.to_array(),
            val,
            &mut self.data,
            &self.shape,
            extent.minimum.to_array(),
        );
    }

    // Returns the voxel at the querried position in local space.
    #[inline]
    pub fn tile_at(&self, pos: UVec2) -> V {
        self.data[self.shape.linearize(pos.to_array()) as usize]
    }

    // Returns a mutable reference to the the voxel at the querried position in local space.
    #[inline]
    pub fn tile_at_mut(&mut self, pos: UVec2) -> &mut V {
        &mut self.data[self.shape.linearize(pos.to_array()) as usize]
    }

    #[inline]
    pub fn slice(&self) -> &[V] {
        &self.data
    }

    #[inline]
    pub fn slice_mut(&mut self) -> &mut [V] {
        &mut self.data
    }
}
