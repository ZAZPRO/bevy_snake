use bevy::prelude::*;

use crate::libs::schedule::InGameSet;

#[derive(Component)]
struct PausedUiTag;

fn spaw_pause_ui(
    mut commands: Commands,
    query: Query<Entity, With<PausedUiTag>>,
    time: Res<Time<Virtual>>,
) {
    if time.is_paused() && query.is_empty() {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                },
                PausedUiTag,
            ))
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

fn despawn_pause_ui(
    mut commands: Commands,
    query: Query<Entity, With<PausedUiTag>>,
    time: Res<Time<Virtual>>,
) {
    if !time.is_paused() && !query.is_empty() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct PauseUiPlugin;

impl Plugin for PauseUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spaw_pause_ui.in_set(InGameSet::UserInput))
            .add_systems(Update, despawn_pause_ui.in_set(InGameSet::UserInput));
    }
}
