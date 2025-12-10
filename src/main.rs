mod game;
mod algorithm;

use bevy::prelude::*;
use game::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugins(GameScenePlugin).run();
}
