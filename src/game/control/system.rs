use crate::game::control::GameState;
use crate::game::solve_renderer::{GoalPoint, StartPoint, render_start_goal};
use bevy::prelude::*;

pub fn on_cancel(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Idle);
    println!("Cancel");
}

pub fn on_done_start(
    commands: Commands,
    algorithm_resource: Res<crate::game::algorithm_resource::AlgorithmResource>,
    start_point_query: Query<Entity, With<StartPoint>>,
    goal_point_query: Query<Entity, With<GoalPoint>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    render_start_goal(
        commands,
        algorithm_resource,
        start_point_query,
        goal_point_query,
    );
    next_state.set(GameState::Idle);
    println!("Set Start");
}

pub fn on_done_goal(
    commands: Commands,
    algorithm_resource: Res<crate::game::algorithm_resource::AlgorithmResource>,
    start_point_query: Query<Entity, With<StartPoint>>,
    goal_point_query: Query<Entity, With<GoalPoint>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    render_start_goal(
        commands,
        algorithm_resource,
        start_point_query,
        goal_point_query,
    );
    next_state.set(GameState::Idle);
    println!("Set Goal");
}
