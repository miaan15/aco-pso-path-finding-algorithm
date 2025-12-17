use bevy::prelude::*;

#[derive(Resource)]
pub struct ClickPosition {
    pub grid_x: Option<usize>,
    pub grid_y: Option<usize>,
}

impl Default for ClickPosition {
    fn default() -> Self {
        Self {
            grid_x: None,
            grid_y: None,
        }
    }
}