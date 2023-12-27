use bevy::prelude::*;

use super::{
    cell::{Cell, CellBundle},
    globals::FOOD_COLOR,
};

#[derive(Component)]
pub struct Food;

#[derive(Bundle)]
pub struct FoodBundle {
    pub food: Food,
    pub cell: CellBundle,
}

impl FoodBundle {
    pub fn new(grid_x: u32, grid_y: u32) -> Self {
        let cell = Cell {
            x: grid_x,
            y: grid_y,
        };

        Self {
            food: Food,
            cell: CellBundle::new(cell, FOOD_COLOR),
        }
    }
}
