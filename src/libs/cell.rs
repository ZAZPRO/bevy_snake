use bevy::prelude::*;

use super::{
    food::Food, game_configuration::GameConfiguration, globals::GRID_CELL, schedule::InGameSet,
    utils::grid_to_screen,
};

#[derive(Component, Clone, Copy, Default, PartialEq, Reflect)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

#[derive(Bundle)]
pub struct CellBundle {
    pub cell: Cell,
    pub sprite: SpriteBundle,
}

impl CellBundle {
    pub fn new(cell: Cell, color: Color) -> Self {
        Self::new_with_z(cell, color, 0.)
    }

    pub fn new_with_z(cell: Cell, color: Color, z: f32) -> Self {
        let pos = grid_to_screen(cell.x, cell.y);

        Self {
            cell,
            sprite: SpriteBundle {
                sprite: Sprite { color, ..default() },
                transform: Transform {
                    translation: Vec3 {
                        x: pos.x,
                        y: pos.y,
                        z,
                    },
                    scale: Vec3 {
                        x: GRID_CELL,
                        y: GRID_CELL,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn update_cells_positions(
    mut query: Query<(&Cell, &mut Transform), Without<Food>>,
    game_configration: Res<GameConfiguration>,
) {
    if game_configration.tick_timer.just_finished() {
        for (cell, mut transform) in query.iter_mut() {
            let new_pos = grid_to_screen(cell.x, cell.y);

            transform.translation = Vec3 {
                x: new_pos.x,
                y: new_pos.y,
                z: transform.translation.z,
            };
        }
    }
}

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Cell>().add_systems(
            Update,
            update_cells_positions.in_set(InGameSet::GlobalPostionUpdates),
        );
    }
}
