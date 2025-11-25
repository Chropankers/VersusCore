use bevy::app::App;

use crate::systems::{
    input::input_system,
    collision_hitbox::collision_hitbox_system,
    state_machine::state_machine_system,
    movement::movement_system,
    time_freese::time_freeze_system,
    resource_meter::resource_meter_system,
    hit_resolution::hit_resolution_system,
};

pub fn configure_systems(app: &mut App) {
    use bevy::app::Update;

    app.add_systems(
        Update,
        (
            input_system,
            collision_hitbox_system,
            state_machine_system
        )
    )
}