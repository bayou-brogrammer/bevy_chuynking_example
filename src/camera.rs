use crate::{
    bterm::{SCREEN_HEIGHT, SCREEN_WIDTH},
    prelude::*,
};

pub const VIEWPORT_X_OFFSET: i32 = SCREEN_WIDTH / 2;
pub const VIEWPORT_Y_OFFSET: i32 = SCREEN_HEIGHT / 2;
pub const VIEWPORT_WIDTH: i32 = 384;
pub const VIEWPORT_HEIGHT: i32 = 384;

#[derive(Component, Debug, Copy, Clone)]
pub struct CameraView {
    pub viewport: Rect,
    pub player_pos: Point,
}

impl CameraView {
    pub fn new(player_pos: Point) -> Self {
        Self {
            viewport: Rect::with_size(
                player_pos.x - VIEWPORT_X_OFFSET,
                player_pos.y - VIEWPORT_Y_OFFSET,
                VIEWPORT_WIDTH,
                VIEWPORT_HEIGHT,
            ),
            player_pos,
        }
    }

    fn calculate_viewport(&self) -> Rect {
        Rect::with_size(
            self.player_pos.x - VIEWPORT_X_OFFSET,
            self.player_pos.y - VIEWPORT_Y_OFFSET,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
        )
    }

    pub fn on_player_move(&mut self, player_pos: Point) {
        self.player_pos = player_pos;
        self.viewport = self.calculate_viewport();
    }

    pub fn world_to_screen(&self, pt: Point) -> Point {
        let bot = pt - self.player_pos;
        bot + Point::new(VIEWPORT_X_OFFSET, VIEWPORT_Y_OFFSET)
    }

    pub fn world_to_screen_text(&self, pt: Point) -> Point {
        let ws = self.world_to_screen(pt);
        ws * Point::new(2, 1)
    }

    pub fn screen_to_world(&self, mouse_pt: Point) -> Point {
        Point::new(mouse_pt.x + self.viewport.x1, mouse_pt.y + self.viewport.y1)
    }
}

fn camera_follow(
    mut camera: ResMut<CameraView>,
    player_q: Query<&Position, (With<Player>, Changed<Position>)>,
) {
    if let Ok(player_pos) = player_q.get_single() {
        camera.on_player_move(player_pos.tile.to_point());
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(CoreStage::Last, "camera_stage", SystemStage::parallel());

        app.add_system_set_to_stage(
            "camera_stage",
            ConditionSet::new()
                .label("camera_stage")
                .run_in_state(GameState::InGame)
                .with_system(camera_follow)
                .into(),
        );
    }
}
