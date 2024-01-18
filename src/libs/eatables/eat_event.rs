use bevy::prelude::*;

use crate::libs::{cell::Cell, schedule::InGameSet, snake::Head};

use super::food::Food;

#[derive(Event)]
pub struct EatEvent {
    pub id: Entity,
    pub pos: Cell,
    pub food: Food,
}

fn send_eat_event(
    head: Query<&Cell, With<Head>>,
    foods: Query<(Entity, &Cell, &Food)>,
    mut ev_eat: EventWriter<EatEvent>,
) {
    if let Ok(&head) = head.get_single() {
        for (id, &pos, &food) in foods.iter() {
            if head == pos {
                ev_eat.send(EatEvent { id, pos, food });
            }
        }
    }
}

pub struct EatEventPlugin;

impl Plugin for EatEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EatEvent>()
            .add_systems(Update, send_eat_event.in_set(InGameSet::CollisionDetection));
    }
}
