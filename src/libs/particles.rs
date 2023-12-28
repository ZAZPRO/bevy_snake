use bevy::prelude::*;
use bevy_particle_systems::{
    JitteredValue, ParticleBurst, ParticleSystem, ParticleSystemBundle, Playing, VelocityModifier,
};

use crate::libs::globals::FOOD_COLOR;

use super::{events::EatEvent, schedule::InGameSet, utils::grid_to_screen};

fn spawn_particle_on_eat(mut ev_eat: EventReader<EatEvent>, mut commands: Commands) {
    for ev in ev_eat.read() {
        let world_pos = grid_to_screen(ev.pos.x, ev.pos.y);

        commands.spawn((
            ParticleSystemBundle {
                transform: Transform::from_translation(Vec3 {
                    x: world_pos.x,
                    y: world_pos.y,
                    z: 2.,
                }),
                particle_system: ParticleSystem {
                    spawn_rate_per_second: 0.0.into(),
                    max_particles: 50,
                    initial_speed: (0.0..100.0).into(),
                    lifetime: JitteredValue::new(0.4),
                    scale: 2.0.into(),
                    velocity_modifiers: vec![VelocityModifier::Drag(0.001.into())],
                    color: FOOD_COLOR.into(),
                    bursts: vec![ParticleBurst {
                        time: 0.0,
                        count: 50,
                    }],
                    ..ParticleSystem::oneshot()
                },
                ..default()
            },
            Playing,
        ));
    }
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_particle_on_eat.in_set(InGameSet::SpawnEntities),
        );
    }
}
