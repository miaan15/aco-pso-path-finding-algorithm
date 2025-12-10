use std::sync::Arc;

use bevy::math::Vec2;
use crate::algorithm::grid::Grid;

#[derive(Debug, Clone)]
pub struct Problem {
    pub grid_map: Arc<Grid>,
    pub start: Vec2,
    pub goal: Vec2,
}
