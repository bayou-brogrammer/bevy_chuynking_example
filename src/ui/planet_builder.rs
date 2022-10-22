use crate::prelude::*;
use bevy_egui::*;

#[derive(Component)]
pub struct WorldGenUi;

pub fn planet_builder_menu(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut egui_context: ResMut<EguiContext>,
) {
    let pb = PLANET_BUILDER.read();

    egui::Window::new("Planet Builder")
        .title_bar(true)
        .fixed_pos(egui::Pos2::new(25.0, 25.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(&pb.get_status());
        });

    if pb.is_done() && keyboard.just_pressed(KeyCode::Space) {
        // Bail out
        commands.insert_resource(NextState(GameState::Embark));
    }
}

pub fn resume_planet_builder_menu(res: Res<UiResources>) {
    println!("Building globe");

    // Get the builder inserted
    let pb = PLANET_BUILDER.read();
    pb.generate(&res.worldgen_seed, res.worldgen_lacunarity);
}

pub struct PlanetBuilderMenuPlugin;
impl Plugin for PlanetBuilderMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::PlanetGen, resume_planet_builder_menu)
            .add_system(planet_builder_menu.run_in_state(GameState::PlanetGen))
            .add_exit_system_set(
                GameState::PlanetGen,
                ConditionSet::new()
                    .with_system(despawn_all_with::<WorldGenUi>)
                    .with_system(rm_resource!(PlanetBuilder))
                    .into(),
            );
    }
}
