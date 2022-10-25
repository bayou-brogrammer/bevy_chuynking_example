use crate::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;

/// Creates the requested chunks and attach them an ECS entity.
pub fn create_chunks(
    mut cmds: Commands,
    // mut chunk_entities: ResMut<ChunkEntities>,
    mut chunks_command_queue: ResMut<ChunkCommandQueue>,
) {
    // let task_pool = AsyncComputeTaskPool::get();
    // chunks_command_queue
    //     .create
    //     .drain(..)
    //     .map(|chunk_loc| {
    //         let planet_loc = chunk_loc.to_planet_location();
    //         BuildRenderChunkTask(
    //             task_pool.spawn(async move { build_render_chunk(planet_loc, chunk_loc) }),
    //         )
    //     })
    //     .for_each(|load_task| {
    //         cmds.spawn().insert(load_task);
    //     });

    let task_pool = AsyncComputeTaskPool::get();
    chunks_command_queue
        .create
        .drain(..)
        .map(|chunk_loc| ChunkLoadTask(task_pool.spawn(async move { load_chunk(chunk_loc) })))
        .for_each(|load_task| {
            cmds.spawn().insert(load_task);
        });
}

pub fn destroy_chunks(
    mut commands: Commands,
    chunks_q: Query<&Chunk>,
    mut chunk_entities: ResMut<ChunkEntities>,
    mut chunks: ResMut<ChunkMap<TileType, ChunkShape>>,
    mut chunks_command_queue: ResMut<ChunkCommandQueue>,
) {
    for command in chunks_command_queue.destroy.drain(..) {
        let chunk_entity = chunk_entities.detach_entity(command).unwrap();
        let chunk = chunks_q.get(chunk_entity).unwrap();
        save_chunk(chunk);
        chunks.remove(command.as_ivec2());
        commands.entity(chunk_entity).despawn_recursive();
    }

    // let task_pool = AsyncComputeTaskPool::get();

    // chunks_command_queue
    //     .destroy
    //     .drain(..)
    //     .map(|chunk_loc| {
    //         let chunk_entity = chunk_entities.entity(chunk_loc).unwrap();
    //         let chunk = chunks_q.get(chunk_entity).cloned().unwrap();

    //         (
    //             chunk_entity,
    //             ChunkSaveTask(task_pool.spawn(async move {
    //                 save_chunk(chunk);
    //             })),
    //         )
    //     })
    //     .for_each(|(entity, task)| {
    //         commands.entity(entity).insert(task);
    //     });
}

pub fn clear_dirty_chunks(mut dirty_chunks: ResMut<DirtyChunks>) { dirty_chunks.0.clear(); }
