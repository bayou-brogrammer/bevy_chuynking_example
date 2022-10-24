use crate::prelude::*;

pub fn fov(
    loaded_chunks: Res<ChunkEntities>,
    mut fov_q: Query<(&Position, &mut FieldOfView)>,
) {
    let regions = REGIONS.read();
    for (pos, mut fov) in fov_q.iter_mut().filter(|(_, f)| f.is_dirty) {
        fov.is_dirty = false;

        let pt = pos.tile.to_point();
        let chunk_key = pos.chunk_min;

        let region = regions.get_region(pos.region.to_region_index()).unwrap();

        if loaded_chunks.entity(chunk_key).is_some() {
            fov.visible_tiles = field_of_view_set(Point::new(pt.x, pt.y), fov.radius, region);
        } else {
            println!("FOV: No chunk entity found for {:?}", chunk_key);
        }
    }
}
