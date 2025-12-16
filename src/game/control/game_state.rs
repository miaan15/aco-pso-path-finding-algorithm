use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Idle,
    SetStart,
    SetGoal,
    Cancel,
    DoneStart,
    DoneGoal,
}
