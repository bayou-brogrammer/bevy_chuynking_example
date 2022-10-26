use crate::prelude::*;

mod fov;
mod movement;
mod player;
mod render;

use fov::*;
use movement::*;
use player::*;
use render::*;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenderingPlugin).add_plugin(PlayerPlugin);

        app.add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(movement)
                // .with_system(fov)
                .into(),
        );
    }
}
