use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use crate::algorithm::{problem::Problem, solve::hybrid::HybridStrategy};
use crate::game::control::GameState;

#[derive(Resource)]
pub struct PathfindingTask {
    pub handle: Option<JoinHandle<()>>,
    pub result: Arc<Mutex<Option<Option<Vec<Vec2>>>>>,
    pub is_complete: Arc<Mutex<bool>>,
}

impl Default for PathfindingTask {
    fn default() -> Self {
        Self {
            handle: None,
            result: Arc::new(Mutex::new(None)),
            is_complete: Arc::new(Mutex::new(false)),
        }
    }
}

impl Clone for PathfindingTask {
    fn clone(&self) -> Self {
        Self {
            handle: None,
            result: self.result.clone(),
            is_complete: self.is_complete.clone(),
        }
    }
}

pub fn start_pathfinding_thread(problem: Problem) -> PathfindingTask {
    let result = Arc::new(Mutex::new(None));
    let is_complete = Arc::new(Mutex::new(false));
    let result_clone = result.clone();
    let is_complete_clone = is_complete.clone();

    let handle = thread::spawn(move || {
        let mut strategy = HybridStrategy::new(problem.grid.clone());
        let path = strategy.path_finding(problem.start, problem.goal);

        if let Ok(mut result_guard) = result_clone.lock() {
            *result_guard = Some(path);
        }

        if let Ok(mut complete_guard) = is_complete_clone.lock() {
            *complete_guard = true;
        }
    });

    PathfindingTask {
        handle: Some(handle),
        result,
        is_complete,
    }
}

pub fn check_pathfinding_completion(
    mut commands: Commands,
    mut algorithm_resource: ResMut<crate::game::algorithm_resource::AlgorithmResource>,
    task: ResMut<PathfindingTask>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(complete_guard) = task.is_complete.lock() {
        if *complete_guard {
            if let Ok(result_guard) = task.result.lock() {
                if let Some(result) = result_guard.as_ref() {
                    let complete_path = if let (Some(start), Some(goal)) = (
                        algorithm_resource.problem.start,
                        algorithm_resource.problem.goal,
                    ) {
                        match result {
                            Some(path) => {
                                let mut path_vec = path.clone();
                                path_vec.insert(0, start);
                                path_vec.push(goal);
                                Some(path_vec)
                            }
                            None => Some(vec![start, goal]),
                        }
                    } else {
                        None
                    };

                    algorithm_resource.path = complete_path;
                    next_state.set(GameState::DoneRun);

                    println!("Pathfinding completed in background thread");

                    commands.remove_resource::<PathfindingTask>();
                }
            }
        }
    }
}
