use crate::algorithm::{grid::Grid, problem::Problem};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct AlgorithmResource {
    pub grid: Arc<Mutex<Grid>>,
    pub problem: Problem,
    pub path: Option<Vec<bevy::prelude::Vec2>>,
    pub astar_path: Option<Vec<bevy::prelude::Vec2>>,
}
