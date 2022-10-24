use crate::prelude::*;

mod bounds;
mod chunk;
mod chunk_resources;
mod entities;
mod iter;
mod location;
mod storage;
mod systems;

pub use bounds::*;
pub use chunk::*;
pub use chunk_resources::*;
pub use entities::*;
pub use iter::*;
pub use location::*;
pub use storage::*;
pub use systems::*;

pub struct ChunkingPlugin;
impl Plugin for ChunkingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ChunkCommandQueue>()
            .init_resource::<DirtyChunks>()
            .init_resource::<ChunkEntities>()
            .insert_resource::<ChunkLoadRadius>(ChunkLoadRadius { horizontal: 4, vertical: 4 })
            .insert_resource::<CurrentLocalPlayerChunk>(CurrentLocalPlayerChunk::ZERO)
            .add_stage_after(
                CoreStage::Update,
                ChunkLoadingStage,
                SystemStage::parallel()
                    .with_system_set(
                        ConditionSet::new()
                            .label(ChunkLoadingSystem::UpdateViewChunks)
                            .run_in_state(GameState::InGame)
                            .run_if_resource_exists::<ChunkLoadRadius>()
                            .run_if_resource_exists::<CurrentLocalPlayerChunk>()
                            .with_system(update_view_chunks)
                            .into(),
                    )
                    .with_system_set(
                        ConditionSet::new()
                            .label(ChunkLoadingSystem::CreateChunks)
                            .after(ChunkLoadingSystem::UpdateViewChunks)
                            .run_in_state(GameState::InGame)
                            .with_system(create_chunks)
                            .into(),
                    )
                    .with_system_set(
                        ConditionSet::new()
                            .label(ChunkLoadingSystem::CreateChunks)
                            .after(ChunkLoadingSystem::UpdateViewChunks)
                            .run_in_state(GameState::InGame)
                            .with_system(create_chunks)
                            .with_system(process_chunk_load)
                            .into(),
                    ),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(destroy_chunks)
                    .with_system(process_chunk_save)
                    .into(),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                ConditionSet::new()
                    .label(ChunkLoadingSystem::ClearDirtyChunks)
                    .run_in_state(GameState::InGame)
                    .with_system(clear_dirty_chunks)
                    .into(),
            );
    }
}
