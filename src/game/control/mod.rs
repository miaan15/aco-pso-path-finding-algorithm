mod game_state;
mod input;
mod system;

pub use game_state::GameState;

use input::*;
use system::*;

use bevy::prelude::*;
use crate::game::click_position::ClickPosition;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClickPosition>()
            .add_systems(Update, handle_keyboard_input)
            .add_systems(Update, handle_mouse_selection.run_if(in_state(GameState::SetStart)))
            .add_systems(Update, handle_mouse_selection.run_if(in_state(GameState::SetGoal)))
            .add_systems(Update, handle_mouse_selection.run_if(in_state(GameState::PlaceMode)))
            .add_systems(Update, handle_mouse_selection.run_if(in_state(GameState::DeleteMode)))
            .add_systems(OnEnter(GameState::Cancel), on_cancel)
            .add_systems(OnEnter(GameState::DoneStart), on_done_start)
            .add_systems(OnEnter(GameState::DoneGoal), on_done_goal)
            .add_systems(OnEnter(GameState::DonePlace), on_done_place)
            .add_systems(OnEnter(GameState::DoneDelete), on_done_delete);
    }
}
