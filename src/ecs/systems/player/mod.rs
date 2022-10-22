use crate::prelude::*;

pub fn player_input(
    mut keys: ResMut<Input<KeyCode>>,
    mut move_events: EventWriter<WantsToMove>,

    player_q: Query<(Entity, &mut Position), With<Player>>,
    mut camera_q: Query<&mut OrthographicProjection, With<BracketCamera>>,
) {
    let key = keys.get_pressed().next().cloned();
    if let Some(key) = key {
        let mut delta = Point::new(0, 0);
        let (player_entity, pos) = player_q.single();

        match key {
            KeyCode::Left => delta.x -= 1,
            KeyCode::Right => delta.x += 1,
            KeyCode::Down => delta.y += 1,
            KeyCode::Up => delta.y -= 1,
            KeyCode::Equals | KeyCode::Minus => zoom_camera(key, &mut camera_q.single_mut()),

            _ => {}
        }

        // move to new position
        if delta.x != 0 || delta.y != 0 {
            let destination = pos.tile.to_point() + delta;
            move_events.send(WantsToMove(player_entity, destination));
        }

        // reset keyboard, bevys bug when changing states
        keys.reset(key);
    }
}

fn zoom_camera(key: KeyCode, proj: &mut OrthographicProjection) {
    if key == KeyCode::Equals || key == KeyCode::Plus {
        proj.scale -= 0.1;
    }

    if key == KeyCode::Minus {
        proj.scale += 0.1;
    }

    proj.scale = proj.scale.clamp(0.1, 3.0);
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_if_resource_equals(TurnState::AwaitingInput)
                .with_system(player_input)
                .into(),
        );
    }
}
