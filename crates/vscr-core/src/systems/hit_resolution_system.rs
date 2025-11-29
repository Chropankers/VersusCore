use bevy::prelude::*;

use crate::components::{
    health::Health,
    state::{CharacterState, StateMachine},
    time::TimeFreeze,
    tags::CharacterTag,
};
use crate::events::{HitEvent, KoEvent};

/// Applies damage, changes state, adds hitstop, and emits KoEvent when needed.
pub fn resolve_hits(
    mut commands: Commands,
    mut hit_reader: EventReader<HitEvent>,
    mut char_query: Query<(Entity, &mut Health, Option<&mut StateMachine>), With<CharacterTag>>,
    mut ko_writer: EventWriter<KoEvent>,
) {
    for hit in hit_reader.read() {
        if let Ok((_victim_ent, mut health, maybe_state)) = char_query.get_mut(hit.victim) {
            // Apply damage
            health.current -= hit.damage;

            // Put victim into hitstun if they are not already KO’d
            if health.current > 0 {
                if let Some(mut sm) = maybe_state {
                    sm.state = CharacterState::Hitstun;
                    sm.frames_in_state = 0;
                }
            }

            // Add hitstop to victim (and optionally to attacker later)
            commands.entity(hit.victim).insert(TimeFreeze {
                frames_remaining: hit.hitstun_frames.min(15) as i16, // short freeze for now
            });

            // KO check
            if health.current <= 0 {
                ko_writer.send(KoEvent {
                    winner: hit.attacker,
                    loser: hit.victim,
                });
            }
        }
    }
}
