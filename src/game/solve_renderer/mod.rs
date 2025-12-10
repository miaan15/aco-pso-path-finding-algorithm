mod component;
mod system;

pub use system::{render_start_goal, render_path};
pub use component::{StartPoint, GoalPoint, PathRenderer};

use bevy::prelude::*;

pub struct SolvePlugin;

impl Plugin for SolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_start_goal)
            .add_systems(Update, render_path);
    }
}

