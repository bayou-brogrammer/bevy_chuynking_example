use crate::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy_egui::*;

#[derive(Component)]
pub struct EmbarkResources {
    pub planet: Planet,
    pub loc: IVec2,
}

#[derive(Component)]
pub struct EmbarkGrid;

pub fn resume_embark_menu(mut commands: Commands, ui: Res<UiAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui.backgrounds.clone(),
            sprite: TextureAtlasSprite::new(1),
            ..Default::default()
        })
        .insert(BackgroundImage {})
        .insert(EmbarkGrid {});

    let planet = load_planet();
    let tiles: Vec<(TilePos, u32)> = fill_tiles(&planet);

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
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, 999.0),
            ..Default::default()
        })
        .insert(EmbarkGrid);

    commands.insert_resource(EmbarkResources { planet, loc: IVec2::new(0, 0) });
}

pub fn embark_menu(
    wnds: Res<Windows>,
    mut commands: Commands,
    mut embark: ResMut<EmbarkResources>,
    mut egui_context: ResMut<EguiContext>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
    mut mouse_button_event_reader: EventReader<MouseButtonInput>,
) {
    // Mouse Picking
    let mut highlighed_location = IVec2::new(0, 0);
    let mut description = String::new();

    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width(), wnd.height());

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let mut camera_transform = q_camera.single_mut();

        camera_transform.scale = Vec3::new(1.5, 1.5, 1.0);

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let width = WORLD_WIDTH as f32 * 8.0;
        let height = WORLD_HEIGHT as f32 * 8.0;
        if pos_wld.y > -(height / 2.0)
            && pos_wld.y < height / 2.0
            && pos_wld.x > -(width / 2.0)
            && pos_wld.x < width / 2.0
        {
            highlighed_location = IVec2::new(
                ((pos_wld.x + (width / 2.0)) / 8.0) as i32,
                ((pos_wld.y + (height / 2.0)) / 8.0) as i32,
            );

            let pidx =
                planet_idx(highlighed_location.x as usize, highlighed_location.y as usize);
            let lb = &embark.planet.landblocks[pidx];
            let bidx = lb.biome_idx;
            description = format!(
              "{}.\n Avg Altitude: {}.\n Rainfall: {}mm.\n Variance: {}\nAvg Temperature: {} C",
              crate::raws::RAWS.read().biomes.areas[bidx].name,
              lb.height,
              lb.rainfall_mm,
              lb.variance,
              lb.temperature_c,
          );
        }
    }

    if highlighed_location != IVec2::ZERO {
        for event in mouse_button_event_reader.iter() {
            if event.state.is_pressed() && event.button == MouseButton::Left {
                embark.loc = highlighed_location;

                let crash_location = PlanetLocation::new(highlighed_location);
                let tile_loc = crash_location.to_world() + IVec2 { x: 128, y: 128 };
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
                commands.insert_resource(NextState(GameState::RegionGen));
            }
        }
    }

    egui::Window::new("Prepare to Evacuate the Colony Ship")
        .title_bar(true)
        .fixed_pos(egui::Pos2::new(500., 10.0))
        .show(egui_context.ctx_mut(), |ui| {
            if highlighed_location != IVec2::ZERO {
                ui.label("Select escape pod target");
                ui.label(format!("Tile: {highlighed_location}"));
                ui.label(description);
            }
        });
}

pub struct EmbarkMenuPlugin;
impl Plugin for EmbarkMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Embark, resume_embark_menu)
            .add_system(embark_menu.run_in_state(GameState::Embark))
            .add_exit_system(GameState::Embark, despawn_all_with::<EmbarkGrid>);
    }
}
