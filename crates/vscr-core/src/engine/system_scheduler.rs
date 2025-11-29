use bevy::app::App;
use bevy::prelude::IntoSystemConfigs;

use crate::systems::{
    input_system,
    state_machine_system,
    movement_system,
    collision_hitbox_system,
    hit_resolution_system,
    time_freeze_system,
    resource_meter_system,
};

pub fn configure_systems(app: &mut App) {
    use bevy::app::Update;

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
        .chain(),
    );
}
