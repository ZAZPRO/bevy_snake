use bevy::prelude::*;
use bevy_inspector_egui::{InspectorOptions, inspector_options::ReflectInspectorOptions};

use super::{globals::GAME_SPEED, schedule::InGameSet};

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GameConfiguration {
    pub tick_timer: Timer,
}

impl Default for GameConfiguration {
    fn default() -> Self {
        Self {
            tick_timer: Timer::from_seconds(GAME_SPEED, TimerMode::Repeating),
        }
    }
}

fn advance_timer(mut configuration: ResMut<GameConfiguration>, time: Res<Time>) {
    configuration.tick_timer.tick(time.delta());
}

pub struct GameConfigurationPlugin;

impl Plugin for GameConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameConfiguration>()
            .insert_resource(GameConfiguration::default())
            .add_systems(Update, advance_timer.in_set(InGameSet::DespawnEntities));
    }
}
