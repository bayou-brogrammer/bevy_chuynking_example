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
            .insert_resource::<ChunkLoadRadius>(ChunkLoadRadius { horizontal: 2, vertical: 2 })
            .insert_resource(CurrentLocalPlayerChunk {
                chunk_min: IVec2::ZERO,
                world_pos: IVec2::ZERO,
            })
            .add_stage_after(
                CoreStage::Update,
                ChunkLoadingStage,
                SystemStage::parallel()
                    .with_system(
                        update_view_chunks
                            .label(ChunkLoadingSystem::UpdateViewChunks)
                            .with_run_criteria(update_view_chunks_criteria),
                    )
                    .with_system(
                        create_chunks
                            .label(ChunkLoadingSystem::CreateChunks)
                            .after(ChunkLoadingSystem::UpdateViewChunks),
                    )
                    .with_system_set(
                        SystemSet::new()
                            .label(ChunkLoadingSystem::CreateChunks)
                            .after(ChunkLoadingSystem::UpdateViewChunks)
                            .with_system(create_chunks)
                            .with_system(process_chunk_load),
                    ),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                SystemSet::new().with_system(destroy_chunks).with_system(process_chunk_save),
            )
            .add_system_to_stage(
                CoreStage::Last,
                clear_dirty_chunks.label(ChunkLoadingSystem::ClearDirtyChunks),
            );
    }
}
