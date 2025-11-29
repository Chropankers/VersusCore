use bevy::prelude::*;

use crate::components::{kinematics, state, time};

pub fn apply_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut kinematics::Velocity,
        &state::StateMachine,
        &kinematics::MovementConfig,
        Option<&time::TimeFreeze>,
    )>,
) { /* ... */ }
