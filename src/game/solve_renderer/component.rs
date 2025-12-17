use bevy::prelude::*;

#[derive(Component)]
pub struct StartPoint;

#[derive(Component)]
pub struct GoalPoint;

#[derive(Component)]
pub struct PointRenderer {
    pub width: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct PathRenderer {
    pub width: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct AStarPathRenderer {
    pub width: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct TemporaryLineRenderer {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct TemporaryLines {
    pub lines: Vec<(Vec2, Vec2)>,
}

impl PointRenderer {
    pub fn new(radius: f32, color: Color) -> Self {
        Self { width: radius, color }
    }
}

impl PathRenderer {
    pub fn new(width: f32, color: Color) -> Self {
        Self { width, color }
    }
}

impl AStarPathRenderer {
    pub fn new(width: f32, color: Color) -> Self {
        Self { width, color }
    }
}
