mod component;
mod system;

use bevy::prelude::*;

use system::{setup_grid_renderer, render_grid};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid_renderer)
            .add_systems(Update, render_grid);
    }
}
