use super::component::{GoalPoint, PathRenderer, AStarPathRenderer, PointRenderer, StartPoint, TemporaryLineRenderer, TemporaryLines};
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
    astar_path_query: Query<Entity, With<AStarPathRenderer>>,
) {
    for entity in path_query.iter().chain(astar_path_query.iter()) {
        commands.entity(entity).despawn();
    }

    if let Some(path) = &algorithm_resource.astar_path {
        if path.len() >= 2 {
            for i in 0..path.len() - 1 {
                let start = path[i];
                let end = path[i + 1];

                let direction = end - start;
                let length = direction.length();
                let angle = f32::atan2(direction.y, direction.x);

                commands.spawn((
                    AStarPathRenderer::new(2.0, Color::srgb(0.5, 0.5, 0.5)),
                    Transform {
                        translation: ((start + end) / 2.0).extend(0.4),
                        rotation: Quat::from_rotation_z(angle),
                        scale: Vec3::new(length, 1.0, 1.0),
                    },
                    Visibility::default(),
                )).with_children(|parent| {
                    parent.spawn(Sprite {
                        color: Color::srgb(0.5, 0.5, 0.5),
                        custom_size: Some(Vec2::new(1.0, 2.5)),
                        ..default()
                    });
                });
            }
        }
    }

    if let Some(path) = &algorithm_resource.path {
        if path.len() >= 2 {
            for i in 0..path.len() - 1 {
                let start = path[i];
                let end = path[i + 1];

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
}

pub fn render_temporary_lines(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TemporaryLineRenderer)>,
    time: Res<Time>,
    mut temp_lines: ResMut<TemporaryLines>,
) {
    // Spawn new temporary lines from the resource
    for (start, end) in temp_lines.lines.drain(..) {
        temporary_line_render(&mut commands, start, end);
    }

    // Update timers and remove expired lines
    for (entity, mut temp_line) in query.iter_mut() {
        temp_line.timer.tick(time.delta());
        if temp_line.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn temporary_line_render(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
) {
    // Calculate line properties
    let direction = end - start;
    let length = direction.length();
    let angle = f32::atan2(direction.y, direction.x);

    commands.spawn((
        TemporaryLineRenderer {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        },
        Transform {
            translation: (start + end).extend(0.5) / 2.0,
            rotation: Quat::from_rotation_z(angle),
            scale: Vec3::new(length, 1.0, 1.0),
        },
        Visibility::default(),
    )).with_children(|parent| {
        parent.spawn(Sprite {
            color: Color::srgb(0.7, 0.7, 0.7), // Light grey
            custom_size: Some(Vec2::new(1.0, 2.0)),
            ..default()
        });
    });
}

/// Convenience function to add a temporary debug line from anywhere.
/// This adds the line to the TemporaryLines resource, which will be rendered in the next frame.
/// Usage: `temp_debug_line(world, Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0));`
pub fn temp_debug_line(world: &mut World, start: Vec2, end: Vec2) {
    if let Some(mut temp_lines) = world.get_resource_mut::<TemporaryLines>() {
        temp_lines.lines.push((start, end));
    }
}

pub fn clear_path(
    mut algorithm_resource: ResMut<AlgorithmResource>,
    path_query: Query<Entity, With<PathRenderer>>,
    astar_path_query: Query<Entity, With<AStarPathRenderer>>,
    mut commands: Commands,
) {
    // Clear paths from algorithm resource
    algorithm_resource.path = None;
    algorithm_resource.astar_path = None;

    // Remove path entities
    for entity in path_query.iter().chain(astar_path_query.iter()) {
        commands.entity(entity).despawn();
    }
}

