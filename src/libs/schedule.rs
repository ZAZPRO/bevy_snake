use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InGameSet {
    DespawnEntities,
    UserInput,
    SpawnEntities,
    EntityUpdates,
    GlobalPostionUpdates,
    CollisionDetection,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                // Flush commands (i.e. `apply_deferred` runs)
                InGameSet::UserInput,
                InGameSet::SpawnEntities,
                InGameSet::EntityUpdates,
                InGameSet::GlobalPostionUpdates,
                InGameSet::CollisionDetection,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::DespawnEntities)
                .before(InGameSet::UserInput),
        );
    }
}
