use bevy::prelude::*;

use super::{events::EatEvent, schedule::InGameSet};

fn despawn_food_on_eat(mut ev_eat: EventReader<EatEvent>, mut commands: Commands) {
    for ev in ev_eat.read() {
        commands.entity(ev.0).despawn();
    }
}

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_food_on_eat.in_set(InGameSet::DespawnEntities),
        );
    }
}
