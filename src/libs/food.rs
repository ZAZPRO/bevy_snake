use bevy::prelude::*;
use rand::Rng;

use super::{
    cell::{Cell, CellBundle},
    events::EatEvent,
    game_states::GameState,
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
        commands.entity(ev.id).despawn();
    }
}

fn spawn_food_on_eat(mut ev_eat: EventReader<EatEvent>, mut commands: Commands) {
    for _ in ev_eat.read() {
        commands.spawn(random_pos_food_bundle());
    }
}

fn destroy_food(mut commands: Commands, query: Query<Entity, With<Food>>) {
    for food in query.iter() {
        commands.entity(food).despawn();
    }
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_food_randomly)
            .add_systems(Update, spawn_food_on_eat.in_set(InGameSet::SpawnEntities))
            .add_systems(
                Update,
                despawn_food_on_eat.in_set(InGameSet::DespawnEntities),
            )
            .add_systems(
                OnExit(GameState::InGame),
                destroy_food.in_set(InGameSet::DespawnEntities),
            );
    }
}
