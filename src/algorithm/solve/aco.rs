use crate::algorithm::{problem::Problem, types::Ray};
use bevy::prelude::*;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Eq)]
pub struct Node {
    pos: (i32, i32),
}
impl Node {
    fn new(x: i32, y: i32) -> Self {
        Node { pos: (x, y) }
    }
}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

#[derive(Debug, Clone, Eq)]
struct Line {
    from: Node,
    to: Node,
}
impl Line {
    fn new(from: Node, to: Node) -> Self {
        Line { from, to }
    }
}
impl Hash for Line {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}
impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

pub struct AcoStrategy {
    pub exploitation_chance: f64,
    pub alpha: f64,
    pub beta: f64,

    pub elicitation_constant: f64,

    pub evaporation_coefficient: f64,
    pub deposit_constant: f64,
    pub global_evaporation_coefficient: f64,
    pub global_deposit_constant: f64,

    pub init_pheromone: f64,

    pub number_per_ant_group: u32,
    pub max_ant_group_try: u32,
    pub number_ant_group: u32,
}

impl AcoStrategy {
    pub fn path_finding(&self, problem: &Problem) -> Option<Vec<Vec2>> {
        let start = problem.start.unwrap();
        let goal = problem.goal.unwrap();
        let start_node = self.world_to_node_pos(start, problem).unwrap();
        let goal_node = self.world_to_node_pos(goal, problem).unwrap();

        let mut global_pheromones: HashMap<Line, f64> = HashMap::new();

        for group_idx in 0..self.number_ant_group {
            let mut pheromones: HashMap<Line, f64> = global_pheromones.clone();

            let mut tabu: Vec<HashSet<Node>> = Vec::new();
            for _ in 0..self.number_per_ant_group {
                let mut hs = HashSet::new();
                hs.insert(start_node.clone());
                tabu.push(hs);
            }

            let mut ants_cur_path: Vec<Vec<Node>> = Vec::new();
            ants_cur_path.resize(self.number_per_ant_group as usize, Vec::new());
            ants_cur_path
                .iter_mut()
                .for_each(|x| x.push(start_node.clone()));

            let mut ants_path_len: Vec<f64> = Vec::new();
            ants_path_len.resize(self.number_per_ant_group as usize, 0.0);

            for _ in 0..self.max_ant_group_try {
                for ant_idx in 0..self.number_per_ant_group {
                    let cur_ant_node = ants_cur_path.get(ant_idx as usize).unwrap().last().unwrap();
                    if *cur_ant_node == goal_node {
                        continue;
                    }

                    let cur_tabu = tabu.get_mut(ant_idx as usize).unwrap();

                    let next_ant_node = self.calculate_next_node(
                        cur_ant_node.clone(),
                        &pheromones,
                        &cur_tabu,
                        problem,
                    );
                    let cur_line = Line::new(cur_ant_node.clone(), next_ant_node.clone());

                    let cur_path_len = ants_path_len.get_mut(ant_idx as usize).unwrap();
                    *cur_path_len += Vec2::distance(
                        self.node_to_world_pos(cur_ant_node.clone(), problem),
                        self.node_to_world_pos(next_ant_node.clone(), problem),
                    ) as f64;

                    ants_cur_path
                        .get_mut(ant_idx as usize)
                        .unwrap()
                        .push(next_ant_node.clone());
                    cur_tabu.insert(next_ant_node.clone());

                    if let Some(line_pheromone) = pheromones.get_mut(&cur_line) {
                        *line_pheromone = (1.0 - self.evaporation_coefficient) * *line_pheromone
                            + self.evaporation_coefficient
                                * ((self.deposit_constant + 1.0) / (*cur_path_len + 1.0));
                    } else {
                        pheromones.insert(
                            cur_line.clone(),
                            (1.0 - self.evaporation_coefficient) * self.init_pheromone
                                + self.evaporation_coefficient
                                    * ((self.deposit_constant + 1.0) / (*cur_path_len + 1.0)),
                        );
                    }
                }
            }

            let mut best_path = ants_cur_path.first().unwrap();
            let mut best_path_len = *ants_path_len.first().unwrap();
            for ant_idx in 1..self.number_per_ant_group {
                if ants_path_len[ant_idx as usize] > best_path_len {
                    best_path = ants_cur_path.get(ant_idx as usize).unwrap().as_ref();
                    best_path_len = ants_path_len[ant_idx as usize];
                }
            }

            best_path.windows(2).for_each(|x| {
                if let Some(p) = global_pheromones.get_mut(&Line::new(x[0].clone(), x[1].clone())) {
                    *p = (1.0 - self.global_evaporation_coefficient) * *p
                        + self.global_evaporation_coefficient
                            * ((self.global_deposit_constant + 1.0) / (best_path_len + 1.0));
                } else {
                    global_pheromones.insert(
                        Line::new(x[0].clone(), x[1].clone()),
                        (1.0 - self.global_evaporation_coefficient) * self.init_pheromone
                            + self.global_evaporation_coefficient
                                * ((self.global_deposit_constant + 1.0) / (best_path_len + 1.0)),
                    );
                }
            });

            if group_idx == self.number_ant_group - 1 {
                return Some(
                    best_path
                        .iter()
                        .map(|x| self.node_to_world_pos(x.clone(), problem))
                        .collect(),
                );
            }
        }

        None
    }

