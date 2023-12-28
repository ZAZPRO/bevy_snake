use bevy::prelude::*;

use super::{game_states::GameState, score::Score, utils::despawn_ui};

fn create_menu(mut commands: Commands, score: ResMut<Score>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Your score: {}", score.0),
                TextStyle {
                    font_size: 80.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            left: Val::Percent(10.),
                            right: Val::Percent(10.),
                            top: Val::Percent(10.),
                            bottom: Val::Percent(10.),
                        },
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Main menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

fn button_click(
    query: Query<(&Interaction,), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction,) in query.iter() {
        if interaction == &Interaction::Pressed {
            next_state.set(GameState::StartMenu);
        }
    }
}

pub struct FinishMenuPlugin;

impl Plugin for FinishMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::FinishMenu), create_menu)
            .add_systems(Update, button_click.run_if(in_state(GameState::FinishMenu)))
            .add_systems(OnExit(GameState::FinishMenu), despawn_ui);
    }
}
