use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct ColliderAabb {
    pub half_extents: Vec2, // in local space
    pub offset: Vec2,       // from entity origin
    pub is_hitbox: bool,    // true = attack, false = hurtbox
}
