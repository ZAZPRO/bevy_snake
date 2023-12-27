use bevy::prelude::*;

use super::{cell::Cell, food::Food, schedule::InGameSet, snake::Head};

#[derive(Event)]
pub struct EatEvent(pub Entity);

fn send_eat_event(
    head: Query<&Cell, With<Head>>,
    foods: Query<(Entity, &Cell), With<Food>>,
    mut ev_eat: EventWriter<EatEvent>,
) {
    if let Ok(head) = head.get_single() {
        for (id, food) in foods.iter() {
            if *head == *food {
                ev_eat.send(EatEvent(id));
                // commands.entity(id).despawn();
            }
        }
    }
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EatEvent>()
            .add_systems(Update, send_eat_event.in_set(InGameSet::CollisionDetection));
    }
}
