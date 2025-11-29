use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundPhase {
    Intro,
    Ready,
    Fight,
    KoFreeze,
    Result,
}

#[derive(Resource, Reflect)]
pub struct RoundState {
    pub phase: RoundPhase,
    pub frames_in_phase: u32,
}

impl Default for RoundState {
    fn default() -> Self {
        Self { phase: RoundPhase::Intro, frames_in_phase: 0 }
    }
}

pub fn register_round_flow(app: &mut App) {
    app.add_systems(Update, advance_round_state);
}

fn advance_round_state(
    mut round: ResMut<RoundState>,
    mut round_events: EventWriter<crate::events::RoundStateEvent>,
    ko_events: EventReader<crate::events::KoEvent>,
) {
    round.frames_in_phase += 1;

    match round.phase {
        RoundPhase::Intro => {
            if round.frames_in_phase > 60 {
                round.phase = RoundPhase::Ready;
                round.frames_in_phase = 0;
                round_events.send(crate::events::RoundStateEvent { new_state: RoundPhase::Ready });
            }
        }
        RoundPhase::Ready => {
            if round.frames_in_phase > 60 {
                round.phase = RoundPhase::Fight;
                round.frames_in_phase = 0;
                round_events.send(crate::events::RoundStateEvent { new_state: RoundPhase::Fight });
            }
        }
        RoundPhase::Fight => {
            if !ko_events.is_empty() {
                round.phase = RoundPhase::KoFreeze;
                round.frames_in_phase = 0;
                round_events.send(crate::events::RoundStateEvent { new_state: RoundPhase::KoFreeze });
            }
        }
        RoundPhase::KoFreeze => {
            if round.frames_in_phase > 90 {
                round.phase = RoundPhase::Result;
                round.frames_in_phase = 0;
                round_events.send(crate::events::RoundStateEvent { new_state: RoundPhase::Result });
            }
        }
        RoundPhase::Result => {
            // later: rematch / next round / match end
        }
    }
}
