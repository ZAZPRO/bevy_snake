use bevy::prelude::*;

use super::{
    cell::Cell,
    food::Food,
    schedule::InGameSet,
    snake::{Head, Tail},
};

#[derive(Event)]
pub struct EatEvent(pub Entity);

#[derive(Event)]
pub struct SnakeSelfCollisionEvent;

fn send_eat_event(
    head: Query<&Cell, With<Head>>,
    foods: Query<(Entity, &Cell), With<Food>>,
    mut ev_eat: EventWriter<EatEvent>,
) {
    if let Ok(head) = head.get_single() {
        for (id, food) in foods.iter() {
            if *head == *food {
                ev_eat.send(EatEvent(id));
            }
        }
    }
}

fn send_snake_self_collision_event(
    head: Query<&Cell, With<Head>>,
    tails: Query<&Cell, With<Tail>>,
    mut ev_snake_self_collision: EventWriter<SnakeSelfCollisionEvent>,
) {
    // Len check is a dirty hack, but it works.
    if tails.iter().len() > 1 {
        if let Ok(head) = head.get_single() {
            for tails in tails.iter() {
                if *head == *tails {
                    ev_snake_self_collision.send(SnakeSelfCollisionEvent);
                }
            }
        }
    }
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EatEvent>()
            .add_event::<SnakeSelfCollisionEvent>()
            .add_systems(
                Update,
                (send_eat_event, send_snake_self_collision_event)
                    .in_set(InGameSet::CollisionDetection),
            );
    }
}
