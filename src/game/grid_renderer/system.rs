use crate::algorithm::grid::Grid;
use crate::algorithm::grid::GridCell;
use crate::game::algorithm_resource::AlgorithmResource;
use crate::game::grid_renderer::component::GridRenderer;
use bevy::prelude::*;
use std::sync::Arc;

pub fn setup_grid_renderer(mut commands: Commands, algorithm_resource: Res<AlgorithmResource>) {
    let grid = algorithm_resource.grid.clone();

    commands.spawn((
        GridRenderer::new(grid, Color::srgb(0.7, 0.4, 0.2)),
        Transform::default(),
        Visibility::default(),
    ));
}

pub fn render_grid(
    mut commands: Commands,
    grid_query: Query<(Entity, &GridRenderer), Changed<GridRenderer>>,
) {
    for (entity, grid_renderer) in grid_query.iter() {
        let grid: &Grid = &grid_renderer.grid;

        commands.entity(entity).despawn_children();

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if let Some(GridCell::Air) = grid.get(x, y) {
                    continue;
                }
                if let Some(quad) = grid.get_cell_quad(x, y) {
                    let center_x = quad.pos.x + quad.siz.x / 2.0;
                    let center_y = quad.pos.y + quad.siz.y / 2.0;

                    commands.entity(entity).with_children(|parent| {
                        parent.spawn((
                            Sprite {
                                color: grid_renderer.wall_color,
                                custom_size: Some(Vec2::new(quad.siz.x, quad.siz.y)),
                                ..default()
                            },
                            Transform::from_xyz(center_x, center_y, 0.0),
                        ));
                    });
                }
            }
        }
    }
}
