mod component;
mod system;

pub use system::render_start_goal;
pub use component::*;

use bevy::prelude::*;

pub struct SolvePlugin;

impl Plugin for SolvePlugin {
    fn build(&self, app: &mut App) {
    }
}

