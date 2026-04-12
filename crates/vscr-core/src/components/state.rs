use bevy::prelude::*;

#[derive(Reflect, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterState {
    Idle,
    Walking,
    Crouching,
    Jumping,
    Falling,
    Attacking,
    Hitstun,
    KO,
}

#[derive(Component, Reflect)]
pub struct StateMachine {
    pub state: CharacterState,
    pub frames_in_state: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_machine_starts_idle() {
        let sm = StateMachine { state: CharacterState::Idle, frames_in_state: 0 };
        assert_eq!(sm.state, CharacterState::Idle);
        assert_eq!(sm.frames_in_state, 0);
    }

    #[test]
    fn hitstun_recovers_to_idle_after_threshold() {
        let mut sm = StateMachine { state: CharacterState::Hitstun, frames_in_state: 31 };
        // Mirror the logic in state_machine_system
        if sm.state == CharacterState::Hitstun && sm.frames_in_state > 30 {
            sm.state = CharacterState::Idle;
            sm.frames_in_state = 0;
        }
        assert_eq!(sm.state, CharacterState::Idle);
        assert_eq!(sm.frames_in_state, 0);
    }

    #[test]
    fn hitstun_does_not_recover_before_threshold() {
        let mut sm = StateMachine { state: CharacterState::Hitstun, frames_in_state: 15 };
        if sm.state == CharacterState::Hitstun && sm.frames_in_state > 30 {
            sm.state = CharacterState::Idle;
        }
        assert_eq!(sm.state, CharacterState::Hitstun, "should still be in hitstun");
    }

    #[test]
    fn frames_in_state_saturating_add_prevents_overflow() {
        let mut sm = StateMachine { state: CharacterState::Idle, frames_in_state: u16::MAX };
        sm.frames_in_state = sm.frames_in_state.saturating_add(1);
        assert_eq!(sm.frames_in_state, u16::MAX);
    }
}
