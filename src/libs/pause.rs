use bevy::prelude::*;

use super::{input::action_events::ActionPauseEvent, schedule::InGameSet};

fn pause_game(mut ev_pause: EventReader<ActionPauseEvent>, mut time: ResMut<Time<Virtual>>) {
    for _ in ev_pause.read() {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}

pub struct GamePausePlugin;

impl Plugin for GamePausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pause_game.in_set(InGameSet::UserInput));
    }
}
