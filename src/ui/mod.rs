use bevy::prelude::{Component, PluginGroup};

mod debug;
mod embark;
mod embark_region;
mod main_menu;
mod world_gen;

pub use debug::*;
pub use embark::*;
pub use embark_region::*;
pub use main_menu::*;
pub use world_gen::*;

#[derive(Component)]
pub struct BackgroundImage;

#[derive(Component)]
pub struct UiResources {
    pub worldgen_seed: String,
    pub worldgen_lacunarity: f32,
}

impl Default for UiResources {
    fn default() -> Self {
        Self { worldgen_lacunarity: 2.0, worldgen_seed: "Test Seed".to_string() }
    }
}

pub struct UIPlugins;
impl PluginGroup for UIPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(MainMenuPlugin)
            .add(WorldGenMenuPlugin)
            .add(EmbarkMenuPlugin)
            .add(EmbarkRegionPlugin)
            .add(DebugUiPlugin);
    }
}
