use bevy::asset::AssetPlugin;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;

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
    #[serde(default = "default_ground_y")]
    ground_y: f32,
}

#[derive(Deserialize, Debug, Clone)]
struct FighterConfig {
    name: String,
    player_id: u8,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    #[serde(default)]
    flip_x: bool,
    #[serde(default = "default_movement_speed")]
    movement_speed: f32,
    shuriken_texture: String,
    #[serde(default = "default_shuriken_speed")]
    shuriken_speed: f32,
    #[serde(default = "default_shuriken_lifetime")]
    shuriken_lifetime: f32,
    #[serde(default = "default_shuriken_scale")]
    shuriken_scale: f32,
    #[serde(default)]
    hitboxes: FighterHitboxesConfig,
    animations: FighterAnimationsConfig,
}

#[derive(Deserialize, Debug, Clone)]
struct FighterHitboxesConfig {
    #[serde(default = "default_hurtbox_width")]
    hurtbox_width: f32,
    #[serde(default = "default_hurtbox_height")]
    hurtbox_height: f32,
    #[serde(default = "default_hurtbox_center_y")]
    hurtbox_center_y: f32,
    #[serde(default = "default_projectile_hitbox_radius")]
    projectile_hitbox_radius: f32,
}

impl Default for FighterHitboxesConfig {
    fn default() -> Self {
        Self {
            hurtbox_width: default_hurtbox_width(),
            hurtbox_height: default_hurtbox_height(),
            hurtbox_center_y: default_hurtbox_center_y(),
            projectile_hitbox_radius: default_projectile_hitbox_radius(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct FighterAnimationsConfig {
    clips: HashMap<String, AnimationClipConfig>,
    state_map: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
struct AnimationClipConfig {
    texture: String,
    frame_width: u32,
    frame_height: u32,
    columns: u32,
    rows: u32,
    first: usize,
    last: usize,
    #[serde(default = "default_animation_fps")]
    fps: f32,
    #[serde(default = "default_clip_looping")]
    looping: bool,
}

fn default_ground_y() -> f32 {
    -80.0
}

fn default_animation_fps() -> f32 {
    8.0
}

fn default_clip_looping() -> bool {
    true
}

fn default_movement_speed() -> f32 {
    220.0
}

fn default_shuriken_speed() -> f32 {
    420.0
}

fn default_shuriken_lifetime() -> f32 {
    1.6
}

fn default_shuriken_scale() -> f32 {
    6.0
}

fn default_hurtbox_width() -> f32 {
    72.0
}

fn default_hurtbox_height() -> f32 {
    120.0
}

fn default_hurtbox_center_y() -> f32 {
    56.0
}

fn default_projectile_hitbox_radius() -> f32 {
    18.0
}

#[derive(Resource)]
struct DemoMode {
    stage_only: bool,
}

impl Default for DemoMode {
    fn default() -> Self {
        Self { stage_only: false }
    }
}

#[derive(Resource)]
struct TuningState {
    ground_y: f32,
    default_ground_y: f32,
    selected_player_id: u8,
    selected_field: TuneField,
}

#[derive(Resource, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    Debug,
    CharacterControl,
}

#[derive(Resource)]
struct ActiveInputMode {
    mode: InputMode,
}

impl Default for ActiveInputMode {
    fn default() -> Self {
        Self {
            mode: InputMode::CharacterControl,
        }
    }
}

#[derive(Component)]
struct Fighter;

#[derive(Component)]
struct FighterTuning {
    name: String,
    player_id: u8,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    flip_x: bool,
    movement_speed: f32,
}

#[derive(Component, Clone, Copy)]
struct FighterDefaults {
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    flip_x: bool,
}

#[derive(Component, Clone, Copy)]
struct FighterMotion {
    airborne_offset: f32,
    vertical_velocity: f32,
}

#[derive(Component, Clone, Copy)]
struct FighterHitboxes {
    hurtbox_width: f32,
    hurtbox_height: f32,
    hurtbox_center_y: f32,
    projectile_hitbox_radius: f32,
}

#[derive(Clone)]
struct LoadedClip {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    first: usize,
    last: usize,
    fps: f32,
    looping: bool,
}

#[derive(Component, Clone)]
struct FighterAnimationSet {
    clips: HashMap<String, LoadedClip>,
    state_map: HashMap<String, String>,
    shuriken_image: Handle<Image>,
    shuriken_speed: f32,
    shuriken_lifetime: f32,
    shuriken_scale: f32,
}

impl FighterAnimationSet {
    fn clip_for_state(&self, state_name: &str) -> Option<&LoadedClip> {
        let clip_name = self.state_map.get(state_name)?;
        self.clips.get(clip_name)
    }
}

#[derive(Component)]
struct FighterActionState {
    state: String,
    locked_time_left: f32,
}

#[derive(Component)]
struct AnimationPlayback {
    first: usize,
    last: usize,
    looping: bool,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Projectile {
    owner_player_id: u8,
    velocity: Vec2,
    lifetime: f32,
    hitbox_radius: f32,
}

#[derive(Component)]
struct Stage;

#[derive(Component)]
struct FitToWindow;

#[derive(Component)]
struct TuningHudText;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum TuneField {
    PosX,
    PosY,
    Scale,
    GroundY,
}

impl TuneField {
    fn label(self) -> &'static str {
        match self {
            Self::PosX => "pos_x",
            Self::PosY => "pos_y",
            Self::Scale => "scale",
            Self::GroundY => "ground_y",
        }
    }
}

#[derive(Component)]
struct TuneFieldButton(TuneField);

const STATE_IDLE: &str = "idle";
const STATE_WALK: &str = "walk";
const STATE_ATTACK: &str = "attack";
const STATE_THROW: &str = "throw";
const STATE_HURT: &str = "hurt";
const STATE_JUMP: &str = "jump";
const STATE_JUMP_UP: &str = "jump_up";
const STATE_JUMP_DOWN: &str = "jump_down";

#[derive(Clone, Copy)]
struct PlayerInputBinding {
    left: KeyCode,
    right: KeyCode,
    jump: KeyCode,
    attack: KeyCode,
    throw: KeyCode,
}

const JUMP_VELOCITY: f32 = 620.0;
const GRAVITY: f32 = 1800.0;

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
        .insert_resource(ActiveInputMode::default())
        .add_systems(Startup, (setup_camera, setup_tuning_ui))
        .add_systems(Startup, stage_only_or_scene_loader)
        .add_systems(Update, toggle_input_mode)
        .add_systems(Update, fit_stage_to_window)
        .add_systems(Update, tuning_button_clicks)
        .add_systems(Update, tuning_keyboard_input)
        .add_systems(Update, tuning_mouse_wheel_adjust)
        .add_systems(Update, update_projectiles)
        .add_systems(Update, control_fighters)
        .add_systems(Update, animate_fighter_sprites)
        .add_systems(Update, apply_tuned_values)
        .add_systems(Update, update_tuning_ui)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_tuning_ui(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(120.0),
        height: Val::Px(28.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(3.0)),
        ..default()
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                width: Val::Px(520.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.82)),
        ))
        .with_children(|parent| {
            parent.spawn((
                TuningHudText,
                Text::new("Loading scene..."),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|row| {
                    spawn_field_button(row, TuneField::PosX, "pos_x", button_node.clone());
                    spawn_field_button(row, TuneField::PosY, "pos_y", button_node.clone());
                    spawn_field_button(row, TuneField::Scale, "scale", button_node.clone());
                    spawn_field_button(row, TuneField::GroundY, "ground_y", button_node.clone());
                });
        });
}

fn spawn_field_button(parent: &mut ChildBuilder, field: TuneField, label: &str, node: Node) {
    parent
        .spawn((
            Button,
            TuneFieldButton(field),
            node,
            BackgroundColor(Color::srgb(0.2, 0.2, 0.27)),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn toggle_input_mode(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<ActiveInputMode>) {
    if !keys.just_pressed(KeyCode::F1) {
        return;
    }
    mode.mode = match mode.mode {
        InputMode::Debug => InputMode::CharacterControl,
        InputMode::CharacterControl => InputMode::Debug,
    };
}

fn stage_only_or_scene_loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mode: Res<DemoMode>,
) {
    if mode.stage_only {
        spawn_stage_sprite(
            &mut commands,
            &asset_server,
            "stages/stage.png",
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
        );
        return;
    }

    let cfg = load_scene_toml("assets/demo/scene.toml");
    commands.insert_resource(TuningState {
        ground_y: cfg.stage.ground_y,
        default_ground_y: cfg.stage.ground_y,
        selected_player_id: cfg.fighters.first().map(|f| f.player_id).unwrap_or(1),
        selected_field: TuneField::Scale,
    });

    spawn_stage_sprite(
        &mut commands,
        &asset_server,
        cfg.stage.texture.as_str(),
        Vec3::new(cfg.stage.pos_x, cfg.stage.pos_y, 0.0),
        cfg.stage.scale,
    );

    for f in &cfg.fighters {
        let anim_set = build_animation_set(&asset_server, &mut texture_atlas_layouts, f);
        let initial_clip = anim_set
            .clip_for_state(STATE_IDLE)
            .unwrap_or_else(|| panic!("fighter '{}' is missing '{}' state mapping", f.name, STATE_IDLE))
            .clone();

        let mut sprite = Sprite::from_atlas_image(
            initial_clip.image.clone(),
            TextureAtlas {
                layout: initial_clip.layout.clone(),
                index: initial_clip.first,
            },
        );
        sprite.anchor = Anchor::BottomCenter;
        sprite.flip_x = f.flip_x;

        commands.spawn((
            Fighter,
            FighterTuning {
                name: f.name.clone(),
                player_id: f.player_id,
                pos_x: f.pos_x,
                pos_y: f.pos_y,
                scale: f.scale,
                flip_x: f.flip_x,
                movement_speed: f.movement_speed,
            },
            FighterDefaults {
                pos_x: f.pos_x,
                pos_y: f.pos_y,
                scale: f.scale,
                flip_x: f.flip_x,
            },
            FighterMotion {
                airborne_offset: 0.0,
                vertical_velocity: 0.0,
            },
            FighterHitboxes {
                hurtbox_width: f.hitboxes.hurtbox_width,
                hurtbox_height: f.hitboxes.hurtbox_height,
                hurtbox_center_y: f.hitboxes.hurtbox_center_y,
                projectile_hitbox_radius: f.hitboxes.projectile_hitbox_radius,
            },
            anim_set,
            FighterActionState {
                state: STATE_IDLE.to_string(),
                locked_time_left: 0.0,
            },
            AnimationPlayback {
                first: initial_clip.first,
                last: initial_clip.last,
                looping: initial_clip.looping,
            },
            AnimationTimer(Timer::from_seconds(
                1.0 / initial_clip.fps.max(0.01),
                TimerMode::Repeating,
            )),
            sprite,
            Transform::from_xyz(f.pos_x, cfg.stage.ground_y + f.pos_y, 10.0)
                .with_scale(Vec3::splat(f.scale)),
            GlobalTransform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ));
    }
}

fn build_animation_set(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    fighter: &FighterConfig,
) -> FighterAnimationSet {
    let mut clips = HashMap::new();
    for (clip_name, clip_cfg) in &fighter.animations.clips {
        let clip = load_clip(
            asset_server,
            texture_atlas_layouts,
            &fighter.name,
            clip_name.as_str(),
            clip_cfg,
        );
        clips.insert(clip_name.clone(), clip);
    }
    for (state_name, clip_name) in &fighter.animations.state_map {
        assert!(
            clips.contains_key(clip_name),
            "fighter '{}' state '{}' references missing clip '{}'",
            fighter.name,
            state_name,
            clip_name
        );
    }
    assert!(
        fighter.animations.state_map.contains_key(STATE_IDLE),
        "fighter '{}' must define '{}' in animations.state_map",
        fighter.name,
        STATE_IDLE
    );
    assert!(
        fighter.animations.state_map.contains_key(STATE_WALK),
        "fighter '{}' must define '{}' in animations.state_map",
        fighter.name,
        STATE_WALK
    );
    assert!(
        fighter.animations.state_map.contains_key(STATE_ATTACK),
        "fighter '{}' must define '{}' in animations.state_map",
        fighter.name,
        STATE_ATTACK
    );
    assert!(
        fighter.animations.state_map.contains_key(STATE_THROW),
        "fighter '{}' must define '{}' in animations.state_map",
        fighter.name,
        STATE_THROW
    );

    FighterAnimationSet {
        clips,
        state_map: fighter.animations.state_map.clone(),
        shuriken_image: asset_server.load(fighter.shuriken_texture.as_str()),
        shuriken_speed: fighter.shuriken_speed,
        shuriken_lifetime: fighter.shuriken_lifetime,
        shuriken_scale: fighter.shuriken_scale,
    }
}

fn load_clip(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    fighter_name: &str,
    clip_name: &str,
    cfg: &AnimationClipConfig,
) -> LoadedClip {
    assert!(cfg.frame_width > 0 && cfg.frame_height > 0, "{} {} clip has invalid frame size", fighter_name, clip_name);
    assert!(cfg.columns > 0 && cfg.rows > 0, "{} {} clip has invalid grid", fighter_name, clip_name);
    let frame_count = (cfg.columns * cfg.rows) as usize;
    assert!(
        cfg.first < frame_count && cfg.last < frame_count && cfg.last >= cfg.first,
        "{} {} clip range {}..={} invalid for {} frames",
        fighter_name,
        clip_name,
        cfg.first,
        cfg.last,
        frame_count
    );
    assert!(cfg.fps > 0.0, "{} {} clip fps must be positive", fighter_name, clip_name);

    let image = asset_server.load(cfg.texture.as_str());
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(cfg.frame_width, cfg.frame_height),
        cfg.columns,
        cfg.rows,
        None,
        None,
    ));

    LoadedClip {
        image,
        layout,
        first: cfg.first,
        last: cfg.last,
        fps: cfg.fps,
        looping: cfg.looping,
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

fn binding_for_player(player_id: u8) -> Option<PlayerInputBinding> {
    match player_id {
        1 => Some(PlayerInputBinding {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            jump: KeyCode::KeyW,
            attack: KeyCode::KeyJ,
            throw: KeyCode::KeyK,
        }),
        2 => Some(PlayerInputBinding {
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
            jump: KeyCode::ArrowUp,
            attack: KeyCode::KeyN,
            throw: KeyCode::KeyM,
        }),
        _ => None,
    }
}

fn set_action_state(
    action: &mut FighterActionState,
    state: &str,
    lock_duration: f32,
) {
    action.state = state.to_string();
    action.locked_time_left = lock_duration.max(0.0);
}

fn state_clip_duration(anims: &FighterAnimationSet, state: &str) -> f32 {
    anims.clip_for_state(state).map(clip_duration).unwrap_or(0.15)
}

fn control_fighters(
    mut commands: Commands,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    tuning: Option<Res<TuningState>>,
    mode: Res<ActiveInputMode>,
    mut fighters: Query<(
        &mut FighterTuning,
        &mut FighterMotion,
        &FighterHitboxes,
        &mut FighterActionState,
        &FighterAnimationSet,
        &mut AnimationPlayback,
        &mut AnimationTimer,
        &mut Sprite,
    )>,
) {
    if mode.mode != InputMode::CharacterControl {
        return;
    }
    let ground_y = tuning.as_ref().map(|t| t.ground_y).unwrap_or(default_ground_y());
    for (mut fighter, mut motion, hitboxes, mut action, anims, mut playback, mut timer, mut sprite) in &mut fighters {
        let Some(binding) = binding_for_player(fighter.player_id) else {
            continue;
        };

        let dt = time.delta_secs();
        if keys.just_pressed(binding.jump) && motion.airborne_offset <= 0.0 {
            motion.vertical_velocity = JUMP_VELOCITY;
        }
        if motion.airborne_offset > 0.0 || motion.vertical_velocity > 0.0 {
            motion.vertical_velocity -= GRAVITY * dt;
            motion.airborne_offset = (motion.airborne_offset + motion.vertical_velocity * dt).max(0.0);
            if motion.airborne_offset <= 0.0 {
                motion.vertical_velocity = 0.0;
            }
        }

        if action.locked_time_left > 0.0 {
            action.locked_time_left = (action.locked_time_left - dt).max(0.0);
            if action.locked_time_left <= 0.0 {
                action.state = STATE_IDLE.to_string();
            }
        } else if keys.just_pressed(binding.throw) {
            set_action_state(&mut action, STATE_THROW, state_clip_duration(anims, STATE_THROW));
            spawn_shuriken(
                &mut commands,
                &fighter,
                ground_y,
                hitboxes.projectile_hitbox_radius,
                anims,
            );
        } else if keys.just_pressed(binding.attack) {
            set_action_state(&mut action, STATE_ATTACK, state_clip_duration(anims, STATE_ATTACK));
        } else {
            let mut axis = 0.0f32;
            if keys.pressed(binding.left) {
                axis -= 1.0;
            }
            if keys.pressed(binding.right) {
                axis += 1.0;
            }

            if axis != 0.0 {
                fighter.pos_x += axis * fighter.movement_speed * dt;
                fighter.flip_x = axis < 0.0;
                action.state = STATE_WALK.to_string();
            } else {
                action.state = STATE_IDLE.to_string();
            }
            if motion.airborne_offset > 0.0 {
                let airborne_state = if motion.vertical_velocity >= 0.0 {
                    if anims.clip_for_state(STATE_JUMP_UP).is_some() {
                        STATE_JUMP_UP
                    } else {
                        STATE_JUMP
                    }
                } else if anims.clip_for_state(STATE_JUMP_DOWN).is_some() {
                    STATE_JUMP_DOWN
                } else {
                    STATE_JUMP
                };
                if anims.clip_for_state(airborne_state).is_some() {
                    action.state = airborne_state.to_string();
                }
            }
        }

        if let Some(clip) = anims.clip_for_state(action.state.as_str()) {
            apply_clip(clip, &mut playback, &mut timer, &mut sprite);
        }
    }
}

fn spawn_shuriken(
    commands: &mut Commands,
    fighter: &FighterTuning,
    ground_y: f32,
    hitbox_radius: f32,
    anims: &FighterAnimationSet,
) {
    let dir = if fighter.flip_x { -1.0 } else { 1.0 };
    commands.spawn((
        Projectile {
            owner_player_id: fighter.player_id,
            velocity: Vec2::new(dir * anims.shuriken_speed, 0.0),
            lifetime: anims.shuriken_lifetime,
            hitbox_radius,
        },
        Sprite::from_image(anims.shuriken_image.clone()),
        Transform::from_xyz(
            fighter.pos_x + dir * 28.0,
            ground_y + fighter.pos_y + 22.0,
            30.0,
        )
        .with_scale(Vec3::splat(anims.shuriken_scale)),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

fn clip_duration(clip: &LoadedClip) -> f32 {
    let frames = (clip.last - clip.first + 1) as f32;
    (frames / clip.fps.max(0.01)).max(0.05)
}

fn apply_clip(
    clip: &LoadedClip,
    playback: &mut AnimationPlayback,
    timer: &mut AnimationTimer,
    sprite: &mut Sprite,
) {
    timer.set_duration(std::time::Duration::from_secs_f32(
        1.0 / clip.fps.max(0.01),
    ));
    let changed = playback.first != clip.first || playback.last != clip.last || playback.looping != clip.looping;
    playback.first = clip.first;
    playback.last = clip.last;
    playback.looping = clip.looping;

    let image_changed = sprite.image != clip.image;
    let layout_changed = match &sprite.texture_atlas {
        Some(atlas) => atlas.layout != clip.layout,
        None => true,
    };

    if image_changed || layout_changed || changed {
        sprite.image = clip.image.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: clip.layout.clone(),
            index: clip.first,
        });
        timer.reset();
    }
}

fn animate_fighter_sprites(
    time: Res<Time>,
    mut fighters: Query<(&AnimationPlayback, &mut AnimationTimer, &mut Sprite), With<Fighter>>,
) {
    for (playback, mut timer, mut sprite) in &mut fighters {
        timer.tick(time.delta());
        if !timer.just_finished() {
            continue;
        }
        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };
        if atlas.index >= playback.last {
            if playback.looping {
                atlas.index = playback.first;
            } else {
                atlas.index = playback.last;
            }
        } else {
            atlas.index += 1;
        }
    }
}

fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    tuning: Option<Res<TuningState>>,
    mut projectiles: Query<(Entity, &mut Transform, &mut Projectile)>,
    mut fighters: Query<(
        &FighterTuning,
        &FighterMotion,
        &FighterHitboxes,
        &mut FighterActionState,
        &FighterAnimationSet,
    ), With<Fighter>>,
) {
    let ground_y = tuning.as_ref().map(|t| t.ground_y).unwrap_or(default_ground_y());
    let mut despawn_entities = HashSet::new();

    for (entity, mut transform, mut proj) in &mut projectiles {
        let dt = time.delta_secs();
        transform.translation.x += proj.velocity.x * dt;
        transform.translation.y += proj.velocity.y * dt;
        proj.lifetime -= dt;
        if proj.lifetime <= 0.0 || transform.translation.x.abs() > 2200.0 {
            despawn_entities.insert(entity);
            continue;
        }

        for (fighter, motion, hitboxes, mut action, anims) in &mut fighters {
            if fighter.player_id == proj.owner_player_id {
                continue;
            }
            let fighter_center = Vec2::new(
                fighter.pos_x,
                ground_y + fighter.pos_y + motion.airborne_offset + hitboxes.hurtbox_center_y,
            );
            let projectile_pos = transform.translation.truncate();
            let dx = (projectile_pos.x - fighter_center.x).abs();
            let dy = (projectile_pos.y - fighter_center.y).abs();
            if dx <= hitboxes.hurtbox_width * 0.5 + proj.hitbox_radius
                && dy <= hitboxes.hurtbox_height * 0.5 + proj.hitbox_radius
            {
                set_action_state(
                    &mut action,
                    STATE_HURT,
                    state_clip_duration(anims, STATE_HURT),
                );
                despawn_entities.insert(entity);
                break;
            }
        }
    }

    for entity in despawn_entities {
        commands.entity(entity).despawn();
    }
}

fn apply_tuned_values(
    tuning: Option<Res<TuningState>>,
    mut fighters: Query<(&FighterTuning, &FighterMotion, &mut Transform, &mut Sprite), With<Fighter>>,
) {
    let Some(tuning) = tuning else { return };
    for (fighter, motion, mut transform, mut sprite) in &mut fighters {
        transform.translation.x = fighter.pos_x;
        transform.translation.y = tuning.ground_y + fighter.pos_y + motion.airborne_offset;
        transform.scale = Vec3::splat(fighter.scale);
        sprite.flip_x = fighter.flip_x;
    }
}

fn tuning_button_clicks(
    mut buttons: Query<(&Interaction, &TuneFieldButton), (Changed<Interaction>, With<Button>)>,
    tuning: Option<ResMut<TuningState>>,
    mode: Res<ActiveInputMode>,
) {
    if mode.mode != InputMode::Debug {
        return;
    }
    let Some(mut tuning) = tuning else { return };
    for (interaction, button) in &mut buttons {
        if *interaction == Interaction::Pressed {
            tuning.selected_field = button.0;
        }
    }
}

fn tuning_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    tuning: Option<ResMut<TuningState>>,
    mode: Res<ActiveInputMode>,
    mut fighters: Query<(&mut FighterTuning, &FighterDefaults, &mut FighterMotion)>,
) {
    if mode.mode != InputMode::Debug {
        return;
    }
    let Some(mut tuning) = tuning else { return };

    let mut ids: Vec<u8> = fighters.iter().map(|(f, _, _)| f.player_id).collect();
    ids.sort_unstable();
    ids.dedup();
    if ids.is_empty() {
        return;
    }
    if !ids.contains(&tuning.selected_player_id) {
        tuning.selected_player_id = ids[0];
    }

    if keys.just_pressed(KeyCode::Tab) {
        let cur = ids
            .iter()
            .position(|id| *id == tuning.selected_player_id)
            .unwrap_or(0);
        tuning.selected_player_id = ids[(cur + 1) % ids.len()];
    }

    if keys.just_pressed(KeyCode::KeyR) {
        if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            tuning.ground_y = tuning.default_ground_y;
            for (mut fighter, defaults, mut motion) in &mut fighters {
                fighter.pos_x = defaults.pos_x;
                fighter.pos_y = defaults.pos_y;
                fighter.scale = defaults.scale;
                fighter.flip_x = defaults.flip_x;
                motion.airborne_offset = 0.0;
                motion.vertical_velocity = 0.0;
            }
        } else {
            for (mut fighter, defaults, mut motion) in &mut fighters {
                if fighter.player_id != tuning.selected_player_id {
                    continue;
                }
                fighter.pos_x = defaults.pos_x;
                fighter.pos_y = defaults.pos_y;
                fighter.scale = defaults.scale;
                fighter.flip_x = defaults.flip_x;
                motion.airborne_offset = 0.0;
                motion.vertical_velocity = 0.0;
                break;
            }
        }
    }

    if keys.just_pressed(KeyCode::Digit1) {
        tuning.selected_field = TuneField::PosX;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        tuning.selected_field = TuneField::PosY;
    }
    if keys.just_pressed(KeyCode::Digit3) {
        tuning.selected_field = TuneField::Scale;
    }
    if keys.just_pressed(KeyCode::Digit4) {
        tuning.selected_field = TuneField::GroundY;
    }

    let positive = keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::ArrowUp);
    let negative = keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::ArrowDown);
    if positive == negative {
        return;
    }

    let fine = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let delta = step_for_field(tuning.selected_field, fine) * if positive { 1.0 } else { -1.0 };
    apply_delta(&mut tuning, &mut fighters, delta);
}

