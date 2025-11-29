use bevy::prelude::*;

pub mod engine;
pub mod components;
pub mod systems;
pub mod events;
pub mod round;
pub mod dsl_runtime;
pub mod output_bridge;

pub struct VersusCorePlugin;

impl Plugin for VersusCorePlugin {
    fn build(&self, app: &mut App) {
        engine::register_components(app);
        engine::configure_systems(app);
        round::register_round_flow(app);

        app
            .add_event::<events::HitEvent>()
            .add_event::<events::KoEvent>()
            .add_event::<events::RoundStateEvent>();
    }
}