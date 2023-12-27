use bevy::prelude::*;

pub struct CameraPlugin;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
