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
