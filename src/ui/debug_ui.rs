use crate::prelude::*;
use bevy::{
    diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    ecs::schedule::ShouldRun,
    input::{keyboard::KeyboardInput, ButtonState},
};
use bevy_egui::*;

#[derive(Default)]
struct DebugUIState {
    display_debug_info: bool,
}

fn display_chunk_stats(
    chunks: Query<&Chunk>,
    mut egui: ResMut<EguiContext>,
    dirty_chunks: Option<Res<DirtyChunks>>,
    loaded_chunks: Option<Res<ChunkEntities>>,
    player_q: Query<&Position, With<Player>>,
    chunk_command_queue: Option<ResMut<ChunkCommandQueue>>,
    chunk_loading_radius: Option<ResMut<ChunkLoadRadius>>,
) {
    egui::Window::new("Chunking").show(egui.ctx_mut(), |ui| {
        if let Ok(player_pos) = player_q.get_single() {
            ui.heading("Player Chunking");
            ui.label(format!("Player region: {:?}", player_pos.region));
            ui.label(format!("Player chunk: {:?}", player_pos.chunk_min));
            ui.label(format!("Player tile position: {:?}", player_pos.tile));
            ui.separator();
        }

        ui.heading("Chunks");

        if let Some(loaded_chunks) = loaded_chunks {
            ui.label(format!("Loaded chunk count: {}", loaded_chunks.len()));
            ui.label(format!(
                "Chunks invalidations (per frame):  {}",
                dirty_chunks.unwrap().num_dirty()
            ));

            ui.separator();

            if let Some(mut chunk_command_queue) = chunk_command_queue {
                if ui.button("Clear loaded chunks").clicked() {
                    chunk_command_queue.queue_unload(loaded_chunks.iter_keys());
                }
            }

            if let Some(mut chunk_loading_radius) = chunk_loading_radius {
                ui.label(format!(
                    "Horizontal chunk loading radius: H: {} / V: {}",
                    chunk_loading_radius.horizontal, chunk_loading_radius.vertical,
                ));

                if ui.button("Increment").clicked() {
                    chunk_loading_radius.horizontal += 1;
                    chunk_loading_radius.vertical += 1;
                }
                if ui.button("Decrement").clicked() {
                    chunk_loading_radius.horizontal -= 1;
                    chunk_loading_radius.vertical -= 1;
                }
                // ui.add(egui::Slider::new(&mut chunk_loading_radius.horizontal, 1..=32));
                // ui.text_edit_singleline(&mut ui_state.label);
                ui.separator();
            }

            loaded_chunks.iter().for_each(|(chunk_key, chunk_entity)| {
                ui.separator();

                let chunk = chunks.get(*chunk_entity).unwrap();
                ui.label(format!("Chunk Key {:?}", chunk_key));
                ui.label(format!("Chunk tiles {:?}", chunk.tiles.len()));
                ui.label(format!("Chunk tiles {:?}", chunk.tiles.len()));
            });
        }
    });
}

fn display_debug_stats(
    mut egui: ResMut<EguiContext>,
    diagnostics: Res<Diagnostics>,
    game_state: Res<CurrentState<GameState>>,
) {
    egui::Window::new("game stuff").show(egui.ctx_mut(), |ui| {
        ui.label(format!("Current Game State: {:?}", game_state.0));

        ui.label(format!(
            "Avg. FPS: {:.02}",
            diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .unwrap()
                .average()
                .unwrap_or_default()
        ));

        ui.label(format!(
            "Total Entity count: {}",
            diagnostics
                .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .unwrap()
                .value()
                .unwrap_or_default()
        ));
    });
}

fn toggle_debug_ui_displays(
    mut ui_state: ResMut<DebugUIState>,
    mut inputs: EventReader<KeyboardInput>,
) {
    for input in inputs.iter() {
        match input.key_code {
            Some(key_code)
                if key_code == KeyCode::F3 && input.state == ButtonState::Pressed =>
            {
                ui_state.display_debug_info = !ui_state.display_debug_info;
            }

            _ => {}
        }
    }
}

fn display_debug_ui_criteria(ui_state: Res<DebugUIState>) -> ShouldRun {
    if ui_state.display_debug_info {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugUIState>()
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(EntityCountDiagnosticsPlugin)
            .add_stage_after(
                CoreStage::PostUpdate,
                "debug_ui_stage",
                SystemStage::parallel().with_system(toggle_debug_ui_displays).with_system_set(
                    SystemSet::new()
                        .with_system(display_debug_stats)
                        .with_system(display_chunk_stats)
                        .with_run_criteria(display_debug_ui_criteria),
                ),
            );
    }
}
