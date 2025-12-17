use crate::algorithm::solve::{hybrid::HybridStrategy, a_star::AStarStrategy};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
    mut timers: ResMut<crate::game::timer::AlgorithmTimers>,
) {
    let start = algorithm_resource.problem.start;
    let goal = algorithm_resource.problem.goal;

    if let (Some(start), Some(goal)) = (start, goal) {
        let hybrid_start_time = Instant::now();
        let hybrid_path = strategy_resource
            .hybrid_strategy
            .path_finding(Some(start), Some(goal));
        let hybrid_duration = hybrid_start_time.elapsed();

        let astar_start_time = Instant::now();
        let astar_path = strategy_resource
            .astar_strategy
            .path_finding(Some(start), Some(goal));
        let astar_duration = astar_start_time.elapsed();

        timers.hybrid_last_ms = hybrid_duration.as_secs_f64() * 1000.0;
        timers.hybrid_total_ms += timers.hybrid_last_ms;
        if timers.hybrid_last_ms > timers.hybrid_max_ms {
            timers.hybrid_max_ms = timers.hybrid_last_ms;
        }

        timers.a_star_last_ms = astar_duration.as_secs_f64() * 1000.0;
        timers.a_star_total_ms += timers.a_star_last_ms;
        if timers.a_star_last_ms > timers.a_star_max_ms {
            timers.a_star_max_ms = timers.a_star_last_ms;
        }

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
    mut timers: ResMut<crate::game::timer::AlgorithmTimers>,
) {
    strategy_resource.hybrid_strategy.reset();
    timers.reset_totals();
}
