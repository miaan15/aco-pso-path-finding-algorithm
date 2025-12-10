mod component;
mod system;

use bevy::prelude::*;
use system::render_start_goal;

pub struct SolvePlugin;

impl Plugin for SolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_start_goal);
    }
}

