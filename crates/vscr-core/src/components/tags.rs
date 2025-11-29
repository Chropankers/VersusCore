use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct PlayerTag {
    pub id: u8, // 1, 2, etc.
}

#[derive(Component, Reflect)]
pub struct CharacterTag; // just marks "this is a controllable character"

#[derive(Component, Reflect)]
pub struct TeamTag {
    pub team: u8, // 0 = left, 1 = right, etc.
}
