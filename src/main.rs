mod game;
mod algorithm;

use bevy::prelude::*;
use game::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(30.0))
        .add_plugins(GameScenePlugin)
        .run();
}
