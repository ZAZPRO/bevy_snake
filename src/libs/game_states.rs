use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    StartMenu,
    InGame,
    FinishMenu,
}

pub struct GameStatatesPlugin;

impl Plugin for GameStatatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}
