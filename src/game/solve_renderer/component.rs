use bevy::prelude::*;

#[derive(Component)]
pub struct StartPoint;

#[derive(Component)]
pub struct GoalPoint;

#[derive(Component)]
pub struct PointRenderer {
    pub radius: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct PathRenderer {
    pub width: f32,
    pub color: Color,
}

impl PointRenderer {
    pub fn new(radius: f32, color: Color) -> Self {
        Self { radius, color }
    }
}

impl PathRenderer {
    pub fn new(width: f32, color: Color) -> Self {
        Self { width, color }
    }
}
