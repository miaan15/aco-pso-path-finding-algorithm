use bevy::prelude::*;
use std::sync::Arc;

use crate::algorithm::grid::Grid;

#[derive(Component)]
pub struct GridRenderer {
    pub wall_color: Color,
    pub grid: Arc<Grid>,
}

impl GridRenderer {
    pub fn new(grid: Arc<Grid>, wall_color: Color) -> Self {
        Self {
            wall_color,
            grid,
        }
    }
}
