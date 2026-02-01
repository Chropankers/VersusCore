use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use bevy::window::PrimaryWindow;
use serde::Deserialize;
use std::fs;

// --- Config structs ----------------------------------------------------------

#[derive(Deserialize, Debug, Clone)]
struct SceneConfig {
    stage: StageConfig,
    fighters: Vec<FighterConfig>,
}

#[derive(Deserialize, Debug, Clone)]
struct StageConfig {
    texture: String,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
}

#[derive(Deserialize, Debug, Clone)]
struct FighterConfig {
    name: String,
    player_id: u8,
    texture: String,
    pos_x: f32,
    pos_y: f32,  // interpreted as offset from floor
    scale: f32,
}

// --- Resources ---------------------------------------------------------------

#[derive(Resource, Debug, Clone)]
struct LoadedScene(SceneConfig);

#[derive(Resource)]
struct FloorY(pub f32);

#[derive(Resource)]
struct DemoMode {
    stage_only: bool,
}

impl Default for DemoMode {
    fn default() -> Self {
        Self {
            stage_only: false, // set true to test stage-only mode
        }
    }
}

// --- Components --------------------------------------------------------------

#[derive(Component)]
struct Fighter;

#[derive(Component)]
struct PlayerId(u8);

#[derive(Component)]
struct FighterName(String);

#[derive(Component)]
struct Stage;

#[derive(Component)]
struct FitToWindow;

// --- Entry point ------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets".to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "VersusCore Demo".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(DemoMode::default())
        // Tune this to match “top of the ground” visually once stage fits the window
        .insert_resource(FloorY(-80.0))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, stage_only_or_scene_loader)
        // Runs every frame until the stage image is loaded, then applies a fit scale
        .add_systems(Update, fit_stage_to_window)
        .run();
}

// --- Systems ----------------------------------------------------------------

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn stage_only_or_scene_loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mode: Res<DemoMode>,
    floor: Res<FloorY>,
) {
    if mode.stage_only {
        info!("Stage-only mode: spawning assets/stages/stage.png");
        spawn_stage_sprite(
            &mut commands,
            &asset_server,
            "stages/stage.png",
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
        );
        return;
    }

    info!("Scene mode: loading assets/demo/scene.toml and spawning stage + fighters");
    let cfg = load_scene_toml("assets/demo/scene.toml");
    commands.insert_resource(LoadedScene(cfg.clone()));

    // Stage
    spawn_stage_sprite(
        &mut commands,
        &asset_server,
        cfg.stage.texture.as_str(),
        Vec3::new(cfg.stage.pos_x, cfg.stage.pos_y, 0.0),
        cfg.stage.scale,
    );

    // Fighters (snap to floor; scale per-config)
    for f in cfg.fighters.iter() {
        let tex: Handle<Image> = asset_server.load(f.texture.as_str());

        let x = f.pos_x;
        let y = floor.0 + f.pos_y;

        commands.spawn((
            Fighter,
            PlayerId(f.player_id),
            FighterName(f.name.clone()),
            Sprite { image: tex, ..default() },
            Transform::from_xyz(x, y, 10.0).with_scale(Vec3::splat(f.scale)),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ));
    }
}

fn spawn_stage_sprite(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_path: &str,
    pos: Vec3,
    scale: f32,
) {
    let tex: Handle<Image> = asset_server.load(texture_path);

    // FitToWindow makes it auto-scale once the image is loaded.
    // If you want to keep TOML scale only, remove FitToWindow and the Update system.
    commands.spawn((
        Stage,
        FitToWindow,
        Sprite { image: tex, ..default() },
        Transform::from_translation(pos).with_scale(Vec3::splat(scale)),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

fn fit_stage_to_window(
    windows: Query<&Window, With<PrimaryWindow>>,
    images: Res<Assets<Image>>,
    mut stage_q: Query<(&Sprite, &mut Transform), (With<Stage>, With<FitToWindow>)>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let Ok((sprite, mut transform)) = stage_q.get_single_mut() else { return; };

    let Some(image) = images.get(&sprite.image) else {
        // Not loaded yet
        return;
    };

    // Image pixel dimensions
    let w = image.texture_descriptor.size.width as f32;
    let h = image.texture_descriptor.size.height as f32;
    if w <= 0.0 || h <= 0.0 {
        return;
    }

    // Fit-to-window (uniform scale) with a tiny margin
    let margin = 0.98;
    let sx = (window.width() / w) * margin;
    let sy = (window.height() / h) * margin;
    let s = sx.min(sy);

    transform.scale = Vec3::splat(s);

    // Center it (comment out if you want TOML pos_x/pos_y to control)
    transform.translation.x = 0.0;
    transform.translation.y = 0.0;
}

fn load_scene_toml(path: &str) -> SceneConfig {
    let s = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read scene config '{}': {}", path, e));

    toml::from_str(&s)
        .unwrap_or_else(|e| panic!("Failed to parse TOML scene config '{}': {}", path, e))
}
