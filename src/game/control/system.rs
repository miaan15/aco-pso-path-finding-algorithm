use crate::game::control::GameState;
use crate::game::pathfinding_system::{PathfindingStrategy, reset_pathfinding};
use crate::game::solve_renderer::{GoalPoint, StartPoint, render_start_goal};
use crate::game::click_position::ClickPosition;
use crate::algorithm::grid::GridCell;
use bevy::prelude::*;

pub fn on_cancel(
    mut strategy_resource: ResMut<PathfindingStrategy>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    reset_pathfinding(strategy_resource);
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

pub fn on_done_place(
    mut algorithm_resource: ResMut<crate::game::algorithm_resource::AlgorithmResource>,
    click_position: Res<ClickPosition>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let (Some(grid_x), Some(grid_y)) = (click_position.grid_x, click_position.grid_y) {
        // Don't place walls on the border
        if grid_x > 0 && grid_x < 19 && grid_y > 0 && grid_y < 14 {
            let mut grid = algorithm_resource.grid.lock().unwrap();
            grid.set(grid_x, grid_y, GridCell::Wall);
            println!("Placed wall at grid: ({}, {})", grid_x, grid_y);
        }
    }
    next_state.set(GameState::Idle);
}

pub fn on_done_delete(
    mut algorithm_resource: ResMut<crate::game::algorithm_resource::AlgorithmResource>,
    click_position: Res<ClickPosition>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let (Some(grid_x), Some(grid_y)) = (click_position.grid_x, click_position.grid_y) {
        // Don't delete walls on the border
        if grid_x > 0 && grid_x < 19 && grid_y > 0 && grid_y < 14 {
            let mut grid = algorithm_resource.grid.lock().unwrap();
            grid.set(grid_x, grid_y, GridCell::Air);
            println!("Deleted wall at grid: ({}, {})", grid_x, grid_y);
        }
    }
    next_state.set(GameState::Idle);
}
