use bevy::prelude::*;
use std::sync::{Arc, Mutex};

use crate::algorithm::grid::Grid;

#[derive(Component)]
pub struct GridRenderer {
    pub wall_color: Color,
    pub grid: Grid,
}

impl GridRenderer {
    pub fn new(grid: Grid, wall_color: Color) -> Self {
        Self {
            wall_color,
            grid,
        }
    }
}
