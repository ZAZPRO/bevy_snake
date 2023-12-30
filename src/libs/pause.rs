use bevy::prelude::*;

use super::{input::get_user_input, schedule::InGameSet, utils};

fn pause_game(
    mut commands: Commands,
    gamepads: Res<Gamepads>,
    keyboard_input: Res<Input<KeyCode>>,
    button_inputs: Res<Input<GamepadButton>>,
    mut time: ResMut<Time<Virtual>>,
    query: Query<Entity, With<Node>>,
) {
    let input_state = get_user_input(gamepads, keyboard_input, button_inputs);

    if input_state.action_pause {
        if time.is_paused() {
            utils::despawn_ui(commands, query);

            time.unpause();
        } else {
            time.pause();

            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Paused",
                        TextStyle {
                            font_size: 40.0,
                            // color: Color::rgb(0.9, 0.9, 0.9),
                            color: Color::rgb(0.1, 0.1, 0.1),
                            ..default()
                        },
                    ));
                });
        }
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pause_game.in_set(InGameSet::UserInput));
    }
}
