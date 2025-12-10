use crate::algorithm::{grid::Grid, problem::Problem};
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Resource)]
pub struct AlgorithmResource {
    pub grid: Arc<Grid>,
    pub problem: Problem,
    pub path: Option<Vec<bevy::prelude::Vec2>>,
}
