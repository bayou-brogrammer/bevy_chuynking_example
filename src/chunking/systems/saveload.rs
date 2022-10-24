use crate::prelude::*;
use bevy::tasks::Task;
use futures_lite::future;

#[derive(Debug, Component)]
pub struct ChunkSaveTask(pub Task<()>);

#[derive(Debug, Component)]
pub struct ChunkLoadTask(pub Task<Chunk>);

pub fn chunk_filename(chunk_id: ChunkLocation) -> String {
    chunk_save_location(&format!("{}_{}.chunk", chunk_id.x, chunk_id.y))
}

pub fn load_chunk(chunk_id: ChunkLocation) -> Chunk {
    match load_data::<Chunk>(chunk_filename(chunk_id)) {
        Ok(chunk) => chunk,
        Err(_) => {
            println!("Chunk not found on disk, generating {:?}", chunk_id);
            let mut chunk = Chunk::new(0, chunk_id);
            chunk.generate();
            chunk
        }
    }
}

pub fn save_chunk(chunk: &Chunk) {
    let chunk_id = chunk.pos;
    if let Err(err) = save_data(chunk_filename(chunk_id), chunk) {
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
            chunks.remove(chunk.pos.as_ivec2());
            let other_entity = chunk_entities.detach_entity(chunk.pos).unwrap();

            println!("Chunk entity: {:?} / Other: {:?}", chunk_entity, other_entity);

            commands.entity(chunk_entity).remove::<ChunkSaveTask>();
            commands.entity(chunk_entity).despawn_recursive();
            commands.entity(other_entity).despawn_recursive();
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
