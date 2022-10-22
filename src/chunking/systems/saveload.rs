use crate::prelude::*;
use bevy::tasks::Task;
use futures_lite::future;

#[derive(Debug, Component)]
pub struct ChunkSaveTask(pub Task<()>);

#[derive(Debug, Component)]
pub struct ChunkLoadTask(pub Task<Chunk>);

pub fn load_chunk(chunk_id: ChunkLocation) -> Chunk {
    match load_data::<Chunk>(format!("savegame/chunk_{}_{}.dat", chunk_id.x, chunk_id.y)) {
        Ok(chunk) => chunk,
        Err(_) => Chunk::new(chunk_id),
    }
}

pub fn save_chunk(chunk: Chunk) {
    let chunk_id = chunk.pos;
    if let Err(err) =
        save_data(format!("savegame/chunk_{}_{}.dat", chunk_id.x, chunk_id.y), chunk)
    {
        println!("Failed to save chunk: {:?}", err);
    }
}

pub fn process_chunk_save(
    mut commands: Commands,
    mut chunk_entities: ResMut<ChunkEntities>,
    mut chunks: ResMut<ChunkMap<TileType, ChunkShape>>,
    mut saved_chunks: Query<(Entity, &Chunk, &mut ChunkSaveTask)>,
) {
    for (chunk_entity, chunk, mut task) in saved_chunks.iter_mut() {
        if future::block_on(future::poll_once(&mut task.0)).is_some() {
            commands.entity(chunk_entity).remove::<ChunkSaveTask>();
            commands.entity(chunk_entity).despawn_recursive();
            chunks.remove(chunk.pos.as_ivec2());
            chunk_entities.detach_entity(chunk.pos);
        }
    }
}

pub fn process_chunk_load(
    mut commands: Commands,
    mut chunk_entities: ResMut<ChunkEntities>,
    mut saved_chunks: Query<(Entity, &mut ChunkLoadTask)>,
) {
    for (chunk_entity, mut task) in saved_chunks.iter_mut() {
        if let Some(chunk) = future::block_on(future::poll_once(&mut task.0)) {
            chunk_entities.attach_entity(chunk.pos, commands.spawn().insert(chunk).id());
            commands.entity(chunk_entity).despawn_recursive();
        }
    }
}
