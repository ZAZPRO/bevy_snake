use bevy::prelude::*;

use super::{events::EatEvent, game_states::GameState, schedule::InGameSet};

#[derive(Resource, Default)]
pub struct Score(pub u32);

fn grow_score_on_eat(mut ev_eat: EventReader<EatEvent>, mut score: ResMut<Score>) {
    for _ in ev_eat.read() {
        score.0 += 1;
    }
}

fn reset_score(mut score: ResMut<Score>) {
    *score = Score::default();
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_systems(
                Update,
                grow_score_on_eat.in_set(InGameSet::CollisionDetection),
            )
            .add_systems(OnExit(GameState::FinishMenu), reset_score);
    }
}
