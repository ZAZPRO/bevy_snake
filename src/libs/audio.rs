use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AudioAssets {
    pub eat_sound: Handle<AudioSource>,
    pub snake_movement_sound: Handle<AudioSource>,
}

fn load_assets(mut assets: ResMut<AudioAssets>, asset_server: Res<AssetServer>) {
    *assets = AudioAssets {
        eat_sound: asset_server.load("food_eat_recording_voice.ogg"),
        snake_movement_sound: asset_server.load("snake_movement_recording_voice.ogg"),
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioAssets::default())
            .add_systems(Startup, load_assets);
    }
}
