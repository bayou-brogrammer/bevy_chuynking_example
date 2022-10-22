use crate::prelude::*;

const COLS: [(u8, u8, u8); 32] = [
    RED, GREEN, BLUE, YELLOW, PURPLE, CYAN, ORANGE, PINK, RED, GREEN, BLUE, YELLOW, PURPLE,
    CYAN, ORANGE, PINK, RED, GREEN, BLUE, YELLOW, PURPLE, CYAN, ORANGE, PINK, RED, GREEN,
    BLUE, YELLOW, PURPLE, CYAN, ORANGE, PINK,
];

pub fn render_state(
    chunks: Query<&Chunk>,
    camera: Res<CameraView>,
    ctx: Res<BracketContext>,
    // loaded_chunks: Res<ChunkEntities>,
    renderables: Query<(&Glyph, &Position)>,
    player: Query<&FieldOfView, With<Player>>,
) {
    let mut batch = ctx.new_draw_batch();
    batch.target(LAYER_ZERO);
    batch.cls();

    let player_fov = player.single();
    chunks.iter().enumerate().for_each(|(idx, chunk)| {
        let mut color = COLS[idx % 32];

        chunk.tiles.iter().enumerate().for_each(|(idx, tile)| {
            let pt = Point::new(idx % CHUNK_SIZE, idx / CHUNK_SIZE)
                + Point { x: chunk.pos.x as i32, y: chunk.pos.y as i32 };

            let screen_pt = camera.world_to_screen(pt);

            let glyph = match tile {
                TileType::Floor => '.',
                TileType::Wall => '#',
                TileType::Water => '~',
                TileType::Grass => '"',
            };

            if player_fov.visible_tiles.contains(&pt) {
                color = WHITE;
            }

            batch.set(screen_pt, ColorPair::new(color, BLACK), to_cp437(glyph));
        });
    });

    // loaded_chunks.iter().enumerate().for_each(|(idx, (chunk_loc, chunk_entity))| {
    //     let chunk_loc = ChunkLocation::new(chunk_loc.x as usize, chunk_loc.y as usize);
    //     let color = COLS[idx % 32];
    //     let chunk = chunks.get(*chunk_entity).unwrap();

    // chunk.tiles.iter().enumerate().for_each(|(idx, tile)| {
    //     let pt = Point::new(idx % CHUNK_SIZE, idx / CHUNK_SIZE)
    //         + Point { x: chunk.pos.x as i32, y: chunk.pos.y as i32 };

    //     let glyph = match tile {
    //         TileType::Floor => '.',
    //         TileType::Wall => '#',
    //         TileType::Water => '~',
    //         TileType::Grass => '"',
    //     };
    //     batch.set(pt, ColorPair::new(color, BLACK), to_cp437(glyph));
    // });
    // });

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
