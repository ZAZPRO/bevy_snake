#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;

use bevy::{
    prelude::*,
    window::{PresentMode, PrimaryWindow},
    winit::WinitWindows,
};
use bevy_particle_systems::ParticleSystemPlugin;
use snake::libs::{
    animation::CustomAnimationPlugin,
    audio::AudioPlugin,
    camera::CameraPlugin,
    cell::CellPlugin,
    eatables::{eat_event::EventsPlugin, food::FoodPlugin, powerups::powerup::PowerupPlugins},
    game_configuration::GameConfigurationPlugin,
    game_states::GameStatatesPlugin,
    globals::{BACKGROUND_COLOR, WINDOW_SIZE},
    input::{action_events::ActionEventsPlugin, read_input::ReadInputPlugin},
    particles::ParticlePlugin,
    pause::PausePlugin,
    schedule::SchedulePlugin,
    score::ScorePlugin,
    snake::SnakePlugin,
    ui::snake_ui_plugin::SnakeUiPlugins,
};
use winit::window::Icon;

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!("../../build/windows/icon.png"));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

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
        .add_systems(PreStartup, set_window_icon)
        .add_plugins(SchedulePlugin)
        .add_plugins(GameStatatesPlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(ParticleSystemPlugin)
        .add_plugins(GameConfigurationPlugin)
        .add_plugins(CustomAnimationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ReadInputPlugin)
        .add_plugins(ActionEventsPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(CellPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(PausePlugin)
        .add_plugins(PowerupPlugins)
        .add_plugins(SnakeUiPlugins);

    app.run();
}
