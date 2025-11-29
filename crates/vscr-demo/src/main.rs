use bevy::prelude::*;
use vscr_core::VersusCorePlugin;
use vscr_debug::VersusCoreDebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "VersusCore Demo".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(VersusCorePlugin)
        .add_plugins(VersusCoreDebugPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // spawn camera, stage, placeholder characters…
}
