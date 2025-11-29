use bevy::prelude::*;

use crate::components::{health, input, state, tags};
use crate::round::RoundState;

/// Very small placeholder state update to keep characters responsive.
pub fn update_character_state(
    round: Res<RoundState>,
    mut query: Query<
        (&health::Health, &input::InputBuffer, &mut state::StateMachine),
        With<tags::CharacterTag>,
    >,
) {
    let _phase = round.phase;

    for (_health, _input, mut sm) in query.iter_mut() {
        sm.frames_in_state = sm.frames_in_state.saturating_add(1);

        // Simple recovery out of hitstun for now
        if sm.state == state::CharacterState::Hitstun && sm.frames_in_state > 30 {
            sm.state = state::CharacterState::Idle;
            sm.frames_in_state = 0;
        }
    }
}
