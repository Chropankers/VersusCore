use bevy::prelude::*;

pub mod hitbox_viewer;
pub mod state_overlay;
pub mod frame_step;

pub struct VersusCoreDebugPlugin;

impl Plugin for VersusCoreDebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, hitbox_viewer::draw_hitboxes)
            .add_systems(Update, state_overlay::draw_state_overlay)
            .add_systems(Update, frame_step::frame_step_control);
    }
}
