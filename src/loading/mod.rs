use crate::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_egui::*;

mod loaders;

pub struct LoadingResource {
    cycle: u8,
}

pub fn resume_loading_screen(mut commands: Commands) {
    commands.insert_resource(LoadingResource { cycle: 0 });
}

pub fn loading_screen(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut res: ResMut<LoadingResource>,
) {
    println!("Loading screen");
    egui::Window::new("Loading - Please Wait")
        .resizable(false)
        .title_bar(false)
        .fixed_pos(egui::Pos2::new(500.0, 200.0))
        .show(egui_context.ctx_mut(), |ui| match res.cycle {
            0..=2 => res.cycle += 1,
            3 => loaders::load_raws(&mut commands, &mut res, ui),
            _ => {}
        });
}

#[derive(AssetCollection)]
pub struct UiAssets {
    #[asset(key = "backgrounds")]
    pub backgrounds: Handle<TextureAtlas>,

    #[asset(key = "embark_tiles")]
    pub embark_tiles: Handle<TextureAtlas>,

    #[asset(key = "worldgen_tiles")]
    pub worldgen_tex: Handle<Image>,
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Assets)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                    "loader/ui-assets.assets",
                ])
                .with_collection::<UiAssets>()
                .continue_to_state(GameState::Loading),
        );

        app.add_enter_system(GameState::Loading, resume_loading_screen)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Loading)
                    .with_system(loading_screen)
                    .into(),
            )
            .add_exit_system(GameState::Loading, rm_resource!(LoadingResource));
    }
}
