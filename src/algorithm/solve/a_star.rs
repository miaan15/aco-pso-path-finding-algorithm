use crate::algorithm::{grid::Grid, types::Ray};
use bevy::prelude::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq)]
struct Node {
    pos: Vec2,
    pre: Option<Vec2>,
    g: f32,
    h: f32,
}

impl Node {
    fn f(&self) -> f32 {
        self.g + self.h
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f()
            .partial_cmp(&self.f())
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.h.partial_cmp(&other.h).unwrap_or(Ordering::Equal))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.x.to_bits().hash(state);
        self.pos.y.to_bits().hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProbeDirection {
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
}

impl ProbeDirection {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        static DIRECTIONS: [ProbeDirection; 8] = [
            ProbeDirection::Right,
            ProbeDirection::UpRight,
            ProbeDirection::Up,
            ProbeDirection::UpLeft,
            ProbeDirection::Left,
            ProbeDirection::DownLeft,
            ProbeDirection::Down,
            ProbeDirection::DownRight,
        ];
        DIRECTIONS.iter()
    }
}

pub struct AStarStrategy {
    pub step_size: f32,
    grid: Arc<Mutex<Grid>>,
}

impl AStarStrategy {
    pub fn new(grid: Arc<Mutex<Grid>>) -> Self {
        Self {
            step_size: 20.0,
            grid,
        }
    }

    pub fn path_finding(&self, start: Option<Vec2>, goal: Option<Vec2>) -> Option<Vec<Vec2>> {
        let start = start?;
        let goal = goal?;

        let mut queue = BinaryHeap::new();
        let mut g_costs: HashMap<(u32, u32), f32> = HashMap::new();
        let mut predecessors: HashMap<(u32, u32), Vec2> = HashMap::new();

        let start_node = Node {
            pos: start,
            pre: None,
            g: 0.0,
            h: Self::heuristic(start, goal),
        };
        queue.push(start_node);

        let start_key = (start.x.to_bits(), start.y.to_bits());
        g_costs.insert(start_key, 0.0);

        let mut goal_node: Option<Node> = None;

        while let Some(cur) = queue.pop() {
            let cur_key = (cur.pos.x.to_bits(), cur.pos.y.to_bits());

            if let Some(&best_g) = g_costs.get(&cur_key) {
                if cur.g > best_g {
                    continue;
                }
            }

            if self.has_sight(cur.pos, goal) {
                let goal_key = (goal.x.to_bits(), goal.y.to_bits());
                predecessors.insert(goal_key, cur.pos);
                goal_node = Some(Node {
                    pos: goal,
                    pre: Some(cur.pos),
                    g: cur.g + goal.distance(cur.pos),
                    h: 0.0,
                });
                break;
            }

            for dir in ProbeDirection::iter() {
                if let Some(new_pos) = self.get_new_pos(cur.pos, *dir) {
                    let new_dist = new_pos.distance(cur.pos);
                    let new_g = cur.g + new_dist;
                    let new_key = (new_pos.x.to_bits(), new_pos.y.to_bits());

                    let should_process = g_costs
                        .get(&new_key)
                        .map_or(true, |&existing_g| new_g < existing_g);

                    if should_process {
                        let new_node = Node {
                            pos: new_pos,
                            pre: Some(cur.pos),
                            g: new_g,
                            h: Self::heuristic(new_pos, goal),
                        };

                        queue.push(new_node);
                        g_costs.insert(new_key, new_g);
                        predecessors.insert(new_key, cur.pos);
                    }
                }
            }
        }

        if let Some(goal) = goal_node {
            let mut path = Vec::new();
            let mut current = goal.pos;
            path.push(current);

            while current != start {
                let cur_key = (current.x.to_bits(), current.y.to_bits());
                if let Some(&prev) = predecessors.get(&cur_key) {

                    path.push(prev);
                    current = prev;
                } else {
                    return None;
                }
            }

            path.reverse();
            Some(path)
        } else {
            None
        }
    }

    fn heuristic(a: Vec2, b: Vec2) -> f32 {
        a.distance(b)
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

    fn get_new_pos(&self, root: Vec2, direction: ProbeDirection) -> Option<Vec2> {
        let offset = match direction {
            ProbeDirection::Right => Vec2::X,               // (1, 0)
            ProbeDirection::UpRight => Vec2::X + Vec2::Y,   // (1, 1)
            ProbeDirection::Up => Vec2::Y,                  // (0, 1)
            ProbeDirection::UpLeft => -Vec2::X + Vec2::Y,   // (-1, 1)
            ProbeDirection::Left => -Vec2::X,               // (-1, 0)
            ProbeDirection::DownLeft => -Vec2::X - Vec2::Y, // (-1, -1)
            ProbeDirection::Down => -Vec2::Y,               // (0, -1)
            ProbeDirection::DownRight => Vec2::X - Vec2::Y, // (1, -1)
        };

        let new_pos = root + offset * self.step_size;

        // Only return the new position if there's a clear line of sight
        if self.has_sight(root, new_pos) {
            Some(new_pos)
        } else {
            None
        }
    }
}
