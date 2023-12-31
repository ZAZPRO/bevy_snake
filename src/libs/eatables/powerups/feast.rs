use bevy::prelude::*;

use crate::libs::{
    animation::AnimationHandles,
    cell::Cell,
    eatables::{
        eat_event::EatEvent,
        food::{random_pos_food_bundle, Food},
    },
    game_configuration::GameConfiguration,
    schedule::InGameSet,
};

use super::powerup::Powerup;

fn feast_on_powerup(
    mut ev_eat: EventReader<EatEvent>,
    mut commands: Commands,
    animation_handles: Res<AnimationHandles>,
    query: Query<&Cell, Without<Food>>,
    game_configuration: Res<GameConfiguration>,
) {
    let mut iter = ev_eat.read();

    // As I expect to have only one event at a time, I don't need to iterate over it.
    // So I just consume the iterator if it's not empty.
    if iter.len() > 0 {
        let event = iter.next();

        for _ in iter {}

        if let Some(ev) = event {
            if ev.food.0 == Powerup::Feast {
                let food_bundles = random_pos_food_bundle(
                    animation_handles.breathe.clone(),
                    query,
                    game_configuration,
                    None,
                    Some(Powerup::Feast.power()),
                );

                for food_bundle in food_bundles {
                    commands.spawn(food_bundle);
                }
            }
        }
    }
}

pub struct FeastPowerupPlugin;

impl Plugin for FeastPowerupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, feast_on_powerup.in_set(InGameSet::SpawnEntities));
    }
}
