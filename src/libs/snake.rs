use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use super::{
    cell::{Cell, CellBundle},
    food::Food,
    globals::{GRID_CENTER, HEAD_COLOR, TAIL_COLOR},
};

#[derive(Default, Reflect, InspectorOptions, PartialEq, Clone, Copy)]
#[reflect(InspectorOptions)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match &self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Head {
    pub direction: Direction,
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Tail;

#[derive(Bundle)]
pub struct HeadBundle {
    head: Head,
    cell: CellBundle,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Snake {
    pub parts: Vec<Entity>,
}

impl Snake {
    pub fn new(commands: &mut Commands, snake: &mut ResMut<Snake>) {
        let cell = Cell {
            x: GRID_CENTER,
            y: GRID_CENTER,
        };

        let id = commands
            .spawn(CellBundle::new_with_z(cell, HEAD_COLOR, 1.))
            .insert(Head {
                direction: Direction::Up,
            })
            .id();
        snake.parts.push(id);
    }

    pub fn new_tail(
        commands: &mut Commands,
        query: &Query<&Cell, Without<Food>>,
        snake: &mut ResMut<Snake>,
    ) {
        let last_id = snake.parts.last().unwrap();
        let last_cell = query.get(*last_id).unwrap();

        let id = commands
            .spawn(CellBundle::new(*last_cell, TAIL_COLOR))
            .insert(Tail)
            .id();
        snake.parts.push(id);
    }
}
