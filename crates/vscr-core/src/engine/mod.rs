use bevy::prelude::*;
use bevy::prelude::IntoSystemConfigs;

pub mod entity_registry;   // optional if you split later
pub mod system_scheduler;  // optional if you split later

// ---- your existing functions here ----

pub fn register_components(app: &mut App) {
    use crate::components::{
        tags,
        kinematics,
        collider,
        health,
        input,
        state,
        time,
        resources,
    };
    use crate::round::RoundState;

    app
        .register_type::<tags::PlayerTag>()
        .register_type::<tags::CharacterTag>()
        .register_type::<kinematics::Velocity>()
        .register_type::<collider::ColliderAabb>()
        .register_type::<health::Health>()
        .register_type::<state::CharacterState>()
        .register_type::<time::TimeFreeze>()
        .register_type::<resources::Meters>()
        .insert_resource(RoundState::default());
}

pub fn configure_systems(app: &mut App) {
    use bevy::app::Update;
    use crate::systems::*;

    app.add_systems(
        Update,
        (
            input_system::apply_player_input,
            state_machine_system::update_character_state,
            movement_system::apply_movement,
            collision_hitbox_system::detect_hits,
            hit_resolution_system::resolve_hits,
            time_freeze_system::update_time_freeze,
            resource_meter_system::update_meters,
        )
        .chain(), // strict ordering
    );
}
