use bevy::prelude::*;

pub struct InputState {
    pub input_up: bool,
    pub input_down: bool,
    pub input_left: bool,
    pub input_right: bool,
}

pub fn get_user_input(
    gamepads: Res<Gamepads>,
    keyboard_input: Res<Input<KeyCode>>,
    button_inputs: Res<Input<GamepadButton>>,
) -> InputState {
    let gamepad = gamepads.iter().last();

    let gamepad_up: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp))
    } else {
        false
    };
    let keyboard_up = keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W);

    let gamepad_down: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadDown))
    } else {
        false
    };
    let keyboard_down = keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S);

    let gamepad_left: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft))
    } else {
        false
    };
    let keyboard_left = keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A);

    let gamepad_right: bool = if let Some(gamepad) = gamepad {
        button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadRight))
    } else {
        false
    };
    let keyboard_right =
        keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D);

    InputState {
        input_up: (gamepad_up || keyboard_up),
        input_down: (gamepad_down || keyboard_down),
        input_left: (gamepad_left || keyboard_left),
        input_right: (gamepad_right || keyboard_right),
    }
}
