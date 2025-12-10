use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_box(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: bevy::color::Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
