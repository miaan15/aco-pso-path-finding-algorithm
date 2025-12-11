use crate::algorithm::problem::Problem;
use bevy::prelude::*;

use super::aco::*;
use super::pso::*;

pub struct HybridStrategy {}

impl HybridStrategy {
    pub fn path_finding(&self, problem: &Problem) -> Option<Vec<Vec2>> { todo!() }
}
