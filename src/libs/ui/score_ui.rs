use bevy::prelude::*;

use crate::libs::{game_states::GameState, schedule::InGameSet, score::Score};

use super::ui_utils::despawn_ui;

#[derive(Component)]
struct ScoreUiTextTag;

fn spawn_score_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(8.0),
                height: Val::Percent(8.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font_size: 35.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                ScoreUiTextTag,
            ));
        });
}

fn update_score_ui(mut query: Query<&mut Text, With<ScoreUiTextTag>>, score: Res<Score>) {
    if score.is_changed() {
        let mut text = query.single_mut();
        text.sections[0].value = score.0.to_string();
    }
}

pub struct ScoreUiPlugin;

impl Plugin for ScoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_score_ui)
            .add_systems(
                Update,
                update_score_ui.in_set(InGameSet::GlobalPostionUpdates),
            )
            .add_systems(OnExit(GameState::InGame), despawn_ui);
    }
}
