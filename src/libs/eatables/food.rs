use bevy::prelude::*;
use rand::Rng;

use crate::libs::{
    animation::{AnimationHandles, BREATHE_ANIMATION_NAME},
    cell::{Cell, CellBundle},
    game_configuration::GameConfiguration,
    game_states::GameState,
    schedule::InGameSet,
};

use super::{eat_event::EatEvent, powerups::powerup::Powerup};

#[derive(Component, Clone, Copy)]
pub struct Food(pub Powerup);

#[derive(Bundle)]
pub struct FoodBundle {
    pub food: Food,
    pub cell: CellBundle,
    pub animation: AnimationPlayer,
    pub name: Name,
}

impl FoodBundle {
    pub fn new(
        grid_x: u32,
        grid_y: u32,
        animation: Handle<AnimationClip>,
        powerup: Powerup,
    ) -> Self {
        let cell = Cell {
            x: grid_x,
            y: grid_y,
        };

        let mut player = AnimationPlayer::default();
        player.play(animation).repeat();

        Self {
            food: Food(powerup),
            cell: CellBundle::new(cell, powerup.get_color()),
            name: Name::new(BREATHE_ANIMATION_NAME),
            animation: player,
        }
    }
}

pub fn random_pos_food_bundle(
    animation: Handle<AnimationClip>,
    query: Query<&Cell, Without<Food>>,
    game_configuration: Res<GameConfiguration>,
    powerup: Option<Powerup>,
    amount: Option<u32>,
) -> Vec<FoodBundle> {
    // My crude logic, this can be improved in many ways.
    // 1. Get a Vec of game field size filled with all possible positions.
    // 2. Remove taken positions.
    // 3. Pick a random position from the remaining ones.

    let mut taken_pos: Vec<Cell> = game_configuration.field.clone();

    for cell in query.iter() {
        let id = taken_pos.iter().position(|&c| c == *cell);
        if let Some(id) = id {
            taken_pos.remove(id);
        };
    }

    let amount: u32 = amount.unwrap_or(1);

    let mut food_bundles: Vec<FoodBundle> = Vec::new();

    for _ in 0..amount {
        let random_pos_id = rand::thread_rng().gen_range(0..taken_pos.len());
        let random_pos = taken_pos[random_pos_id];
        taken_pos.remove(random_pos_id);

        let powerup = powerup.unwrap_or(Powerup::get_random_powerup());

        let food_bundle = FoodBundle::new(random_pos.x, random_pos.y, animation.clone(), powerup);
        food_bundles.push(food_bundle);
    }

    food_bundles
}

fn spawn_first_food(
    mut commands: Commands,
    animation_handles: Res<AnimationHandles>,
    query: Query<&Cell, Without<Food>>,
    game_configuration: Res<GameConfiguration>,
) {
    let food_bundle = random_pos_food_bundle(
        animation_handles.breathe.clone(),
        query,
        game_configuration,
        Some(Powerup::Normal),
        None,
    )
    .pop()
    .unwrap();

    commands.spawn(food_bundle);
}

fn despawn_food_on_eat(mut ev_eat: EventReader<EatEvent>, mut commands: Commands) {
    for ev in ev_eat.read() {
        commands.entity(ev.id).despawn();
    }
}

fn spawn_food_on_eat(
    mut ev_eat: EventReader<EatEvent>,
    mut commands: Commands,
    animation_handles: Res<AnimationHandles>,
    query: Query<&Cell, Without<Food>>,
    food_on_field: Query<&Food>,
    game_configuration: Res<GameConfiguration>,
) {
    let mut iter = ev_eat.read();

    // As I expect to have only one event at a time, I don't need to iterate over it.
    // So I just consume the iterator if it's not empty.
    if iter.len() > 0 {
        let event = iter.next();

        for _ in iter {}

        if let Some(ev) = event {
            if ev.food.0 != Powerup::Feast && food_on_field.iter().len() == 0 {
                let food_bundle = random_pos_food_bundle(
                    animation_handles.breathe.clone(),
                    query,
                    game_configuration,
                    None,
                    None,
                )
                .pop()
                .unwrap();

                commands.spawn(food_bundle);
            }
        }
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
        app.add_systems(OnEnter(GameState::InGame), spawn_first_food)
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
