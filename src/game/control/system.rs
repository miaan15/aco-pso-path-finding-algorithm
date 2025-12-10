use bevy::prelude::*;
use crate::game::control::GameState;

pub fn on_done_run(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Idle);
    println!("Algorithm complete, returning to Idle");
}

pub fn on_done_start(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Idle);
    println!("Algorithm complete, returning to Idle");
}

pub fn on_done_goal(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Idle);
    println!("Algorithm complete, returning to Idle");
}

