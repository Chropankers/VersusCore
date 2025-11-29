use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct TimeFreeze {
    pub frames_remaining: i16,
}

#[derive(Resource, Reflect, Default)]
pub struct GlobalTimeScale {
    pub scale: f32, // 1.0 normal, 0.0 frozen, >1.0 speedup
}
