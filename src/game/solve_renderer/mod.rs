mod component;
mod system;

pub use system::{render_start_goal, render_path, render_temporary_lines, temporary_line_render, temp_debug_line, clear_path};
pub use component::{StartPoint, GoalPoint, PathRenderer, AStarPathRenderer, TemporaryLineRenderer, TemporaryLines};

use bevy::prelude::*;

pub struct SolvePlugin;

impl Plugin for SolvePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TemporaryLines>()
            .add_systems(Update, render_start_goal)
            .add_systems(Update, render_path)
            .add_systems(Update, render_temporary_lines);
    }
}

