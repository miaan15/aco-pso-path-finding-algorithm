use super::component::{GoalPoint, PathRenderer, PointRenderer, StartPoint};
use crate::game::algorithm_resource::AlgorithmResource;
use bevy::prelude::*;

pub fn render_start_goal(
    mut commands: Commands,
    algorithm_resource: Res<AlgorithmResource>,
    start_point_query: Query<Entity, With<StartPoint>>,
    goal_point_query: Query<Entity, With<GoalPoint>>,
) {
    for entity in start_point_query.iter().chain(goal_point_query.iter()) {
        commands.entity(entity).despawn();
    }

    if let Some(start_pos) = algorithm_resource.problem.start {
        commands
            .spawn((
                StartPoint,
                PointRenderer::new(3.0, Color::srgb(0.0, 1.0, 0.0)),
                Transform::from_xyz(start_pos.x, start_pos.y, 1.0),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn(Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                });
            });
    }

    if let Some(goal_pos) = algorithm_resource.problem.goal {
        commands
            .spawn((
                GoalPoint,
                PointRenderer::new(3.0, Color::srgb(1.0, 0.0, 0.0)),
                Transform::from_xyz(goal_pos.x, goal_pos.y, 1.0),
                Visibility::default(),
            ))
            .with_children(|parent| {
                parent.spawn(Sprite {
                    color: Color::srgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                });
            });
    }
}

pub fn render_path(
    mut commands: Commands,
    algorithm_resource: Res<AlgorithmResource>,
    path_query: Query<Entity, With<PathRenderer>>,
) {
    // Clear existing path entities
    for entity in path_query.iter() {
        commands.entity(entity).despawn();
    }

    // Render new path if available
    if let Some(path) = &algorithm_resource.path {
        if path.len() < 2 {
            return;
        }

        for i in 0..path.len() - 1 {
            let start = path[i];
            let end = path[i + 1];

            // Calculate line properties
            let direction = end - start;
            let length = direction.length();
            let angle = f32::atan2(direction.y, direction.x);

            commands.spawn((
                PathRenderer::new(2.0, Color::srgb(0.2, 0.6, 1.0)),
                Transform {
                    translation: ((start + end) / 2.0).extend(0.5),
                    rotation: Quat::from_rotation_z(angle),
                    scale: Vec3::new(length, 1.0, 1.0),
                },
                Visibility::default(),
            )).with_children(|parent| {
                parent.spawn(Sprite {
                    color: Color::srgb(0.2, 0.6, 1.0),
                    custom_size: Some(Vec2::new(1.0, 3.0)),
                    ..default()
                });
            });
        }
    }
}

pub fn clear_path(
    mut algorithm_resource: ResMut<AlgorithmResource>,
    path_query: Query<Entity, With<PathRenderer>>,
    mut commands: Commands,
) {
    // Clear path from algorithm resource
    algorithm_resource.path = None;

    // Remove path entities
    for entity in path_query.iter() {
        commands.entity(entity).despawn();
    }
}