fn tuning_mouse_wheel_adjust(
    mut wheel_events: EventReader<MouseWheel>,
    tuning: Option<ResMut<TuningState>>,
    mode: Res<ActiveInputMode>,
    mut fighters: Query<(&mut FighterTuning, &FighterDefaults, &mut FighterMotion)>,
) {
    if mode.mode != InputMode::Debug {
        return;
    }
    let Some(mut tuning) = tuning else { return };
    let mut wheel_y = 0.0;
    for ev in wheel_events.read() {
        wheel_y += ev.y;
    }
    if wheel_y.abs() <= f32::EPSILON {
        return;
    }
    let delta = step_for_field(tuning.selected_field, true) * wheel_y;
    apply_delta(&mut tuning, &mut fighters, delta);
}

fn step_for_field(field: TuneField, fine: bool) -> f32 {
    match field {
        TuneField::PosX | TuneField::PosY | TuneField::GroundY => {
            if fine { 1.0 } else { 8.0 }
        }
        TuneField::Scale => {
            if fine { 0.05 } else { 0.25 }
        }
    }
}

fn apply_delta(
    tuning: &mut TuningState,
    fighters: &mut Query<(&mut FighterTuning, &FighterDefaults, &mut FighterMotion)>,
    delta: f32,
) {
    if tuning.selected_field == TuneField::GroundY {
        tuning.ground_y += delta;
        return;
    }
    for (mut fighter, _, _) in fighters.iter_mut() {
        if fighter.player_id != tuning.selected_player_id {
            continue;
        }
        match tuning.selected_field {
            TuneField::PosX => fighter.pos_x += delta,
            TuneField::PosY => fighter.pos_y += delta,
            TuneField::Scale => fighter.scale = (fighter.scale + delta).max(0.1),
            TuneField::GroundY => {}
        }
        break;
    }
}

