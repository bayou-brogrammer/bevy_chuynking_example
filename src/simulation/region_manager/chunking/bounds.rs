// World Size
pub const WORLD_WIDTH: usize = 180;
pub const WORLD_HEIGHT: usize = 90;
pub const WORLD_TILES_COUNT: usize = WORLD_HEIGHT * WORLD_WIDTH;

// Region Size
pub const REGION_WIDTH: usize = 256;
pub const REGION_HEIGHT: usize = 256;
pub const REGION_TILES_COUNT: usize = REGION_WIDTH * REGION_HEIGHT;

// Terrain Chunks
pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_U32: u32 = CHUNK_SIZE as u32;
pub const CHUNK_SIZE_U: usize = CHUNK_SIZE;
pub const CHUNK_WIDTH: usize = REGION_WIDTH / CHUNK_SIZE;
pub const CHUNK_HEIGHT: usize = REGION_HEIGHT / CHUNK_SIZE;
pub const CHUNKS_PER_REGION: usize = CHUNK_WIDTH * CHUNK_HEIGHT;
pub const TILES_PER_CHUNK: usize = CHUNK_SIZE * CHUNK_SIZE;

/// Indexes a map tile within an active map
pub fn mapidx<N: Into<usize>>(x: N, y: N) -> usize {
    let xc = x.into();
    let yc = y.into();
    debug_assert!(xc <= REGION_WIDTH && yc <= REGION_HEIGHT);
    (yc * REGION_WIDTH) + xc
}

/// Indexes a planet-level block
pub fn planet_idx<N: Into<usize>>(x: N, y: N) -> usize {
    let xc = x.into();
    let yc = y.into();
    debug_assert!(xc < WORLD_WIDTH && yc < WORLD_HEIGHT);
    (WORLD_WIDTH * yc) + xc
}

// Indexes a planet-level block id back to x/y
pub fn idx_planet(idx: usize) -> (usize, usize) { (idx % WORLD_WIDTH, idx / WORLD_WIDTH) }

pub fn chunk_idx(x: usize, y: usize) -> usize { (y * CHUNK_SIZE) + x }
