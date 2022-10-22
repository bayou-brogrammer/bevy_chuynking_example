use crate::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

pub use components::*;
pub use events::*;
pub use resources::*;
pub use systems::*;

pub struct EcsPlugin;
impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EventPlugin).add_plugin(SystemsPlugin);
    }
}
