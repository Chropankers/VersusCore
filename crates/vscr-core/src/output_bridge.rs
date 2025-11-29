use bevy::prelude::*;
use crate::components;
use crate::events::HitEvent;

#[derive(Resource, Default)]
pub struct PlayerInputState {
    // filled by vscr-demo from keyboard/gamepad
    pub p1_buttons: components::input::Buttons,
    pub p2_buttons: components::input::Buttons,
}

pub fn log_hits(mut reader: EventReader<HitEvent>) {
    for hit in reader.read() {
        println!("Hit: {:?} -> {:?}, dmg {}", hit.attacker, hit.victim, hit.damage);
    }
}
