use crate::prelude::*;
use bevy::{input::mouse::MouseButtonInput, math::ivec3};
use bevy_egui::*;
use bevy_simple_tilemap::prelude::*;

#[derive(Component)]
pub struct EmbarkResources {
    pub planet: Planet,
    pub tile_x: usize,
    pub tile_y: usize,
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
    let mut tiles: Vec<(IVec3, Option<Tile>)> = Vec::new();
    for y in 0..WORLD_HEIGHT as i32 {
        for x in 0..WORLD_WIDTH as i32 {
            let pidx = planet_idx(x as usize, y as usize);
            let biome_idx = planet.landblocks[pidx].biome_idx;
            let tile_index = crate::raws::RAWS.read().biomes.areas[biome_idx].embark_tile;
            let tx = x - WORLD_WIDTH as i32 / 2;
            let ty = y - WORLD_HEIGHT as i32 / 2;
            tiles.push((
                ivec3(tx, ty, 0),
                Some(Tile { sprite_index: tile_index as u32, ..Default::default() }),
            ));
        }
    }

    let mut tilemap = TileMap::default();
    tilemap.set_tiles(tiles);

    // Set up tilemap
    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: ui.embark_tiles.clone(),
        transform: Transform {
            scale: Vec3::splat(1.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn_bundle(tilemap_bundle).insert(EmbarkGrid {});
    commands.insert_resource(EmbarkResources { planet, tile_x: 0, tile_y: 0 });
}

pub fn embark_menu(
    wnds: Res<Windows>,
    mut commands: Commands,
    mut embark: ResMut<EmbarkResources>,
    mut egui_context: ResMut<EguiContext>,
    q_camera: Query<&Transform, With<Camera>>,
    mut mouse_button_event_reader: EventReader<MouseButtonInput>,
) {
    // Mouse Picking
    let mut tile_x = 0;
    let mut tile_y = 0;
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
        let camera_transform = q_camera.single();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        //eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);
        let width = WORLD_WIDTH as f32 * 8.0;
        let height = WORLD_HEIGHT as f32 * 8.0;
        if pos_wld.y > -(height / 2.0)
            && pos_wld.y < height / 2.0
            && pos_wld.x > -(width / 2.0)
            && pos_wld.x < width / 2.0
        {
            tile_x = ((pos_wld.x + (width / 2.0)) / 8.0) as i32;
            tile_y = ((pos_wld.y + (height / 2.0)) / 8.0) as i32;

            let pidx = planet_idx(tile_x as usize, tile_y as usize);
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

    if tile_x != 0 && tile_y != 0 {
        for event in mouse_button_event_reader.iter() {
            if event.state.is_pressed() && event.button == MouseButton::Left {
                println!("Selected Region: {}x{}", tile_x, tile_y);
                embark.tile_x = tile_x as usize;
                embark.tile_y = tile_y as usize;
                commands.insert_resource(NextState(GameState::InGame));
            }
        }
    }

    egui::Window::new("Prepare to Evacuate the Colony Ship")
        .title_bar(true)
        .fixed_pos(egui::Pos2::new(10.0, 10.0))
        .show(egui_context.ctx_mut(), |ui| {
            if tile_x != 0 && tile_y != 0 {
                ui.label("Select escape pod target");
                ui.label(format!("Tile: {}, {}", tile_x, tile_y));
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
