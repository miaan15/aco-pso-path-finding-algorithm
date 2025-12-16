use crate::algorithm::solve::hybrid::HybridStrategy;
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Resource)]
pub struct PathfindingStrategy {
    pub strategy: HybridStrategy,
}

impl PathfindingStrategy {
    pub fn new(grid: Arc<crate::algorithm::grid::Grid>) -> Self {
        Self {
            strategy: HybridStrategy::new(grid),
        }
    }
}

pub fn update_pathfinding(
    mut strategy_resource: ResMut<PathfindingStrategy>,
    mut algorithm_resource: ResMut<crate::game::algorithm_resource::AlgorithmResource>,
) {
    let start = algorithm_resource.problem.start;
    let goal = algorithm_resource.problem.goal;

    if let (Some(start), Some(goal)) = (start, goal) {
        let path = strategy_resource
            .strategy
            .path_finding(Some(start), Some(goal));

        if algorithm_resource.path != path {
            algorithm_resource.path = path;
        }
    }
}


pub fn reset_pathfinding(
    mut strategy_resource: ResMut<PathfindingStrategy>,
) {
    strategy_resource.strategy.reset();
}
