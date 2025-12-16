use crate::game::control::GameState;
use bevy::prelude::*;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        next_state.set(GameState::Cancel);
        return;
    }

    match current_state.get() {
        GameState::Idle => {
            if keyboard_input.just_pressed(KeyCode::Digit1) {
                next_state.set(GameState::SetStart);
            } else if keyboard_input.just_pressed(KeyCode::Digit2) {
                next_state.set(GameState::SetGoal);
            }
        }
        GameState::SetStart | GameState::SetGoal => {
            if keyboard_input.just_pressed(KeyCode::Digit1) {
                next_state.set(GameState::SetStart);
            } else if keyboard_input.just_pressed(KeyCode::Digit2) {
                next_state.set(GameState::SetGoal);
            }
        }
        _ => {}
    }
}

pub fn handle_mouse_selection(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut algorithm_resource: ResMut<crate::game::algorithm_resource::AlgorithmResource>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let cursor_screen_pos = window.unwrap().cursor_position().unwrap();
        let (camera, camera_transform) = camera_q.iter().next().unwrap();
        let ray = camera
            .viewport_to_world(camera_transform, cursor_screen_pos)
            .unwrap();
        let world_pos = Vec2::new(ray.origin.x, ray.origin.y);

        let pixel_size = algorithm_resource.grid.pixel_size();
        let true_pos = Vec2::new(
            (world_pos.x / pixel_size).floor() * pixel_size,
            (world_pos.y / pixel_size).floor() * pixel_size,
        ) + Vec2::splat(pixel_size / 2.0);

        match current_state.get() {
            GameState::SetStart => {
                algorithm_resource.problem.start = Some(true_pos);
                next_state.set(GameState::DoneStart);
                println!("Start set at: ({:.1}, {:.1})", true_pos.x, true_pos.y);
            }
            GameState::SetGoal => {
                algorithm_resource.problem.goal = Some(true_pos);
                next_state.set(GameState::DoneGoal);
                println!("Goal set at: ({:.1}, {:.1})", true_pos.x, true_pos.y);
            }
            _ => {}
        }
    }
}
