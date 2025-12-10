use bevy::prelude::*;

use crate::algorithm::types::Quad;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridCell {
    Air,
    Wall,
}

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    cell_size: f32,
    root: Vec2,
    data: Vec<GridCell>,
}

impl Grid {
    pub fn new(width: usize, height: usize, cell_size: f32, root: Vec2) -> Self {
        Self {
            width: width,
            height: height,
            cell_size: cell_size,
            root: root,
            data: vec![GridCell::Air; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set(&mut self, x: usize, y: usize, val: GridCell) -> Option<()> {
        let pos = self.get_grid_pos(x, y)?;
        self.data[pos] = val;
        Some(())
    }
    pub fn get(&self, x: usize, y: usize) -> Option<GridCell> {
        let pos = self.get_grid_pos(x, y)?;
        Some(self.data[pos].clone())
    }

    pub fn world_width(&self) -> f32 {
        self.width as f32 * self.cell_size
    }
    pub fn world_height(&self) -> f32 {
        self.height as f32 * self.cell_size
    }

    pub fn bound_quad(&self) -> Quad {
        Quad {
            pos: self.root,
            siz: Vec2::new(self.world_width(), self.world_height()),
        }
    }

    pub fn get_cell_quad(&self, x: usize, y: usize) -> Option<Quad> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(Quad {
            pos: Vec2::new(
                self.root.x + x as f32 * self.cell_size,
                self.root.y + y as f32 * self.cell_size,
            ),
            siz: Vec2::splat(self.cell_size),
        })
    }

    pub fn get_from_world_pos(&self, pos: Vec2) -> Option<(usize, usize)> {
        let local_x = pos.x - self.root.x;
        let local_y = pos.y - self.root.y;

        if local_x < 0.0 || local_y < 0.0 {
            return None;
        }

        let grid_x = (local_x / self.cell_size) as usize;
        let grid_y = (local_y / self.cell_size) as usize;

        if grid_x >= self.width || grid_y >= self.height {
            return None;
        }

        Some((grid_x, grid_y))
    }

    pub fn is_air(&self, x: usize, y: usize) -> Option<bool> {
        let pos = self.get_grid_pos(x, y)?;
        Some(self.data[pos] == GridCell::Air)
    }
    pub fn is_wall(&self, x: usize, y: usize) -> Option<bool> {
        let pos = self.get_grid_pos(x, y)?;
        Some(self.data[pos] == GridCell::Wall)
    }

    fn get_grid_pos(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(y * self.width + x)
    }
}
