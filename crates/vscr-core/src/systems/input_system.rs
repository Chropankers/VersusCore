use bevy::prelude::*;

use crate::components::{input, tags};

/// Copies player input state into each tagged InputBuffer.
pub fn apply_player_input(
    mut query: Query<(&tags::PlayerTag, &mut input::InputBuffer)>,
    input_res: Res<crate::output_bridge::PlayerInputState>,
) {
    for (tag, mut buffer) in query.iter_mut() {
        let buttons = match tag.id {
            1 => input_res.p1_buttons,
            2 => input_res.p2_buttons,
            _ => input::Buttons::empty(),
        };

        if buffer.current == buttons {
            buffer.frames_held = buffer.frames_held.saturating_add(1);
        } else {
            buffer.current = buttons;
            buffer.frames_held = 0;
        }
    }
}
