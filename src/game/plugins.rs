use std::sync::Arc;

use bevy::prelude::*;

use crate::game::algorithm_resource::AlgorithmResource;
use crate::game::grid_renderer::GridPlugin;
use crate::game::system::*;

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_plugins(GridPlugin).insert_resource(create_algorithm_resource());
    }
}
