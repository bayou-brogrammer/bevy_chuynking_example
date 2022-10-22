use crate::CHUNK_SIZE_U32;
use ndshape::ConstShape2u32;

mod buffer;
mod chunk_map;

pub use buffer::*;
pub use chunk_map::*;

pub type ChunkShape = ConstShape2u32<CHUNK_SIZE_U32, CHUNK_SIZE_U32>;
