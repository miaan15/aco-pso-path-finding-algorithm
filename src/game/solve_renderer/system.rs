use bevy::prelude::*;
use crate::game::algorithm_resource::AlgorithmResource;
use super::component::{StartPoint, GoalPoint, PointRenderer};

pub fn render_start_goal(
    mut commands: Commands,
    algorithm_resource: Res<AlgorithmResource>,
    start_point_query: Query<Entity, With<StartPoint>>,
    goal_point_query: Query<Entity, With<GoalPoint>>,
) {
    // Clear existing start/goal renderers
    for entity in start_point_query.iter().chain(goal_point_query.iter()) {
        commands.entity(entity).despawn();
    }

    // Render start point (green)
    if let Some(start_pos) = algorithm_resource.problem.start {
        commands.spawn((
            StartPoint,
            PointRenderer::new(10.0, Color::srgb(0.0, 1.0, 0.0)),
            Transform::from_xyz(start_pos.x, start_pos.y, 1.0),
            Visibility::default(),
        )).with_children(|parent| {
            parent.spawn(Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            });
        });
    }

    // Render goal point (red)
    if let Some(goal_pos) = algorithm_resource.problem.goal {
        commands.spawn((
            GoalPoint,
            PointRenderer::new(10.0, Color::srgb(1.0, 0.0, 0.0)),
            Transform::from_xyz(goal_pos.x, goal_pos.y, 1.0),
            Visibility::default(),
        )).with_children(|parent| {
            parent.spawn(Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            });
        });
    }
}