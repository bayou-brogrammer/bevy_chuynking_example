#![allow(clippy::too_many_arguments)]

pub mod raws;

mod bterm;
mod camera;
mod constants;
mod ecs;
mod loading;
mod saveload;
mod simulation;
mod tile_type;
mod ui;
mod util;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_bevy::prelude::*;
    pub use iyes_loopless::prelude::*;
    pub use serde::{Deserialize, Serialize};

    pub use bevy_ecs_tilemap::prelude::*;
    pub use bracket_bevy::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use direction::Direction;

    pub use crate::camera::*;
    pub use crate::constants::*;
    pub use crate::ecs::*;
    pub use crate::loading::*;
    pub use crate::raws::*;
    pub use crate::saveload::*;
    pub use crate::simulation::*;
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
pub use prelude::*;

fn main() {
    // Setup folders for saving
    setup_io_access().expect("Failed to setup IO access");

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
    .add_plugin(TilemapPlugin);

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
        .insert_resource(ChunkMap::<TileType, ChunkShape>::new(ChunkShape {}))
        .add_startup_system(setup);

    app.insert_resource(PlanetBuilder::new())
        .add_enter_system(GameState::PlanetGen, spawn_planet)
        .add_system(wait_for_planet_spawn.run_in_state(GameState::PlanetGenWait))
        .add_system(wait_for_region_spawn.run_in_state(GameState::RegionGenWait));

    // app.add_system_set(
    //     ConditionSet::new()
    //         .run_in_state(GameState::RegionGen)
    //         // .with_system(load_regions)
    //         .with_system(region_tile_applicator_system)
    //         .into(),
    // );

    app.run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default()).insert(BracketCamera);
}

pub fn spawn_planet(mut commands: Commands, rng: Res<RandomNumbers>) {
    let seed = rng.rand::<u64>();
    let worldgen_lacunarity = rng.range(2., 4.);

    let pb = PlanetBuilder::new();
    pb.generate(&seed.to_string(), worldgen_lacunarity);
    commands.insert_resource(pb);
    commands.insert_resource(NextState(GameState::PlanetGenWait));
}

pub fn wait_for_planet_spawn(mut commands: Commands, pb: Res<PlanetBuilder>) {
    if pb.is_done() {
        let planet = pb.get_planet().unwrap();
        let crash_location = PlanetLocation::new((0, 0).into());
        let tile_loc = crash_location.to_world();
        let pos = Position::with_tile_coords(crash_location, tile_loc.x, tile_loc.y);

        commands
            .spawn()
            .insert(Player)
            .insert(pos)
            .insert(Glyph::new(
                to_cp437('@'),
                ColorPair::new(WHITE, BLACK),
                RenderOrder::Actor,
            ))
            .insert(FieldOfView::new(8));

        commands.insert_resource(CurrentLocalPlayerChunk::new(
            crash_location.to_world(),
            tile_loc,
        ));
        commands.insert_resource(CameraView::new(Point::new(tile_loc.x, tile_loc.y)));

        let mut rb = RegionBuilder::new(planet, crash_location);
        rb.generate();

        commands.insert_resource(rb);
        commands.insert_resource(NextState(GameState::RegionGenWait));
    }
}

pub fn wait_for_region_spawn(mut commands: Commands, rb: Res<RegionBuilder>) {
    if rb.is_done() {
        commands.insert_resource(NextState(GameState::InGame));
    }
}
