use bevy::prelude::*;

use crate::libs::{
    game_configuration::{self, GameConfiguration},
    game_states::GameState,
    utils::despawn_ui,
};

enum ButtonType {
    Easy,
    Medium,
    Hard,
    Extreme,
}

#[derive(Component)]
struct ButtonTag(ButtonType);

#[derive(Bundle)]
struct TaggedButtonBundle {
    button_tag: ButtonTag,
    button_bundle: ButtonBundle,
}

impl TaggedButtonBundle {
    fn create_new(
        parent: &mut ChildBuilder<'_, '_, '_>,
        button_type: ButtonType,
        text: impl Into<String>,
    ) {
        parent
            .spawn(Self {
                button_tag: ButtonTag(button_type),
                button_bundle: ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            left: Val::Percent(2.),
                            right: Val::Percent(2.),
                            top: Val::Percent(2.),
                            bottom: Val::Percent(2.),
                        },
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
    }
}

fn create_menu(mut commands: Commands) {
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
            TaggedButtonBundle::create_new(parent, ButtonType::Easy, "Easy");
            TaggedButtonBundle::create_new(parent, ButtonType::Medium, "Medium");
            TaggedButtonBundle::create_new(parent, ButtonType::Hard, "Hard");
            TaggedButtonBundle::create_new(parent, ButtonType::Extreme, "Extreme");
        });
}

fn button_click(
    query: Query<(&Interaction, &ButtonTag), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_configuration: ResMut<GameConfiguration>,
) {
    for (interaction, tag) in query.iter() {
        if interaction == &Interaction::Pressed {
            match tag.0 {
                ButtonType::Easy => {
                    game_configuration
                        .set_difficulty_and_reset_timer(game_configuration::GameDifficulty::Easy);
                }
                ButtonType::Medium => {
                    game_configuration
                        .set_difficulty_and_reset_timer(game_configuration::GameDifficulty::Medium);
                }
                ButtonType::Hard => {
                    game_configuration
                        .set_difficulty_and_reset_timer(game_configuration::GameDifficulty::Hard);
                }
                ButtonType::Extreme => {
                    game_configuration.set_difficulty_and_reset_timer(
                        game_configuration::GameDifficulty::Extreme,
                    );
                }
            }
            next_state.set(GameState::InGame);
        }
    }
}

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), create_menu)
            .add_systems(Update, button_click.run_if(in_state(GameState::StartMenu)))
            .add_systems(OnExit(GameState::StartMenu), despawn_ui);
    }
}
