use bevy::prelude::*;
use bevy::ecs::entity::Entity;

#[derive(Event)]
pub struct HitEvent {
    pub attacker: Entity,
    pub victim: Entity,
    pub damage: i32,
    pub hitstun_frames: u16,
    // later: hit_type, launch_vector, etc.
}

#[derive(Event)]
pub struct KoEvent {
    pub winner: Entity,
    pub loser: Entity,
}

#[derive(Event)]
pub struct RoundStateEvent {
    pub new_state: crate::round::RoundPhase,
}
