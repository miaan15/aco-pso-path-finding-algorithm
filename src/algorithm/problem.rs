use std::sync::{Arc, Mutex};

use bevy::math::Vec2;
use crate::algorithm::grid::Grid;

#[derive(Debug, Clone)]
pub struct Problem {
    pub grid: Arc<Mutex<Grid>>,
    pub start: Option<Vec2>,
    pub goal: Option<Vec2>,
}
