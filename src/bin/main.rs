#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use snake::libs::{
    camera::CameraPlugin,
    cell::CellPlugin,
    events::EventsPlugin,
    finish_menu::FinishMenuPlugin,
    food::FoodPlugin,
    game_states::GameStatatesPlugin,
    globals::{BACKGROUND_COLOR, WINDOW_SIZE},
    schedule::SchedulePlugin,
    snake::SnakePlugin,
    start_menu::StartMenuPlugin,
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
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SchedulePlugin)
        .add_plugins(GameStatatesPlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(StartMenuPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CellPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(SnakePlugin)
        .add_plugins(FinishMenuPlugin);

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
