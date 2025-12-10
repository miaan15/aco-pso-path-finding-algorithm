use bevy::prelude::*;
use crate::game::control::GameState;
use crate::game::solve_renderer::{GoalPoint, StartPoint, render_start_goal};

pub fn on_done_run(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Idle);
    println!("Algorithm complete, returning to Idle");
}

pub fn on_done_start(
    mut commands: Commands,
    algorithm_resource: Res<crate::game::algorithm_resource::AlgorithmResource>,
    start_point_query: Query<Entity, With<StartPoint>>,
    goal_point_query: Query<Entity, With<GoalPoint>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    render_start_goal(commands, algorithm_resource, start_point_query, goal_point_query);
    next_state.set(GameState::Idle);
    println!("Start point rendered, returning to Idle");
}

pub fn on_done_goal(
    mut commands: Commands,
    algorithm_resource: Res<crate::game::algorithm_resource::AlgorithmResource>,
    start_point_query: Query<Entity, With<StartPoint>>,
    goal_point_query: Query<Entity, With<GoalPoint>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    render_start_goal(commands, algorithm_resource, start_point_query, goal_point_query);
    next_state.set(GameState::Idle);
    println!("Goal point rendered, returning to Idle");
}

