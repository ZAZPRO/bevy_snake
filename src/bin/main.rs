#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, window::PresentMode};
use bevy_particle_systems::ParticleSystemPlugin;
use snake::libs::{
    animation::CustomAnimationPlugin,
    audio::AudioPlugin,
    camera::CameraPlugin,
    cell::CellPlugin,
    events::EventsPlugin,
    food::FoodPlugin,
    game_configuration::GameConfigurationPlugin,
    game_states::GameStatatesPlugin,
    globals::{BACKGROUND_COLOR, WINDOW_SIZE},
    input::{read_input::ReadInputPlugin, action_events::ActionEventsPlugin},
    particles::ParticlePlugin,
    pause::PausePlugin,
    schedule::SchedulePlugin,
    score::ScorePlugin,
    snake::SnakePlugin,
    ui::{
        finish_menu::FinishMenuPlugin, pause_ui::PauseUiPlugin, score_ui::ScoreUiPlugin,
        start_menu::StartMenuPlugin,
    },
};

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".into(),
                resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                resizable: false,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SchedulePlugin)
        .add_plugins(GameStatatesPlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(ParticleSystemPlugin)
        .add_plugins(GameConfigurationPlugin)
        .add_plugins(CustomAnimationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(StartMenuPlugin)
        .add_plugins(ReadInputPlugin)
        .add_plugins(ActionEventsPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(CellPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(PausePlugin)
        .add_plugins(PauseUiPlugin)
        .add_plugins(ScoreUiPlugin)
        .add_plugins(FinishMenuPlugin);

    app.run();
}
