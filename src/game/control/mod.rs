mod game_state;
mod input;

pub use game_state::*;
pub use input::*;

use bevy::prelude::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_input)
            .add_systems(OnEnter(GameState::Done), on_done);
    }
}
