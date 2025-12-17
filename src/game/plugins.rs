use crate::game::control::{ControlPlugin, GameState};
use crate::game::grid_renderer::GridPlugin;
use crate::game::solve_renderer::SolvePlugin;
use crate::game::system::*;
use crate::game::pathfinding_system::{PathfindingStrategy, update_pathfinding};
use crate::game::timer::AlgorithmTimers;
use crate::game::debug_system::log_timing_info;
use bevy::prelude::*;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        let algorithm_resource = create_algorithm_resource();
        let pathfinding_strategy = PathfindingStrategy::new(algorithm_resource.grid.clone());

        app.insert_resource(algorithm_resource)
            .insert_resource(pathfinding_strategy)
            .insert_resource(AlgorithmTimers::default())
            .add_systems(Startup, setup_game)
            .add_systems(Update, (update_pathfinding, log_timing_info).chain())
            .add_plugins(GridPlugin)
            .add_plugins(ControlPlugin)
            .add_plugins(SolvePlugin)
            .insert_state(GameState::Idle);
    }
}
