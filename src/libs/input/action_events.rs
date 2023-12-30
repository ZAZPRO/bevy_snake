use super::direction::Direction;
use bevy::prelude::*;

#[derive(Event)]
pub struct ActionMoveEvent(pub Direction);

#[derive(Event)]
pub struct ActionPauseEvent;

pub struct ActionEventsPlugin;

impl Plugin for ActionEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionMoveEvent>()
            .add_event::<ActionPauseEvent>();
    }
}
