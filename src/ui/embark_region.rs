use crate::prelude::*;
use bevy_egui::*;

#[derive(Component)]
pub struct RegionGenUi;

pub fn embark_region_menu(
    mut commands: Commands,
    builder: ResMut<RegionBuilder>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Building Embark Region")
        .title_bar(true)
        .fixed_pos(egui::Pos2::new(10.0, 10.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(builder.status());
        });

    if builder.is_done() {
        commands.insert_resource(NextState(GameState::InGame));
    }
}

pub fn resume_embark_region(mut commands: Commands, embark: Res<EmbarkResources>) {
    let mut rb = RegionBuilder::new(embark.planet.clone(), PlanetLocation::new(embark.loc));
    rb.generate();
    commands.insert_resource(rb);
}

pub struct EmbarkRegionPlugin;
impl Plugin for EmbarkRegionPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::RegionGen, resume_embark_region)
            .add_system(embark_region_menu.run_in_state(GameState::RegionGen))
            .add_exit_system(GameState::RegionGen, despawn_all_with::<RegionGenUi>);
    }
}
