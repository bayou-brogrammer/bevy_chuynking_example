use bevy::prelude::{Component, PluginGroup};

mod debug_ui;
mod embark;
mod planet_builder;

pub use debug_ui::*;
pub use embark::*;
pub use planet_builder::*;

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
        group.add(PlanetBuilderMenuPlugin).add(EmbarkMenuPlugin).add(DebugUiPlugin);
    }
}
