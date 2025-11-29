use bevy::prelude::*;

use crate::components::time::{GlobalTimeScale, TimeFreeze};

/// Counts down per-entity hitstop and removes the component at 0.
/// Optionally manipulates a GlobalTimeScale resource (for global slow/freeze).
pub fn update_time_freeze(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TimeFreeze)>,
    mut global_scale: Option<ResMut<GlobalTimeScale>>,
) {
    let mut any_frozen = false;

    for (entity, mut freeze) in query.iter_mut() {
        freeze.frames_remaining -= 1;
        if freeze.frames_remaining <= 0 {
            commands.entity(entity).remove::<TimeFreeze>();
        } else {
            any_frozen = true;
        }
    }

    if let Some(mut scale) = global_scale {
        // Simple behaviour: if anyone is frozen, set time to 0, else 1.
        scale.scale = if any_frozen { 0.0 } else { 1.0 };
    }
}
