use crate::algorithm::{grid::Grid, types::Ray};
use bevy::prelude::*;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{Arc, Mutex},
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

pub struct HybridStrategy {
    pub exploitation_chance: f64,
    pub elicitation_constant: f64,
    pub evaporation_coefficient: f64,
    pub deposit_constant: f64,
    pub global_evaporation_coefficient: f64,
    pub global_deposit_constant: f64,
    pub init_pheromone: f64,
    pub ant_number: u32,
    pub max_ant_try: u32,
    
    pub particle_inertia: f64,
    pub particle_global_factor: f64,
    pub particle_local_factor: f64,

    pub init_alpha_min: f64,
    pub init_alpha_max: f64,
    pub init_beta_min: f64,
    pub init_beta_max: f64,

    grid: Arc<Mutex<Grid>>,
    cur_alpha: f64,
    cur_beta: f64,
    global_pheromones: HashMap<Line, f64>,
    global_best_path: Option<Vec<Node>>,
    global_best_len: f64,
    cache_start: Option<Vec2>,
    cache_goal: Option<Vec2>,
}

impl HybridStrategy {
    pub fn new(grid: Arc<Mutex<Grid>>) -> Self {
        Self {
            exploitation_chance: 0.5,
            elicitation_constant: 1000.0,
            evaporation_coefficient: 0.2,
            global_deposit_constant: 6000.0,
            global_evaporation_coefficient: 0.3,
            deposit_constant: 6000.0,
            init_pheromone: 1.0,
            ant_number: 80,
            max_ant_try: 2000,

            particle_inertia: 0.7,
            particle_global_factor: 2.0,
            particle_local_factor: 2.0,

            init_alpha_min: 0.5,
            init_alpha_max: 3.0,
            init_beta_min: 0.5,
            init_beta_max: 3.0,

            grid,
            cur_alpha: 1.2,
            cur_beta: 0.9,
            global_pheromones: HashMap::new(),
            global_best_path: None,
            global_best_len: f64::INFINITY,
            cache_start: None,
            cache_goal: None,
        }
    }
}

impl HybridStrategy {
    pub fn reset(&mut self) {
        self.global_pheromones.clear();
        self.global_best_path = None;
        self.global_best_len = f64::INFINITY;
    }

