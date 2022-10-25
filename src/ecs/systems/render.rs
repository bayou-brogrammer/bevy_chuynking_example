use crate::prelude::*;

pub fn render_state(
    chunks: Query<&Chunk>,
    camera: Res<CameraView>,
    ctx: Res<BracketContext>,
    renderables: Query<(&Glyph, &Position)>,
    // player: Query<&FieldOfView, With<Player>>,
) {
    let mut batch = ctx.new_draw_batch();
    batch.target(LAYER_ZERO);
    batch.cls();

    chunks.iter().for_each(|chunk| {
        chunk.tiles.iter().enumerate().for_each(|(idx, tile)| {
            let pt = Point::new(idx % CHUNK_SIZE, idx / CHUNK_SIZE)
                + Point { x: chunk.location.x as i32, y: chunk.location.y as i32 };

            if camera.viewport.point_in_rect(pt) {
                let screen_pt = camera.world_to_screen(pt);
                let (glyph, color) = match tile {
                    TileType::Floor => ('.', WHITE),
                    TileType::Wall => ('#', WHITE),
                    TileType::Water => ('~', BLUE),
                    TileType::Sand => ('.', SANDYBROWN),
                    TileType::Soil => ('.', BROWN1),
                    TileType::Tree(tree_type) => match tree_type {
                        TreeType::Evergreen => ('T', GREEN),
                        TreeType::Deciduous => ('T', BROWN1),
                    },
                    TileType::Plant(plant_type) => match plant_type {
                        PlantType::Grass => ('"', GREEN),
                        PlantType::Daisy => ('d', YELLOW),
                        PlantType::Heather => ('h', PURPLE),
                    },
                    // TileType::Empty => (' ', BLACK),
                };

                batch.set(screen_pt, ColorPair::new(color, BLACK), to_cp437(glyph));
            }
        });
    });

    let mut entities = renderables.iter().collect::<Vec<_>>();
    entities.sort_by(|&a, &b| b.0.render_order.cmp(&a.0.render_order));
    for (glyph, pos) in entities {
        let screen_pt = camera.world_to_screen(pos.tile.to_point());
        batch.set(screen_pt, glyph.color, glyph.glyph);
    }

    ctx.submit_batch(BATCH_ZERO, batch);
}

pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_state.run_in_state(GameState::InGame));
    }
}
