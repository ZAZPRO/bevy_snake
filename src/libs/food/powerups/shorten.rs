use bevy::prelude::*;

use crate::libs::{
    food::eat_event::EatEvent,
    schedule::InGameSet,
    snake::{Snake, Tail},
};

use super::powerup::Powerup;

fn shorten_snake_on_powerup(
    mut ev_eat: EventReader<EatEvent>,
    mut commands: Commands,
    query: Query<Entity, With<Tail>>,
    mut snake: ResMut<Snake>,
) {
    for ev in ev_eat.read() {
        if ev.food.0 == Powerup::Shorten {
            Snake::remove_last_tail(&mut commands, &query, &mut snake);
        }
    }
}

pub struct ShortenPowerupPlugin;

impl Plugin for ShortenPowerupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            shorten_snake_on_powerup.in_set(InGameSet::DespawnEntities),
        );
    }
}
