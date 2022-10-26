use crate::prelude::*;
use bevy_egui::*;
use bracket_random::prelude::RandomNumberGenerator;

#[derive(Component)]
pub struct BackgroundImage;

pub struct MainMenuState {
    tagline: String,
}

impl Default for MainMenuState {
    fn default() -> Self { Self { tagline: tagline() } }
}

fn get_descriptive_noun(rng: &mut RandomNumberGenerator) -> String {
    rng.random_slice_entry(NOUNS).unwrap().to_string()
}

fn tagline() -> String {
    let mut rng = RandomNumberGenerator::new();
    let mut tagline = match rng.roll_dice(1, 8) {
        1 => "Histories",
        2 => "Chronicles",
        3 => "Sagas",
        4 => "Annals",
        5 => "Narratives",
        6 => "Recitals",
        7 => "Tales",
        8 => "Stories",
        _ => "",
    }
    .into();

    let first_noun = get_descriptive_noun(&mut rng);
    let mut second_noun = get_descriptive_noun(&mut rng);
    while first_noun == second_noun {
        second_noun = get_descriptive_noun(&mut rng);
    }

    tagline = format!("{tagline} of {first_noun} and {second_noun}");

    tagline
}

pub fn main_menu(
    mut commands: Commands,
    mms: Res<MainMenuState>,
    mut egui_context: ResMut<EguiContext>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    egui::Window::new("Random Game?")
        .auto_sized()
        .resizable(false)
        .fixed_pos(egui::Pos2::new(400., 300.))
        .show(egui_context.ctx_mut(), |ui| {
            ui.colored_label(egui::Color32::from_rgb(255, 0, 0), &mms.tagline);

            if ui.button("Create World").clicked() {
                commands.insert_resource(NextState(GameState::WorldGen));
            }

            if does_world_file_exist() && ui.button("Embark").clicked() {
                commands.insert_resource(NextState(GameState::Embark));
            }

            // Quit game option
            if ui.button("Quit").clicked() {
                app_exit_events.send(bevy::app::AppExit);
            }
        });

    egui::Window::new("Dedication").auto_sized().resizable(false).title_bar(false).show(
        egui_context.ctx_mut(),
        |ui| {
            ui.colored_label(egui::Color32::from_rgb(255, 255, 255), DEDICATION);
        },
    );

    egui::Window::new("Copyright").auto_sized().title_bar(false).resizable(false).show(
        egui_context.ctx_mut(),
        |ui| {
            ui.colored_label(egui::Color32::from_rgb(255, 255, 0), COPYRIGHT);
        },
    );
}

pub fn resume_main_menu(mut commands: Commands, ui: Res<UiAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui.backgrounds.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(BackgroundImage {});
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainMenuState>()
            .add_enter_system(GameState::MainMenu, resume_main_menu)
            .add_system(main_menu.run_in_state(GameState::MainMenu))
            .add_exit_system(GameState::MainMenu, despawn_all_with::<BackgroundImage>);
    }
}
