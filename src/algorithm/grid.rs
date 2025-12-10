use bevy::prelude::*;

use crate::algorithm::types::{Quad, Ray, RayHitInfo};

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

    pub fn raycast(&self, ray: Ray) -> Option<RayHitInfo> {
        let dir_norm = ray.dir.normalize_or_zero();
        if dir_norm == Vec2::new(0.0, 0.0) {
            return None;
        }

        let bound_min = self.root;
        let bound_max = self.root
            + Vec2::new(
                self.width as f32 * self.cell_size,
                self.height as f32 * self.cell_size,
            );

        let t_entry = self.box_intersection(ray.root, dir_norm, bound_min, bound_max)?;

        let start_point = if t_entry > 0.0 {
            ray.root + dir_norm * t_entry
        } else {
            ray.root
        };

        let cell_size = self.cell_size;

        let local_x = start_point.x - self.root.x;
        let local_y = start_point.y - self.root.y;

        let mut grid_x = (local_x / cell_size).floor() as isize;
        let mut grid_y = (local_y / cell_size).floor() as isize;

        grid_x = grid_x.clamp(0, self.width as isize - 1);
        grid_y = grid_y.clamp(0, self.height as isize - 1);

        let step_x = if dir_norm.x > 0.0 {
            1
        } else if dir_norm.x < 0.0 {
            -1
        } else {
            0
        };
        let step_y = if dir_norm.y > 0.0 {
            1
        } else if dir_norm.y < 0.0 {
            -1
        } else {
            0
        };

        let t_delta_x = if dir_norm.x != 0.0 {
            cell_size / dir_norm.x.abs()
        } else {
            f32::MAX
        };

        let t_delta_y = if dir_norm.y != 0.0 {
            cell_size / dir_norm.y.abs()
        } else {
            f32::MAX
        };

        let grid_world_x = self.root.x + grid_x as f32 * cell_size;
        let grid_world_y = self.root.y + grid_y as f32 * cell_size;

        let mut t_max_x = if dir_norm.x > 0.0 {
            (grid_world_x + cell_size - ray.root.x) / dir_norm.x
        } else if dir_norm.x < 0.0 {
            (grid_world_x - ray.root.x) / dir_norm.x
        } else {
            f32::MAX
        };

        let mut t_max_y = if dir_norm.y > 0.0 {
            (grid_world_y + cell_size - ray.root.y) / dir_norm.y
        } else if dir_norm.y < 0.0 {
            (grid_world_y - ray.root.y) / dir_norm.y
        } else {
            f32::MAX
        };

        let mut t_current = t_entry.max(0.0);

        while grid_x >= 0
            && grid_x < self.width as isize
            && grid_y >= 0
            && grid_y < self.height as isize
        {
            if let Some(GridCell::Wall) = self.get(grid_x as usize, grid_y as usize) {
                let hit_pos = ray.root + dir_norm * t_current;

                let cell_world_x = self.root.x + grid_x as f32 * cell_size;
                let cell_world_y = self.root.y + grid_y as f32 * cell_size;

                let rel_pos = hit_pos - Vec2::new(cell_world_x, cell_world_y);

                let normal = if rel_pos.x < cell_size * 0.1 {
                    Vec2::new(-1.0, 0.0)
                } else if rel_pos.x > cell_size * 0.9 {
                    Vec2::new(1.0, 0.0)
                } else if rel_pos.y < cell_size * 0.1 {
                    Vec2::new(0.0, -1.0)
                } else if rel_pos.y > cell_size * 0.9 {
                    Vec2::new(0.0, 1.0)
                } else {
                    let cell_center = Vec2::new(
                        cell_world_x + cell_size * 0.5,
                        cell_world_y + cell_size * 0.5,
                    );
                    (cell_center - hit_pos).normalize_or_zero()
                };

                return Some(RayHitInfo {
                    pt: hit_pos,
                    nor: normal,
                    dist: t_current,
                });
            }

            if t_max_x < t_max_y {
                t_current = t_max_x;
                t_max_x += t_delta_x;
                grid_x += step_x;
            } else {
                t_current = t_max_y;
                t_max_y += t_delta_y;
                grid_y += step_y;
            }
        }

        None
    }
    fn box_intersection(
        &self,
        ray_root: Vec2,
        ray_dir: Vec2,
        box_min: Vec2,
        box_max: Vec2,
    ) -> Option<f32> {
        let mut t1 = (box_min.x - ray_root.x) / ray_dir.x;
        let mut t2 = (box_max.x - ray_root.x) / ray_dir.x;

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }

        let mut t3 = (box_min.y - ray_root.y) / ray_dir.y;
        let mut t4 = (box_max.y - ray_root.y) / ray_dir.y;

        if t3 > t4 {
            std::mem::swap(&mut t3, &mut t4);
        }

        let t_min = t1.max(t3).max(0.0);
        let t_max = t2.min(t4);

        if t_min <= t_max { Some(t_min) } else { None }
    }

    fn get_grid_pos(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(y * self.width + x)
    }
}
