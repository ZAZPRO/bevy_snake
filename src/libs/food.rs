use bevy::prelude::*;
use rand::Rng;

use super::{
    animation::{AnimationHandles, BREATHE_ANIMATION_NAME},
    cell::{Cell, CellBundle},
    events::EatEvent,
    game_configuration::GameConfiguration,
    game_states::GameState,
    schedule::InGameSet,
};

#[derive(Component, Debug, Clone, Copy)]
pub enum Powerup {
    Normal,
    Slowdown,
    Shorten,
    Feast,
    //TODO
    //Ghost,
}

impl Powerup {
    pub fn get_color(&self) -> Color {
        match self {
            Powerup::Normal => Color::rgb(0.9, 0.1, 0.1),
            Powerup::Slowdown => Color::rgb(0.0, 0.0, 0.9),
            Powerup::Shorten => Color::rgb(0.9, 0.9, 0.0),
            Powerup::Feast => Color::rgb(0.0, 0.9, 0.0),
        }
    }

    fn get_chance(&self) -> f32 {
        match self {
            Powerup::Normal => 0.80,
            Powerup::Shorten => 0.1,
            Powerup::Feast => 0.05,
            Powerup::Slowdown => 0.05,
        }
    }

    fn chance_to_powerup(random_number: f32) -> Powerup {
        if random_number < Powerup::Feast.get_chance() {
            Powerup::Feast
        } else if random_number < Powerup::Feast.get_chance() + Powerup::Shorten.get_chance() {
            Powerup::Shorten
        } else if random_number
            < Powerup::Feast.get_chance()
                + Powerup::Shorten.get_chance()
                + Powerup::Slowdown.get_chance()
        {
            Powerup::Slowdown
        } else {
            Powerup::Normal
        }
    }

    fn get_random_powerup() -> Powerup {
        let random_number = rand::thread_rng().gen_range(0.0..1.0);
        let powerup = Powerup::chance_to_powerup(random_number);
        println!("Chance {}, powerup: {:?}", random_number, powerup);
        powerup
    }

    fn get_speed(&self) -> f32 {
        match self {
            Powerup::Normal => 1.0,
            Powerup::Slowdown => 0.5,
            Powerup::Shorten => 1.0,
            Powerup::Feast => 1.0,
        }
    }

    fn get_duration(&self) -> u32 {
        match self {
            Powerup::Normal => 0,
            Powerup::Slowdown => 5,
            Powerup::Shorten => 5,
            Powerup::Feast => 0,
        }
    }
}

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

fn random_pos_food_bundle(
    animation: Handle<AnimationClip>,
    query: Query<&Cell, Without<Food>>,
    game_configuration: Res<GameConfiguration>,
    powerup: Option<Powerup>,
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

    let powerup = match powerup {
        Some(powerup) => powerup,
        None => Powerup::get_random_powerup(),
    };

    FoodBundle::new(random_pos.x, random_pos.y, animation, powerup)
}

fn spawn_first_food(
    mut commands: Commands,
    animation_handles: Res<AnimationHandles>,
    query: Query<&Cell, Without<Food>>,
    game_configuration: Res<GameConfiguration>,
) {
    commands.spawn(random_pos_food_bundle(
        animation_handles.breathe.clone(),
        query,
        game_configuration,
        Some(Powerup::Normal),
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
            None,
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
