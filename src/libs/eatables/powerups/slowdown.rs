use std::time::Duration;

use bevy::prelude::*;

use crate::libs::{
    eatables::eat_event::EatEvent, game_configuration::GameConfiguration, schedule::InGameSet,
};

use super::powerup::Powerup;

#[derive(Resource, Default)]
struct SlowdownPowerupTimer(Timer);

fn slowdown_on_powerup(
    mut ev_eat: EventReader<EatEvent>,
    mut configuration: ResMut<GameConfiguration>,
    mut powerup_timer: ResMut<SlowdownPowerupTimer>,
) {
    for ev in ev_eat.read() {
        if ev.food.0 == Powerup::Slowdown {
            let new_game_speed =
                configuration.current_difficulty.get_tick_rate() * Powerup::Slowdown.speed();
            // Change game speed.
            configuration.set_game_speed(new_game_speed);

            // Set timer duration.
            let powerup_timer_duration_secs = new_game_speed * Powerup::Slowdown.power() as f32;

            powerup_timer.0.pause();
            powerup_timer.0.reset();
            powerup_timer
                .0
                .set_duration(Duration::from_secs_f32(powerup_timer_duration_secs));
            powerup_timer.0.unpause();
        }
    }
}

fn revert_slowdown_on_timer(
    mut configuration: ResMut<GameConfiguration>,
    time: Res<Time>,
    mut powerup_timer: ResMut<SlowdownPowerupTimer>,
) {
    if powerup_timer.0.tick(time.delta()).just_finished() {
        let old_difficulty = configuration.current_difficulty;

        configuration.set_difficulty_and_reset_timer(old_difficulty);
    }
}

pub struct SlowdownPowerupPlugin;

impl Plugin for SlowdownPowerupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SlowdownPowerupTimer::default())
            .add_systems(
                Update,
                (slowdown_on_powerup, revert_slowdown_on_timer).in_set(InGameSet::EntityUpdates),
            );
    }
}
