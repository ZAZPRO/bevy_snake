use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use super::{globals::GRID_CELL, utils::grid_to_screen};

#[derive(Component, Clone, Copy, Default, PartialEq, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
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
            cell: cell,
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
