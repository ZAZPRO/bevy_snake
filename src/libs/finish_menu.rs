use bevy::prelude::*;

use super::game_states::GameState;

// Just a placeholder right now
fn finish_redirect(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::StartMenu);
}

pub struct FinishMenuPlugin;

impl Plugin for FinishMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::FinishMenu), finish_redirect);
    }
}
