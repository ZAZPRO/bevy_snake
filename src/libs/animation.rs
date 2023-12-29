use bevy::prelude::*;

use super::globals::GRID_CELL;

pub const BREATHE_ANIMATION_NAME: &str = "breathe";

#[derive(Resource, Default, Clone)]
pub struct AnimationHandles {
    pub breathe: Handle<AnimationClip>,
}

fn create_animations(
    mut animations: ResMut<Assets<AnimationClip>>,
    mut animation_handles: ResMut<AnimationHandles>,
) {
    // The animation API uses the `Name` component to target entities
    let breathe_name = Name::new(BREATHE_ANIMATION_NAME);

    let mut animation = AnimationClip::default();
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![breathe_name],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 1.5],
            keyframes: Keyframes::Scale(vec![
                Vec3::splat(GRID_CELL * 1.0),
                Vec3::splat(GRID_CELL * 0.8),
                Vec3::splat(GRID_CELL * 1.0),
            ]),
        },
    );

    let handle = animations.add(animation);
    animation_handles.breathe = handle;
}

pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationHandles::default())
            .add_systems(PostStartup, create_animations);
    }
}
