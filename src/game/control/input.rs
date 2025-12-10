use bevy::prelude::*;
use crate::game::control::GameState;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        next_state.set(GameState::SetStart);
        println!("State: SetStart");
    } else if keyboard_input.just_pressed(KeyCode::KeyG) {
        next_state.set(GameState::SetGoal);
        println!("State: SetGoal");
    } else if keyboard_input.just_pressed(KeyCode::KeyC) {
        next_state.set(GameState::Idle);
        println!("State: Idle (cancelled)");
    } else if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Loading);
        println!("State: Loading (running algorithm)");
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
        let ray = camera.viewport_to_world(camera_transform, cursor_screen_pos).unwrap();
        let world_pos = Vec2::new(ray.origin.x, ray.origin.y);

        match current_state.get() {
            GameState::SetStart => {
                algorithm_resource.problem.start = Some(world_pos);
                next_state.set(GameState::DoneStart);
                println!("Start set at: ({:.1}, {:.1})", world_pos.x, world_pos.y);
            }
            GameState::SetGoal => {
                algorithm_resource.problem.goal = Some(world_pos);
                next_state.set(GameState::DoneGoal);
                println!("Goal set at: ({:.1}, {:.1})", world_pos.x, world_pos.y);
            }
            _ => {}
        }
    }
}
