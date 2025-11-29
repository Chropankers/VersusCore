use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
