use crate::game::control::{ControlPlugin, GameState};
use crate::game::grid_renderer::GridPlugin;
use crate::game::solve_renderer::SolvePlugin;
use crate::game::system::*;
use bevy::prelude::*;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(create_algorithm_resource())
            .add_systems(Startup, setup_game)
            .add_systems(OnEnter(GameState::Solving), run_pathfinding)
            .add_plugins(GridPlugin)
            .add_plugins(ControlPlugin)
            .add_plugins(SolvePlugin)
            .insert_state(GameState::Idle);
    }
}
