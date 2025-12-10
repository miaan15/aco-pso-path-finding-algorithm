use crate::algorithm::grid::Grid;
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Resource)]
pub struct AlgorithmResource {
    pub grid: Arc<Grid>,
}
