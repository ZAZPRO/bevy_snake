use bevy::prelude::*;

pub fn despawn_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for menu in query.iter() {
        commands.entity(menu).despawn();
    }
}
