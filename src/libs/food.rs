use bevy::prelude::*;
use rand::Rng;

use super::{
    cell::{Cell, CellBundle},
    events::EatEvent,
    globals::{FOOD_COLOR, GRID_SIZE},
    schedule::InGameSet,
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

fn random_pos_food_bundle() -> FoodBundle {
    let x = rand::thread_rng().gen_range(0..GRID_SIZE);
    let y = rand::thread_rng().gen_range(0..GRID_SIZE);

    FoodBundle::new(x, y)
}

fn spawn_food_randomly(mut commands: Commands) {
    commands.spawn(random_pos_food_bundle());
}

fn despawn_food_on_eat(mut ev_eat: EventReader<EatEvent>, mut commands: Commands) {
    for ev in ev_eat.read() {
        commands.entity(ev.0).despawn();
    }
}

fn spawn_food_on_eat(mut ev_eat: EventReader<EatEvent>, mut commands: Commands) {
    for _ in ev_eat.read() {
        commands.spawn(random_pos_food_bundle());
    }
}

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food_randomly)
            .add_systems(
                Update,
                despawn_food_on_eat.in_set(InGameSet::DespawnEntities),
            )
            .add_systems(Update, spawn_food_on_eat.in_set(InGameSet::SpawnEntities));
    }
}