fn update_tuning_ui(
    tuning: Option<Res<TuningState>>,
    mode: Res<ActiveInputMode>,
    fighters: Query<(&FighterTuning, &FighterActionState)>,
    mut hud_text: Query<&mut Text, With<TuningHudText>>,
    mut buttons: Query<(&TuneFieldButton, &mut BackgroundColor), With<Button>>,
) {
    let Some(tuning) = tuning else { return };
    let Ok(mut text) = hud_text.get_single_mut() else { return };

    for (button, mut bg) in &mut buttons {
        *bg = if button.0 == tuning.selected_field {
            BackgroundColor(Color::srgb(0.3, 0.55, 0.35))
        } else {
            BackgroundColor(Color::srgb(0.2, 0.2, 0.27))
        };
    }

    let selected = fighters
        .iter()
        .find(|(f, _)| f.player_id == tuning.selected_player_id);
    let Some((fighter, action)) = selected else {
        text.0 = "No fighter selected".to_string();
        return;
    };
    let mode_label = match mode.mode {
        InputMode::Debug => "DEBUG",
        InputMode::CharacterControl => "CONTROL",
    };

    text.0 = format!(
        "Data-Driven Tooling [{}] (F1 toggle)\nDebug: Tab cycle fighter | click field + arrows/wheel to tune | Shift fine\nGameplay: P1 A/D/W/J/K | P2 <-/->/Up/N/M\nSelected: {} (P{}) state={} field={}\npos_x={:.1} pos_y={:.1} scale={:.2} ground_y={:.1}",
        mode_label,
        fighter.name,
        fighter.player_id,
        action.state.as_str(),
        tuning.selected_field.label(),
        fighter.pos_x,
        fighter.pos_y,
        fighter.scale,
        tuning.ground_y
    );
}

fn fit_stage_to_window(
    windows: Query<&Window, With<PrimaryWindow>>,
    images: Res<Assets<Image>>,
    mut stage_q: Query<(&Sprite, &mut Transform), (With<Stage>, With<FitToWindow>)>,
) {
    let Ok(window) = windows.get_single() else { return };
    let Ok((sprite, mut transform)) = stage_q.get_single_mut() else { return };
    let Some(image) = images.get(&sprite.image) else {
        return;
    };
    let w = image.texture_descriptor.size.width as f32;
    let h = image.texture_descriptor.size.height as f32;
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let margin = 0.98;
    let s = ((window.width() / w) * margin).min((window.height() / h) * margin);
    transform.scale = Vec3::splat(s);
    transform.translation.x = 0.0;
    transform.translation.y = 0.0;
}

fn load_scene_toml(path: &str) -> SceneConfig {
    let s = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read scene config '{}': {}", path, e));
    toml::from_str(&s)
        .unwrap_or_else(|e| panic!("Failed to parse TOML scene config '{}': {}", path, e))
}
