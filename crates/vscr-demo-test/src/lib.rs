// These tests validate data-driven scene loading and asset availability,
// which are the primary responsibilities of the engine at this phase.
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct SceneConfig {
    stage: StageConfig,
    fighters: Vec<FighterConfig>,
}

#[derive(Deserialize, Debug)]
struct StageConfig {
    texture: String,
    pos_x: f32,
    pos_y: f32,
    scale: f32,
}

#[derive(Deserialize, Debug)]
struct FighterConfig {
    name: String,
    player_id: u8,
    texture: String,
    pos_x: f32,
    pos_y: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parse_scene_toml() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest.parent().and_then(|p| p.parent()).expect("workspace root");
        let path = workspace_root.join("assets").join("demo").join("scene.toml");
        let s = fs::read_to_string(&path).expect("scene.toml should be readable");
        let cfg: SceneConfig = toml::from_str(&s).expect("scene.toml should parse");
        // Basic assertions
        assert!(!cfg.stage.texture.is_empty());
        assert!(cfg.fighters.len() >= 2, "expect at least two fighters");
        // check fighter player ids unique
        let ids: Vec<u8> = cfg.fighters.iter().map(|f| f.player_id).collect();
        let unique = ids.iter().cloned().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), ids.len(), "player_id values should be unique");
    }

    #[test]
    fn assets_exist() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest.parent().and_then(|p| p.parent()).expect("workspace root");
        let files = ["stage.png", "p1.png", "p2.png"];
        for f in files.iter() {
            let p = workspace_root.join("assets").join("demo").join(f);
            assert!(p.exists(), "{} should exist", p.display());
            let md = fs::metadata(&p).expect("metadata");
            assert!(md.len() > 0, "{} should be non-empty", p.display());
        }
    }
}
