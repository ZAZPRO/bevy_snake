use bevy::prelude::*;

use crate::libs::{
    eatables::eat_event::EatEvent,
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
            Snake::remove_tails(&mut commands, &query, &mut snake, Powerup::Shorten.power());
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
