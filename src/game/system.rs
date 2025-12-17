use bevy::prelude::*;
use std::sync::Arc;

use crate::{
    algorithm::{
        grid::{Grid, GridCell},
        problem::Problem,
    },
    game::{algorithm_resource::AlgorithmResource, control::GameState},
};

pub fn setup_game(commands: Commands) {
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
            grid: _grid.clone(),
            start: None,
            goal: None,
        },
        path: None,
    }
}

fn create_grid() -> Arc<Grid> {
    let mut grid = Grid::new(19, 15, 60.0, 20.0, Vec2::new(-600.0, -400.0));

    for x in 0..grid.width() {
        grid.set(x, 0, GridCell::Wall);
        grid.set(x, grid.height() - 1, GridCell::Wall);
    }
    for y in 0..grid.height() {
        grid.set(0, y, GridCell::Wall);
        grid.set(grid.width() - 1, y, GridCell::Wall);
    }

    for i in 1..18 {
        grid.set(i, 8, GridCell::Wall);
    }
    for i in 1..14 {
        grid.set(9, i, GridCell::Wall);
    }

    grid.set(1, 8, GridCell::Air);
    grid.set(2, 8, GridCell::Air);
    grid.set(3, 8, GridCell::Air);
    grid.set(17, 8, GridCell::Air);
    grid.set(16, 8, GridCell::Air);
    grid.set(15, 8, GridCell::Air);
    grid.set(9, 13, GridCell::Air);
    grid.set(9, 1, GridCell::Air);
    grid.set(9, 2, GridCell::Air);
    grid.set(9, 9, GridCell::Air);
    grid.set(9, 8, GridCell::Air);
    grid.set(9, 7, GridCell::Air);

    for i in 0..3 {
        grid.set(13, 8 + i, GridCell::Wall);
        grid.set(13, 8 - i, GridCell::Wall);
        grid.set(5, 8 + i, GridCell::Wall);
        grid.set(5, 8 - i, GridCell::Wall);
    }

    Arc::new(grid)
}
