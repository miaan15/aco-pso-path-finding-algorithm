mod game_state;
mod input;
mod system;

pub use game_state::*;
pub use input::*;

use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_input)
            .add_systems(Update, handle_mouse_selection.run_if(in_state(GameState::SetStart)))
            .add_systems(Update, handle_mouse_selection.run_if(in_state(GameState::SetGoal)))
            .add_systems(OnEnter(GameState::Done), on_done);
    }
}
