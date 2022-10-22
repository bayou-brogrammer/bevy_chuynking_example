#![allow(clippy::too_many_arguments)]

pub mod raws;

mod bterm;
mod camera;
mod chunking;
mod ecs;
mod generation;
mod loading;
mod region;
mod saveload;
mod tile_type;
mod ui;
mod util;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_bevy::prelude::*;
    pub use iyes_loopless::prelude::*;
    pub use serde::{Deserialize, Serialize};

    pub use bracket_bevy::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use direction::Direction;

    pub use crate::camera::*;
    pub use crate::chunking::*;
    pub use crate::ecs::*;
    pub use crate::generation::*;
    pub use crate::loading::*;
    pub use crate::raws::*;
    pub use crate::region::*;
    pub use crate::saveload::*;
    pub use crate::tile_type::*;
    pub use crate::ui::*;
    pub use crate::util::*;

    pub use crate::{impl_default, impl_new, rm_resource};

    pub const LAUNCHER_TITLE: &str = "Blood Oath";
    pub const WINDOW_WIDTH: f32 = 960.0;
    pub const WINDOW_HEIGHT: f32 = 720.0;

    // Batches
    pub const BATCH_ZERO: usize = 0;

    // Layers
    pub const LAYER_ZERO: usize = 0;
}
use bevy::render::texture::ImageSettings;
use bevy_egui::EguiPlugin;
use bevy_simple_tilemap::prelude::SimpleTileMapPlugin;
pub use prelude::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: String::from("Game"),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(ImageSettings::default_nearest());

    app.add_plugins_with(DefaultPlugins, |group| {
        #[cfg(feature = "bundled")]
        group.add_before::<bevy::asset::AssetPlugin, _>(
            bevy_embedded_assets::EmbeddedAssetPlugin,
        );
        group
    })
    .add_plugin(EguiPlugin)
    .add_plugin(SimpleTileMapPlugin);

    // Game States Setup
    app.add_loopless_state(GameState::Assets);
    app.insert_resource(TurnState::AwaitingInput);

    app.add_plugin(LoadingPlugin)
        .add_plugin(bterm::setup_bterm())
        .add_plugins(UIPlugins)
        .add_plugin(EcsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ChunkingPlugin);

    app.init_resource::<UiResources>()
        .insert_resource(ChunkMap::<TileType, ChunkShape>::new(ChunkShape {}));

    app.run();
}
