use super::*;

pub fn load_raws(
    commands: &mut Commands,
    res: &mut ResMut<LoadingResource>,
    ui: &mut egui::Ui,
) {
    res.cycle += 1;
    ui.label("Loading Raw Files");
    crate::raws::load_raws();
    commands.insert_resource(NextState(GameState::PlanetGen));
}
