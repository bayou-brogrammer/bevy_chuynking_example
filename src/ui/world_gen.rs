use crate::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::*;

#[derive(Component)]
pub struct DirtyTile;

pub fn resume_world_gen_menu(mut commands: Commands, ui: Res<UiAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui.backgrounds.clone(),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .insert(BackgroundImage {});

    // Get the builder inserted
    let pb = PlanetBuilder::new();
    commands.insert_resource(pb);
}

pub fn world_gen_menu(
    ui: Res<UiAssets>,
    mut commands: Commands,
    mut res: ResMut<UiResources>,
    mut egui_context: ResMut<EguiContext>,
    planet_builder: ResMut<PlanetBuilder>,
    embark_tiles: Query<Entity, With<EmbarkGrid>>,
    dirty_tiles: Query<Entity, With<DirtyTile>>,
) {
    egui::Window::new("Generate a world").fixed_pos(egui::Pos2::new(25.0, 25.0)).show(
        egui_context.ctx_mut(),
        |ui| {
            ui.label("Random Seed");
            ui.text_edit_singleline(&mut res.worldgen_seed);

            ui.label("Bumpiness");
            ui.add(
                egui::Slider::new(&mut res.worldgen_lacunarity, 2.0..=4.0)
                    .clamp_to_range(true),
            );

            if !planet_builder.is_building() {
                if ui.button("Create World").clicked() {
                    embark_tiles.for_each(|e| {
                        commands.entity(e).insert(DirtyTile);
                    });
                    planet_builder.generate(&res.worldgen_seed, res.worldgen_lacunarity);
                }

                if ui.button("Save Planet").clicked() {
                    planet_builder.save_planet();
                    commands.insert_resource(NextState(GameState::MainMenu));
                }
            }
        },
    );

    if let Some(tiles) = planet_builder.tile_info() {
        dirty_tiles.for_each(|e| commands.entity(e).despawn_recursive());

        let tilemap_size = TilemapSize { x: WORLD_WIDTH as u32, y: WORLD_HEIGHT as u32 };
        let tilemap_entity = commands.spawn().id();
        let mut tile_storage = TileStorage::empty(tilemap_size);

        tiles.iter().for_each(|(tile_pos, tile)| {
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: *tile_pos,
                    texture: TileTexture(*tile),
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .insert(EmbarkGrid)
                .id();

            tile_storage.set(tile_pos, tile_entity);
        });

        let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
        let grid_size = tile_size.into();

        commands
            .entity(tilemap_entity)
            .insert_bundle(TilemapBundle {
                tile_size,
                grid_size,
                size: tilemap_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(ui.embark_tiles.clone()),
                transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 0.0),
                ..Default::default()
            })
            .insert(EmbarkGrid);
    }
}

pub struct WorldGenMenuPlugin;
impl Plugin for WorldGenMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::WorldGen, resume_world_gen_menu)
            .add_system(world_gen_menu.run_in_state(GameState::WorldGen))
            .add_exit_system(GameState::WorldGen, despawn_all_with::<BackgroundImage>)
            .add_exit_system(GameState::WorldGen, despawn_all_with::<EmbarkGrid>);
    }
}
