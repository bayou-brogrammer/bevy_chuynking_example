use crate::prelude::*;

pub fn movement(
    mut chunk_pos: ResMut<CurrentLocalPlayerChunk>,
    mut move_events: ResMut<Events<WantsToMove>>,
    mut pos_q: Query<(&mut Position, Option<&Player>)>,
) {
    for WantsToMove(entity, destination) in move_events.drain() {
        if let Ok((mut pos, player)) = pos_q.get_mut(entity) {
            pos.tile = RegionTileLocation::new(destination.x, destination.y);

            let world_pos: IVec2 = pos.tile.to_world();
            let nearest_chunk_origin = !IVec2::splat((CHUNK_SIZE - 1) as i32) & world_pos;
            if pos.chunk_min != nearest_chunk_origin.into() {
                pos.chunk_min = nearest_chunk_origin.into();
            }

            if player.is_some() {
                chunk_pos.world_pos = world_pos;
                chunk_pos.chunk_min = nearest_chunk_origin;
            }
        }
    }
}
