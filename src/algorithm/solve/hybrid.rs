use crate::algorithm::{
    problem::Problem,
    solve::aco::{self, AcoStrategy},
};
use bevy::prelude::*;

pub struct HybridStrategy {}

impl HybridStrategy {
    pub fn path_finding(&self, problem: &Problem) -> Option<Vec<Vec2>> {
        let aco = AcoStrategy {
            exploitation_chance: 0.5,
            alpha: 1.0,
            beta: 1.0,
            elicitation_constant: 1000.0,
            evaporation_coefficient: 0.4,
            deposit_constant: 6000.0,
            init_pheromone: 1.0,
            number_per_ant_group: 50,
            max_ant_group_try: 100,
        };

        aco.path_finding(problem)
    }
}