    pub fn path_finding(&mut self, start: Option<Vec2>, goal: Option<Vec2>) -> Option<Vec<Vec2>> {
        let start = start?;
        let goal = goal?;

        if let Some(cached) = self.cache_start {
            if cached != start {
                self.global_best_len = f64::INFINITY;
                self.cache_start = Some(start.clone());
            }
        } else {
            self.global_best_len = f64::INFINITY;
            self.cache_start = Some(start.clone());
        }
        if let Some(cached) = self.cache_goal {
            if cached != goal {
                self.global_best_len = f64::INFINITY;
                self.cache_goal = Some(goal.clone());
            }
        } else {
            self.global_best_len = f64::INFINITY;
            self.cache_goal = Some(goal.clone());
        }

        if let Some(ref path) = self.global_best_path {
            let mut path_blocked = false;
            for window in path.windows(2) {
                let from_world = self.node_to_world_pos(window[0].clone());
                let to_world = self.node_to_world_pos(window[1].clone());
                if !self.has_sight(from_world, to_world) {
                    path_blocked = true;
                    break;
                }
            }
            if path_blocked {
                self.global_best_len = f64::INFINITY;
                self.global_best_path = None;
            }
        }

        let start_node = self.world_to_node_pos(start).unwrap();
        let goal_node = self.world_to_node_pos(goal).unwrap();

        let mut pheromones: HashMap<Line, f64> = self.global_pheromones.clone();

        let mut tabu: Vec<HashSet<Node>> = Vec::new();
        for _ in 0..self.ant_number {
            let mut hs = HashSet::new();
            hs.insert(start_node.clone());
            tabu.push(hs);
        }

        let mut ants_cur_path: Vec<Vec<Node>> = Vec::new();
        ants_cur_path.resize(self.ant_number as usize, Vec::new());
        ants_cur_path
            .iter_mut()
            .for_each(|x| x.push(start_node.clone()));

        let mut ants_path_len: Vec<f64> = Vec::new();
        ants_path_len.resize(self.ant_number as usize, 0.0);

        for _ in 0..self.max_ant_try {
            for ant_idx in 0..self.ant_number {
                let cur_ant_node = ants_cur_path.get(ant_idx as usize).unwrap().last().unwrap();
                if *cur_ant_node == goal_node {
                    continue;
                }

                let cur_tabu = tabu.get_mut(ant_idx as usize).unwrap();

                let next_ant_node =
                    self.calculate_next_node(cur_ant_node.clone(), &pheromones, &cur_tabu, goal);
                let cur_line = Line::new(cur_ant_node.clone(), next_ant_node.clone());

                let cur_path_len = ants_path_len.get_mut(ant_idx as usize).unwrap();
                *cur_path_len += Vec2::distance(
                    self.node_to_world_pos(cur_ant_node.clone()),
                    self.node_to_world_pos(next_ant_node.clone()),
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

        let mut best_path = None;
        let mut best_path_len = f64::INFINITY;
        for ant_idx in 0..self.ant_number {
            let path = &ants_cur_path[ant_idx as usize];
            if path.last() == Some(&goal_node) && ants_path_len[ant_idx as usize] < best_path_len {
                best_path = Some(path);
                best_path_len = ants_path_len[ant_idx as usize];
            }
        }

        if let Some(path) = best_path {
            path.windows(2).for_each(|x| {
                if let Some(p) = self
                    .global_pheromones
                    .get_mut(&Line::new(x[0].clone(), x[1].clone()))
                {
                    *p = (1.0 - self.global_evaporation_coefficient) * *p
                        + self.global_evaporation_coefficient
                            * ((self.global_deposit_constant + 1.0) / (best_path_len + 1.0));
                } else {
                    self.global_pheromones.insert(
                        Line::new(x[0].clone(), x[1].clone()),
                        (1.0 - self.global_evaporation_coefficient) * self.init_pheromone
                            + self.global_evaporation_coefficient
                                * ((self.global_deposit_constant + 1.0) / (best_path_len + 1.0)),
                    );
                }
            });

            if best_path_len < self.global_best_len {
                self.global_best_path = Some(path.clone());
                self.global_best_len = best_path_len;
            }
        }

        self.global_best_path.clone().map(|path| {
            path.iter()
                .map(|x| self.node_to_world_pos(x.clone()))
                .collect()
        })
    }

    fn calculate_next_node(
        &self,
        node: Node,
        pheromones: &HashMap<Line, f64>,
        tabu: &HashSet<Node>,
        goal: Vec2,
    ) -> Node {
        let is_exploit = HybridStrategy::roll(vec![
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
                goal,
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
            res = next_nodes[HybridStrategy::roll(next_values)].clone();
        }

        res
    }

    fn get_path_value(
        &self,
        line: Line,
        pheromones: &HashMap<Line, f64>,
        tabu: &HashSet<Node>,
        goal: Vec2,
    ) -> f64 {
        if tabu.contains(&line.to) {
            return 0.0000001;
        }

        if !self.node_has_sight(line.from.clone(), line.to.clone()) {
            return 0.0000000001;
        }

        (*pheromones.get(&line).unwrap_or(&self.init_pheromone)).powf(self.cur_alpha)
            * self.get_heuristic(line.to.clone(), goal).powf(self.cur_beta)
    }
    fn get_heuristic(&self, node: Node, goal: Vec2) -> f64 {
        let wpos = self.node_to_world_pos(node);
        (self.elicitation_constant + 1.0) / (Vec2::distance(wpos, goal) as f64 + 1.0)
    }
}

impl HybridStrategy {
    fn world_to_node_pos(&self, wpos: Vec2) -> Option<Node> {
        if wpos.is_nan() {
            None
        } else {
            let pixel_size = self.grid.lock().unwrap().pixel_size();
            Some(Node::new(
                ((wpos.x - pixel_size / 2.0) / pixel_size).round() as i32,
                ((wpos.y - pixel_size / 2.0) / pixel_size).round() as i32,
            ))
        }
    }
    fn node_to_world_pos(&self, npos: Node) -> Vec2 {
        let pixel_size = self.grid.lock().unwrap().pixel_size();
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

    fn node_has_sight(&self, nfrom: Node, nto: Node) -> bool {
        self.has_sight(self.node_to_world_pos(nfrom), self.node_to_world_pos(nto))
    }

    fn has_sight(&self, from: Vec2, to: Vec2) -> bool {
        let direction = (to - from).normalize_or_zero();
        let distance = from.distance(to);
        if distance == 0.0 {
            return true;
        }

        let ray = Ray {
            root: from,
            dir: direction,
        };

        self.grid
            .lock()
            .unwrap()
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
