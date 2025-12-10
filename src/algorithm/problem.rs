use std::sync::Arc;

use bevy::math::Vec2;
use crate::algorithm::grid::Grid;

#[derive(Debug, Clone)]
pub struct Problem {
    pub grid: Arc<Grid>,
    pub start: Option<Vec2>,
    pub goal: Option<Vec2>,
}
