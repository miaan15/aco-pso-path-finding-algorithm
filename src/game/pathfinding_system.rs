use crate::algorithm::solve::{hybrid::HybridStrategy, a_star::AStarStrategy};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct PathfindingStrategy {
    pub hybrid_strategy: HybridStrategy,
    pub astar_strategy: AStarStrategy,
}

impl PathfindingStrategy {
    pub fn new(grid: Arc<Mutex<crate::algorithm::grid::Grid>>) -> Self {
        Self {
            hybrid_strategy: HybridStrategy::new(grid.clone()),
            astar_strategy: AStarStrategy::new(grid.clone()),
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
        let hybrid_path = strategy_resource
            .hybrid_strategy
            .path_finding(Some(start), Some(goal));

        let astar_path = strategy_resource
            .astar_strategy
            .path_finding(Some(start), Some(goal));

        if algorithm_resource.path != hybrid_path {
            algorithm_resource.path = hybrid_path;
        }
        if algorithm_resource.astar_path != astar_path {
            algorithm_resource.astar_path = astar_path;
        }
    }
}


pub fn reset_pathfinding(
    mut strategy_resource: ResMut<PathfindingStrategy>,
) {
    strategy_resource.hybrid_strategy.reset();
}
