use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Quad {
    pub pos: Vec2,
    pub siz: Vec2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub root: Vec2,
    pub dir: Vec2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RayHitInfo {
    pub pt: Vec2,
    pub nor: Vec2,
    pub dist: f32,
}
