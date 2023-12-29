use bevy::prelude::*;
use rand::Rng;

use super::{
    animation::{AnimationHandles, BREATHE_ANIMATION_NAME},
    cell::{Cell, CellBundle},
    events::EatEvent,
    game_configuration::GameConfiguration,
    game_states::GameState,
    globals::FOOD_COLOR,
    schedule::InGameSet,
};

#[derive(Component)]
pub struct Food;

#[derive(Bundle)]
pub struct FoodBundle {
    pub food: Food,
    pub cell: CellBundle,
    pub animation: AnimationPlayer,
    pub name: Name,
}

impl FoodBundle {
    pub fn new(grid_x: u32, grid_y: u32, animation: Handle<AnimationClip>) -> Self {
        let cell = Cell {
            x: grid_x,
            y: grid_y,
        };

        let mut player = AnimationPlayer::default();
        player.play(animation).repeat();

        Self {
            food: Food,
            cell: CellBundle::new(cell, FOOD_COLOR),
            name: Name::new(BREATHE_ANIMATION_NAME),
            animation: player,
        }
    }
}

fn random_pos_food_bundle(
    animation: Handle<AnimationClip>,
    query: Query<&Cell, Without<Food>>,
    game_configuration: Res<GameConfiguration>,
) -> FoodBundle {
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

    let random_pos_id = rand::thread_rng().gen_range(0..taken_pos.len());
    let random_pos = taken_pos[random_pos_id];

    FoodBundle::new(random_pos.x, random_pos.y, animation)
}

fn spawn_food_randomly(
    mut commands: Commands,
    animation_handles: Res<AnimationHandles>,
    query: Query<&Cell, Without<Food>>,
    game_configuration: Res<GameConfiguration>,
) {
    commands.spawn(random_pos_food_bundle(
        animation_handles.breathe.clone(),
        query,
        game_configuration,
    ));
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
    game_configuration: Res<GameConfiguration>,
) {
    let i = ev_eat.read();

    // As I expect to have only one event at a time, I don't need to iterate over it.
    // So I just consume the iterator if it's not empty.
    if i.len() > 0 {
        for _ in i {}

        commands.spawn(random_pos_food_bundle(
            animation_handles.breathe.clone(),
            query,
            game_configuration,
        ));
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
