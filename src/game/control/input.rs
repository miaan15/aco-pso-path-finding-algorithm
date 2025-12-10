use bevy::prelude::*;
use super::game_state::GameState;

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

pub fn on_done(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Idle);
    println!("Algorithm complete, returning to Idle");
}
