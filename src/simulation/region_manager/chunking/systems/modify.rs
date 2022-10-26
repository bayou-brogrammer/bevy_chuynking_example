use crate::prelude::*;

/// Creates the requested chunks and attach them an ECS entity.
pub fn create_chunks(
    mut cmds: Commands,
    mut chunk_entities: ResMut<ChunkEntities>,
    mut chunks_command_queue: ResMut<ChunkCommandQueue>,
) {
    chunks_command_queue.create.drain(..).for_each(|chunk_create_location| {
        let chunk = load_chunk(chunk_create_location);
        chunk_entities.attach_entity(chunk_create_location, cmds.spawn().insert(chunk).id())
    });
}

pub fn destroy_chunks(
    mut commands: Commands,
    chunks_q: Query<&Chunk>,
    mut chunk_entities: ResMut<ChunkEntities>,
    mut chunks: ResMut<ChunkMap<TileType, ChunkShape>>,
    mut chunks_command_queue: ResMut<ChunkCommandQueue>,
) {
    for chunk_destroy_location in chunks_command_queue.destroy.drain(..) {
        let chunk_entity = chunk_entities.detach_entity(chunk_destroy_location).unwrap();
        let chunk = chunks_q.get(chunk_entity).unwrap();

        save_chunk(chunk);

        chunks.remove(chunk_destroy_location.as_ivec2());
        commands.entity(chunk_entity).despawn_recursive();
    }
}

pub fn clear_dirty_chunks(mut dirty_chunks: ResMut<DirtyChunks>) { dirty_chunks.0.clear(); }
