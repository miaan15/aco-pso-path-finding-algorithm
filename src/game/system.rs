use bevy::prelude::*;
use std::sync::Arc;

use crate::{
    algorithm::{
        grid::{Grid, GridCell},
        problem::Problem,
        solve::a_star::AStarStrategy,
    },
    game::{algorithm_resource::AlgorithmResource, control::GameState},
};

pub fn setup_game(mut commands: Commands) {
    setup_camera(commands);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn create_algorithm_resource() -> AlgorithmResource {
    let _grid = create_grid();
    AlgorithmResource {
        grid: _grid.clone(),
        problem: Problem {
            grid_map: _grid.clone(),
            start: None,
            goal: None,
        },
        path: None,
    }
}

fn create_grid() -> Arc<Grid> {
    let mut grid = Grid::new(20, 15, 50.0, Vec2::new(-600.0, -400.0));

    for x in 0..grid.width() {
        grid.set(x, 0, GridCell::Wall);
        grid.set(x, grid.height() - 1, GridCell::Wall);
    }
    for y in 0..grid.height() {
        grid.set(0, y, GridCell::Wall);
        grid.set(grid.width() - 1, y, GridCell::Wall);
    }

    for i in 5..15 {
        grid.set(i, 7, GridCell::Wall);
        grid.set(10, i, GridCell::Wall);
    }

    Arc::new(grid)
}

pub fn run_pathfinding(
    current_state: Res<State<GameState>>,
    mut algorithm_resource: ResMut<AlgorithmResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let (Some(start), Some(goal)) = (
        algorithm_resource.problem.start,
        algorithm_resource.problem.goal,
    ) {
        let a_star = AStarStrategy {};
        let path = a_star.path_finding(&algorithm_resource.problem);

        let complete_path = match path {
            Some(mut p) => {
                p.insert(0, start);
                p.push(goal);
                Some(p)
            }
            None => Some(vec![start, goal]),
        };

        algorithm_resource.path = complete_path;
        next_state.set(GameState::DoneRun);
    }
}
