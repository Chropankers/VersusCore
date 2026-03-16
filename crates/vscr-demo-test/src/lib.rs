#![cfg(test)]

// These tests validate data-driven scene loading and referenced asset availability.
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Deserialize, Debug)]
struct SceneConfig {
    stage: StageConfig,
    fighters: Vec<FighterConfig>,
}

#[derive(Deserialize, Debug)]
struct StageConfig {
    texture: String,
    ground_y: f32,
}

#[derive(Deserialize, Debug)]
struct FighterConfig {
    name: String,
    player_id: u8,
    scale: f32,
    movement_speed: Option<f32>,
    shuriken_texture: String,
    shuriken_speed: Option<f32>,
    shuriken_lifetime: Option<f32>,
    shuriken_scale: Option<f32>,
    hitboxes: Option<FighterHitboxesConfig>,
    flip_x: Option<bool>,
    animations: FighterAnimationsConfig,
}

#[derive(Deserialize, Debug)]
struct FighterHitboxesConfig {
    hurtbox_width: Option<f32>,
    hurtbox_height: Option<f32>,
    hurtbox_center_y: Option<f32>,
    projectile_hitbox_radius: Option<f32>,
}

#[derive(Deserialize, Debug)]
struct FighterAnimationsConfig {
    clips: HashMap<String, AnimationClipConfig>,
    state_map: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct AnimationClipConfig {
    texture: String,
    frame_width: u32,
    frame_height: u32,
    columns: u32,
    rows: u32,
    first: usize,
    last: usize,
    fps: Option<f32>,
}

fn scene_path() -> std::path::PathBuf {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest.parent().and_then(|p| p.parent()).expect("workspace root");
    workspace_root.join("assets").join("demo").join("scene.toml")
}

fn read_scene() -> SceneConfig {
    let path = scene_path();
    let s = fs::read_to_string(&path).expect("scene.toml should be readable");
    toml::from_str(&s).expect("scene.toml should parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_scene_toml() {
        let cfg = read_scene();

        assert!(!cfg.stage.texture.is_empty());
        assert!(cfg.stage.ground_y < 0.0, "ground_y should place the floor below center");
        assert!(cfg.fighters.len() >= 2, "expect at least two fighters");

        let ids: Vec<u8> = cfg.fighters.iter().map(|f| f.player_id).collect();
        let unique = ids.iter().cloned().collect::<HashSet<_>>();
        assert_eq!(unique.len(), ids.len(), "player_id values should be unique");

        for fighter in &cfg.fighters {
            assert!(!fighter.name.is_empty(), "fighter name should be present");
            assert!(fighter.scale > 0.0, "fighter scale must be positive");
            assert!(fighter.movement_speed.unwrap_or(1.0) > 0.0, "movement_speed should be positive");
            assert!(!fighter.shuriken_texture.is_empty(), "shuriken texture should be set");
            assert!(fighter.shuriken_speed.unwrap_or(1.0) > 0.0, "shuriken speed should be positive");
            assert!(fighter.shuriken_lifetime.unwrap_or(1.0) > 0.0, "shuriken lifetime should be positive");
            assert!(fighter.shuriken_scale.unwrap_or(1.0) > 0.0, "shuriken scale should be positive");
            let _ = fighter.flip_x.unwrap_or(false);

            if let Some(hitboxes) = &fighter.hitboxes {
                assert!(hitboxes.hurtbox_width.unwrap_or(1.0) > 0.0, "hurtbox_width must be positive");
                assert!(hitboxes.hurtbox_height.unwrap_or(1.0) > 0.0, "hurtbox_height must be positive");
                assert!(hitboxes.hurtbox_center_y.unwrap_or(0.0) >= 0.0, "hurtbox_center_y must be non-negative");
                assert!(hitboxes.projectile_hitbox_radius.unwrap_or(1.0) > 0.0, "projectile hitbox radius must be positive");
            }

            let required_states = ["idle", "walk", "attack", "throw", "jump", "hurt"];
            for state in required_states {
                let clip_name = fighter
                    .animations
                    .state_map
                    .get(state)
                    .unwrap_or_else(|| panic!("fighter '{}' missing state_map entry '{}'", fighter.name, state));
                assert!(
                    fighter.animations.clips.contains_key(clip_name),
                    "fighter '{}' state '{}' references missing clip '{}'",
                    fighter.name,
                    state,
                    clip_name
                );
            }

            for (clip_name, clip) in &fighter.animations.clips {
                let frame_count = (clip.columns * clip.rows) as usize;
                assert!(clip.frame_width > 0 && clip.frame_height > 0, "{}: clip '{}' frame size must be positive", fighter.name, clip_name);
                assert!(frame_count > 0, "{}: clip '{}' frame count should be non-zero", fighter.name, clip_name);
                assert!(clip.first < frame_count && clip.last < frame_count && clip.last >= clip.first, "{}: clip '{}' frame range invalid", fighter.name, clip_name);
                assert!(clip.fps.unwrap_or(8.0) > 0.0, "{}: clip '{}' fps should be positive", fighter.name, clip_name);
                assert!(!clip.texture.is_empty(), "{}: clip '{}' texture should be set", fighter.name, clip_name);
            }
        }
    }

    #[test]
    fn assets_exist() {
        let cfg = read_scene();
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest.parent().and_then(|p| p.parent()).expect("workspace root");

        let mut files = Vec::new();
        files.push(cfg.stage.texture);
        for fighter in cfg.fighters {
            files.push(fighter.shuriken_texture);
            for clip in fighter.animations.clips.values() {
                files.push(clip.texture.clone());
            }
        }

        for file in files {
            let p = workspace_root.join("assets").join(file);
            assert!(p.exists(), "{} should exist", p.display());
            let md = fs::metadata(&p).expect("metadata");
            assert!(md.len() > 0, "{} should be non-empty", p.display());
        }
    }
}
