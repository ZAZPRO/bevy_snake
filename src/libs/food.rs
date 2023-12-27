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

pub struct FoodPlugin;

fn spawn_food(mut commands: Commands) {
    commands.spawn(FoodBundle::new(4, 4));
    commands.spawn(FoodBundle::new(5, 5));
    commands.spawn(FoodBundle::new(8, 8));
}

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food);
    }
}