    fn calculate_next_node(
        &self,
        node: Node,
        pheromones: &HashMap<Line, f64>,
        tabu: &HashSet<Node>,
        problem: &Problem,
    ) -> Node {
        let is_exploit = AcoStrategy::roll(vec![
            self.exploitation_chance,
            1.0 - self.exploitation_chance,
        ]);

        let mut next_values: Vec<f64> = Vec::new();
        let next_nodes = self.next_node_list(node.clone());
        for nxnode in next_nodes.iter() {
            next_values.push(self.get_path_value(
                Line::new(node.clone(), nxnode.clone()),
                pheromones,
                tabu,
                problem,
            ));
        }

        let mut res: Node = next_nodes[0].clone();
        if is_exploit == 0 {
            let mut max = -1.0;
            for i in 0..next_nodes.len() {
                if next_values[i] > max {
                    max = next_values[i];
                    res = next_nodes[i].clone();
                }
            }
        } else {
            res = next_nodes[AcoStrategy::roll(next_values)].clone();
        }

        res
    }

    fn get_path_value(
        &self,
        line: Line,
        pheromones: &HashMap<Line, f64>,
        tabu: &HashSet<Node>,
        problem: &Problem,
    ) -> f64 {
        if tabu.contains(&line.to) {
            return 0.0000001;
        }

        if !self.node_has_sight(line.from.clone(), line.to.clone(), problem) {
            return 0.0000000001;
        }

        (*pheromones.get(&line).unwrap_or(&self.init_pheromone)).powf(self.alpha)
            * self.get_heuristic(line.to.clone(), problem).powf(self.beta)
    }
    fn get_heuristic(&self, node: Node, problem: &Problem) -> f64 {
        let wpos = self.node_to_world_pos(node, problem);
        let goal = problem.goal.unwrap();
        (self.elicitation_constant + 1.0) / (Vec2::distance(wpos, goal) as f64 + 1.0)
    }
}

impl AcoStrategy {
    fn world_to_node_pos(&self, wpos: Vec2, problem: &Problem) -> Option<Node> {
        if wpos.is_nan() {
            None
        } else {
            let pixel_size = problem.grid.pixel_size();
            Some(Node::new(
                ((wpos.x - pixel_size / 2.0) / pixel_size).round() as i32,
                ((wpos.y - pixel_size / 2.0) / pixel_size).round() as i32,
            ))
        }
    }
    fn node_to_world_pos(&self, npos: Node, problem: &Problem) -> Vec2 {
        let pixel_size = problem.grid.pixel_size();
        Vec2::new(
            npos.pos.0 as f32 * pixel_size + pixel_size / 2.0,
            npos.pos.1 as f32 * pixel_size + pixel_size / 2.0,
        )
    }

    fn next_node_list(&self, npos: Node) -> Vec<Node> {
        let mut v = Vec::new();
        v.push(Node::new(npos.pos.0 + 1, npos.pos.1 + 0));
        v.push(Node::new(npos.pos.0 + 1, npos.pos.1 - 1));
        v.push(Node::new(npos.pos.0 + 0, npos.pos.1 - 1));
        v.push(Node::new(npos.pos.0 - 1, npos.pos.1 - 1));
        v.push(Node::new(npos.pos.0 - 1, npos.pos.1 + 0));
        v.push(Node::new(npos.pos.0 - 1, npos.pos.1 + 1));
        v.push(Node::new(npos.pos.0 + 0, npos.pos.1 + 1));
        v.push(Node::new(npos.pos.0 + 1, npos.pos.1 + 1));
        v
    }

    fn node_has_sight(&self, nfrom: Node, nto: Node, problem: &Problem) -> bool {
        self.has_sight(
            self.node_to_world_pos(nfrom, problem),
            self.node_to_world_pos(nto, problem),
            problem,
        )
    }

    fn has_sight(&self, from: Vec2, to: Vec2, problem: &Problem) -> bool {
        let direction = (to - from).normalize_or_zero();
        let distance = from.distance(to);
        if distance == 0.0 {
            return true;
        }

        let ray = Ray {
            root: from,
            dir: direction,
        };

        problem
            .grid
            .raycast(ray)
            .map_or(true, |hit| hit.dist >= distance)
    }

    fn roll(weights: Vec<f64>) -> usize {
        let total: f64 = weights.iter().sum();

        let mut rng = rand::rng();
        let mut random = rng.random_range(0.0..total);

        for (i, &weight) in weights.iter().enumerate() {
            random -= weight;
            if random < 0.0 {
                return i;
            }
        }

        weights.len() - 1
    }
}
