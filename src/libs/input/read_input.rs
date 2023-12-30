use crate::libs::schedule::InGameSet;

use super::action_events::{ActionMoveEvent, ActionPauseEvent};
use super::direction::Direction;

use bevy::prelude::*;

pub fn get_user_input(
    gamepads: Res<Gamepads>,
    keyboard_input: Res<Input<KeyCode>>,
    button_inputs: Res<Input<GamepadButton>>,
    mut ev_action_move: EventWriter<ActionMoveEvent>,
    mut ev_action_pause: EventWriter<ActionPauseEvent>,
    time: Res<Time<Virtual>>,
) {
    let gamepad = gamepads.iter().last();

    let gamepad_up: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp))
    } else {
        false
    };
    let keyboard_up =
        keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W);

    let gamepad_down: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown))
    } else {
        false
    };
    let keyboard_down =
        keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S);

    let gamepad_left: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft))
    } else {
        false
    };
    let keyboard_left =
        keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A);

    let gamepad_right: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight))
    } else {
        false
    };
    let keyboard_right =
        keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D);

    let gamepad_select: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::Select))
    } else {
        false
    };
    let keyboard_esc = keyboard_input.just_pressed(KeyCode::Escape);

    // Do not produce move actions on pause.
    if !time.is_paused() {
        if gamepad_up || keyboard_up {
            ev_action_move.send(ActionMoveEvent(Direction::Up));
        }

        if gamepad_down || keyboard_down {
            ev_action_move.send(ActionMoveEvent(Direction::Down));
        }

        if gamepad_left || keyboard_left {
            ev_action_move.send(ActionMoveEvent(Direction::Left));
        }

        if gamepad_right || keyboard_right {
            ev_action_move.send(ActionMoveEvent(Direction::Right));
        }
    }

    if gamepad_select || keyboard_esc {
        ev_action_pause.send(ActionPauseEvent);
    }
}
pub struct ReadInputPlugin;

impl Plugin for ReadInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, get_user_input.in_set(InGameSet::UserInput));
    }
}
