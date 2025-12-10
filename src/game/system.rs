use bevy::prelude::*;
use std::sync::Arc;

use crate::{
    algorithm::grid::{Grid, GridCell},
    game::algorithm_resource::AlgorithmResource,
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn create_algorithm_resource() -> AlgorithmResource {
    AlgorithmResource {
        grid: create_grid(),
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
