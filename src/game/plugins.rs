use crate::game::control::{ControlPlugin, GameState};
use crate::game::grid_renderer::GridPlugin;
use crate::game::solve_renderer::SolvePlugin;
use crate::game::system::*;
use crate::game::async_pathfinding::{PathfindingTask, check_pathfinding_completion};
use bevy::prelude::*;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_algorithm_resource())
            .add_systems(Startup, setup_game)
            .add_systems(OnEnter(GameState::Solving), run_pathfinding)
            .add_systems(Update, check_pathfinding_completion.run_if(in_state(GameState::SolvingAsync)))
            .add_systems(OnEnter(GameState::Cancel), cancel_pathfinding)
            .add_plugins(GridPlugin)
            .add_plugins(ControlPlugin)
            .add_plugins(SolvePlugin)
            .insert_state(GameState::Idle);
    }
}
