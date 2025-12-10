mod game_state;
mod input;
mod system;

pub use game_state::GameState;

use game_state::*;
use input::*;
use system::*;

use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_input)
            .add_systems(Update, handle_mouse_selection.run_if(in_state(GameState::SetStart)))
            .add_systems(Update, handle_mouse_selection.run_if(in_state(GameState::SetGoal)))
            .add_systems(OnEnter(GameState::Cancel), on_cancel)
            .add_systems(OnEnter(GameState::DoneRun), on_done_run)
            .add_systems(OnEnter(GameState::DoneStart), on_done_start)
            .add_systems(OnEnter(GameState::DoneGoal), on_done_goal);
    }
}
