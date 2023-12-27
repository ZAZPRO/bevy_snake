use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    math::Vec2,
    ui::Node,
};

use super::globals::{GRID_CELL, LEFT_WINDOW_BORDER, TOP_WINDOW_BORDER};

pub fn grid_to_screen(grid_x: u32, grid_y: u32) -> Vec2 {
    Vec2 {
        x: LEFT_WINDOW_BORDER + GRID_CELL / 2. + GRID_CELL * grid_x as f32,
        y: TOP_WINDOW_BORDER - GRID_CELL / 2. - GRID_CELL * grid_y as f32,
    }
}

pub fn despawn_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for menu in query.iter() {
        commands.entity(menu).despawn();
    }
}
