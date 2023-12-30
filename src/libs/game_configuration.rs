use std::time::Duration;

use bevy::prelude::*;

use super::{
    cell::Cell,
    globals::{BASE_GAME_SPEED, GRID_SIZE},
    schedule::InGameSet,
};

#[derive(Default)]
pub enum GameDifficulty {
    Easy,
    #[default]
    Medium,
    Hard,
    Extreme,
}

impl GameDifficulty {
    pub fn get_tick_rate(&self) -> f32 {
        match self {
            GameDifficulty::Easy => BASE_GAME_SPEED * 1.25,
            GameDifficulty::Medium => BASE_GAME_SPEED,
            GameDifficulty::Hard => BASE_GAME_SPEED * 0.5,
            GameDifficulty::Extreme => BASE_GAME_SPEED * 0.25,
        }
    }
}

#[derive(Resource, Reflect)]
pub struct GameConfiguration {
    pub tick_timer: Timer,
    pub field: Vec<Cell>,
}

impl GameConfiguration {
    pub fn new(difficulty: GameDifficulty) -> Self {
        let mut field: Vec<Cell> = Vec::with_capacity((GRID_SIZE * GRID_SIZE) as usize);

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                field.push(Cell { x, y });
            }
        }

        Self {
            tick_timer: Timer::from_seconds(difficulty.get_tick_rate(), TimerMode::Repeating),
            field,
        }
    }

    pub fn set_difficulty(&mut self, difficulty: GameDifficulty) {
        self.tick_timer.pause();
        self.tick_timer.reset();
        self.tick_timer
            .set_duration(Duration::from_secs_f32(difficulty.get_tick_rate()));
        self.tick_timer.unpause();
    }
}

impl Default for GameConfiguration {
    fn default() -> Self {
        Self::new(GameDifficulty::default())
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
