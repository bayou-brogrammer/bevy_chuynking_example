use crate::prelude::*;

// Screens
// pub const TILE_SIZE: f32 = 8.0;
pub const SCREEN_WIDTH: i32 = 96;
pub const SCREEN_HEIGHT: i32 = 96;

pub fn setup_bterm() -> BTermBuilder {
    BTermBuilder::empty()
        .with_random_number_generator(true)
        .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
        // .with_font("terminal10x10.png", 16, 16, (10.0, 10.0))
        // .with_font("terminal12x12.png", 16, 16, (12.0, 12.0))
        // .with_font("terminal16x16.png", 16, 16, (16.0, 16.0))
        // .with_font("vga8x16.png", 16, 16, (8.0, 16.0))
        .with_simple_console(0, SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_background(true)
}
