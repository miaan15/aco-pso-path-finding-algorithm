mod component;
mod system;

pub use system::{render_start_goal, render_path, clear_path};
pub use component::{StartPoint, GoalPoint, PointRenderer, PathRenderer};
use crate::game::system::run_pathfinding;

use bevy::prelude::*;

pub struct SolvePlugin;

impl Plugin for SolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_start_goal)
            .add_systems(Update, render_path);
    }
}

