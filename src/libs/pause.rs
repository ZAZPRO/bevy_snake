use bevy::prelude::*;

use super::{schedule::InGameSet, input::read_input::get_user_input};

fn pause_game(
    gamepads: Res<Gamepads>,
    keyboard_input: Res<Input<KeyCode>>,
    button_inputs: Res<Input<GamepadButton>>,
    mut time: ResMut<Time<Virtual>>,
) {
    let input_state = get_user_input(gamepads, keyboard_input, button_inputs);

    if input_state.action_pause {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pause_game.in_set(InGameSet::UserInput));
    }
}
